use std::path::Path;
use std::fs::{self, OpenOptions, remove_file};
use std::io::{self, Seek, SeekFrom, Write};
use rand::RngCore;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(name = "shred")]
#[command(about = "Securely overwrite files to hide their contents")]
struct Args {
    /// Files to shred
    #[arg(required = true)]
    files: Vec<String>,

    /// Number of overwrite passes
    #[arg(short = 'n', long = "iterations", default_value = "3")]
    passes: u32,
    /// Suppress progress information
    #[arg(short, long)]
    quiet: bool,
    /// Remove the file after shredding
    #[arg(short = 'u', long)]
    remove: bool,
    ///  Skip confirmation prompt
    #[arg(short, long)]
    force: bool,
    /// Add a final pass with zeroes to hide shredding
    #[arg(short,long)]
    zero: bool,
}

fn overwrite_file(file: &mut std::fs::File, file_size: u64, use_random: bool, quiet: bool) -> std::io::Result<()> {
    const BUFFER_SIZE: usize = 4096;
    let mut buffer = [0u8; BUFFER_SIZE];

    if use_random {
        rand::thread_rng().fill_bytes(&mut buffer);
    }

    let progress = if !quiet {
        let pb = ProgressBar::new(file_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{bar:40} {percent}% ({bytes}/{total_bytes})")
            .expect("Invalid template"));
        Some(pb)
    } else {
        None
    };

    file.seek(SeekFrom::Start(0))?;

    let mut bytes_written: u64 = 0;

    while bytes_written + BUFFER_SIZE as u64 <= file_size {
        file.write_all(&buffer)?;
        bytes_written += BUFFER_SIZE as u64;
        if let Some(ref pb) = progress {
            pb.set_position(bytes_written);
        }    
    }

    let remaining = (file_size - bytes_written) as usize;
    if remaining > 0 {
        file.write_all(&buffer[..remaining])?;
    }

    if let Some(ref pb) = progress {
        // finish() leaves bar visible
        pb.finish_and_clear();
    }

    file.sync_all()?;
    Ok(())
}  

fn validate_file(path: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(format!("File '{}' does not exist.", path.display()).into());
    }

    if path.is_symlink() {
        return Err(format!("File '{}' is a symbolic link; refusing to shred.", path.display()).into());
    }

    if !path.is_file() {
        return Err(format!("File '{}' is not a file.", path.display()).into());
    }    

    let metadata = fs::metadata(path)?;
    if metadata.permissions().readonly() {
        return Err(format!("File '{}' is read-only.", path.display()).into());
    }

    if metadata.len() == 0 {
        eprintln!("File '{}' is empty.", path.display());
    }

    if !force {
        eprintln!("Are you sure you want to shred '{}'? (y/N)", path.display());
        io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            return Err("Aborted by user.".into());
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    for filename in &args.files {
        validate_file(filename, args.force)?;
    }

    for filename in &args.files {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(filename)?;

        let file_size = file.metadata()?.len();

        let passes = args.passes;

        for i in 1..=passes {
            if !args.quiet {
                println!("{}: Pass {}/{} (random)...", filename, i, passes);
            }
            overwrite_file(&mut file, file_size, true, !args.quiet)?;
        }

        if args.zero {
            if !args.quiet {
                println!("{}: Final pass (zeros)...", filename);
            }
            overwrite_file(&mut file, file_size, false, !args.quiet)?;
        }

        if !args.quiet {
            println!("Shredded '{}' ({} bytes)...", filename, file_size);
        }

        if args.remove {
            remove_file(filename)?;
            if !args.quiet {
            println!("File '{}' removed.", filename);
            }
        }
    }

    Ok(())
}
