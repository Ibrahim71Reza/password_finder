use clap::Parser;
use colored::Colorize;
use flate2::read::GzDecoder;
use ignore::WalkBuilder;
use rayon::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

#[derive(Parser, Debug)]
#[command(name = "pwfind", version = "1.0.0", about = "World's fastest password and secret finder", long_about = None)]
struct Cli {
    #[arg(short, long)]
    path: String,

    #[arg(short = 'w', long)]
    word: Option<String>,

    #[arg(long)]
    preset: Option<String>,

    #[arg(short, long, default_value_t = false)]
    regex: bool,

    /// EXTRACT MODE: Print ONLY the exact matched secret/password, not the whole line
    #[arg(short = 'x', long, default_value_t = false)]
    extract: bool,

    #[arg(short, long, value_delimiter = ',', default_value = "txt,json,md,csv,gz")]
    ext: Vec<String>,

    #[arg(short, long)]
    output: Option<String>,
}

struct MatchResult {
    file_path: String,
    line_num: usize,
    content: String,
}

fn search_file(path: &PathBuf, search_word: &str, compiled_regex: Option<&Regex>, extract: bool, tx: mpsc::Sender<MatchResult>) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return,
    };

    let is_gzip = path.extension().and_then(|e| e.to_str()) == Some("gz");

    let reader: Box<dyn Read> = if is_gzip {
        Box::new(GzDecoder::new(file))
    } else {
        Box::new(file)
    };

    let buf_reader = BufReader::new(reader);

    for (line_number, line_result) in buf_reader.lines().enumerate() {
        if let Ok(line) = line_result {
            // ---------------------------------------------------------
            // NEW: Extraction Logic!
            // ---------------------------------------------------------
            if let Some(re) = compiled_regex {
                if extract {
                    // Extract exact matches only!
                    for mat in re.find_iter(&line) {
                        let _ = tx.send(MatchResult {
                            file_path: path.display().to_string(),
                            line_num: line_number + 1,
                            content: mat.as_str().to_string(),
                        });
                    }
                } else if re.is_match(&line) {
                    let _ = tx.send(MatchResult {
                        file_path: path.display().to_string(),
                        line_num: line_number + 1,
                        content: line.clone(),
                    });
                }
            } else {
                // Exact Word Match (Non-Regex)
                if line.contains(search_word) {
                    let content = if extract { search_word.to_string() } else { line.clone() };
                    let _ = tx.send(MatchResult {
                        file_path: path.display().to_string(),
                        line_num: line_number + 1,
                        content,
                    });
                }
            }
        }
    }
}

fn main() {
    let args = Cli::parse();

    println!("{}", "=========================================".blue().bold());
    println!("{} {}", "🚀 Starting".green().bold(), "pwfind (The Ultimate Wordlist Finder)".magenta().bold());
    println!("{} {}", "📂 Target Path:".cyan(), args.path.yellow());

    let (search_pattern, is_regex_mode) = if let Some(p) = &args.preset {
        match p.as_str() {
            "jwt" => ("eyJ[A-Za-z0-9-_=]+\\.[A-Za-z0-9-_=]+\\.?[A-Za-z0-9-_.+/=]*".to_string(), true),
            "aws" => ("AKIA[0-9A-Z]{16}".to_string(), true),
            "email" => ("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}".to_string(), true),
            "ip" => ("\\b\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\b".to_string(), true),
            _ => {
                println!("{} Unknown preset! Available: jwt, aws, email, ip", "[-]".red().bold());
                return;
            }
        }
    } else if let Some(w) = &args.word {
        (w.clone(), args.regex)
    } else {
        println!("{} You must provide either --word <WORD> or --preset <PRESET>", "[-]".red().bold());
        return;
    };

    println!("{} {}", "🔍 Searching for:".cyan(), search_pattern.red().bold());
    
    let mode = if is_regex_mode {
        if args.preset.is_some() { "Regex Search (Smart Preset)" } else { "Regex Search" }
    } else { 
        "Exact Word Match" 
    };
    
    println!("{} {}", "⚙️  Mode:".cyan(), mode.yellow());
    if args.extract {
        println!("{} {}", "✂️  Extract Mode:".cyan(), "ON (Printing only exact matches)".yellow());
    }
    
    if let Some(ref out) = args.output {
        println!("{} {}", "💾 Saving to:".cyan(), out.yellow());
    }
    println!("{}", "=========================================".blue().bold());

    let regex_pattern = if is_regex_mode {
        match Regex::new(&search_pattern) {
            Ok(re) => Some(re),
            Err(e) => {
                println!("{} Invalid Regex format! Error: {}", "[-]".red().bold(), e);
                return;
            }
        }
    } else {
        None
    };

    let mut target_files: Vec<PathBuf> = Vec::new();
    println!("{}", "[*] Crawling directories to find target files...".blue());

    let walker = WalkBuilder::new(&args.path).hidden(false).build();

    for result in walker {
        if let Ok(entry) = result {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext_str = extension.to_string_lossy().to_string();
                    if args.ext.contains(&ext_str) {
                        target_files.push(path.to_path_buf());
                    }
                }
            }
        }
    }

    if target_files.is_empty() {
        println!("{}", "[-] No files found matching those extensions! Try a different path.".red().bold());
        return;
    }

    println!("{} Found {} files to scan. Starting multi-threaded search...\n", "[+]".green().bold(), target_files.len().to_string().yellow().bold());

    let (tx, rx) = mpsc::channel::<MatchResult>();
    let out_file_path = args.output.clone();
    
    let receiver_thread = thread::spawn(move || {
        let mut out_file = match out_file_path {
            Some(path) => match File::create(&path) {
                Ok(f) => Some(f),
                Err(e) => {
                    println!("{} Failed to create output file: {}", "[-]".red().bold(), e);
                    None
                }
            },
            None => None,
        };

        let mut match_count = 0;

        for result in rx {
            match_count += 1;

            println!(
                "{} {} {}{}{} {}",
                "[+]".green().bold(),
                result.file_path.cyan(),
                "(Line ".yellow(),
                result.line_num.to_string().yellow(),
                ")".yellow(),
                result.content.red().bold()
            );

            if let Some(ref mut file) = out_file {
                let clean_text = format!("[+] {} (Line {}) {}\n", result.file_path, result.line_num, result.content);
                let _ = file.write_all(clean_text.as_bytes());
            }
        }
        
        match_count 
    });

    let args_extract = args.extract; // Copy bool for the threads

    target_files.par_iter().for_each(|file| {
        let thread_tx = tx.clone();
        search_file(file, &search_pattern, regex_pattern.as_ref(), args_extract, thread_tx);
    });

    drop(tx); 

    let total_matches = receiver_thread.join().unwrap();

    println!("\n{} Search complete! Found {} matches.", "[*]".blue().bold(), total_matches.to_string().magenta().bold());
}
