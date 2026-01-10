use std::fs::{remove_file, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use rand::RngCore;
use clap::Parser;

#[derive(Parser)]
#[command(name = "shred")]
#[command(about = "Securely overwrite files to hide their contents")]
struct Args {
    /// File to shred
    file: String,

    /// Number of overwrite passes
    #[arg(short = 'n', long = "iterations", default_value = "3")]
    passes: u32,
    /// Show progress information
    #[arg(short, long)]
    verbose: bool,
    /// Remove the file after shredding
    #[arg(short = 'u', long)]
    remove: bool,
}

fn overwrite_file(file: &mut std::fs::File, file_size: u64, use_random: bool) -> std::io::Result<()> {
    const BUFFER_SIZE: usize = 4096;
    let mut buffer = [0u8; BUFFER_SIZE];

    if use_random {
        rand::thread_rng().fill_bytes(&mut buffer);
    }

    file.seek(SeekFrom::Start(0))?;

    let mut bytes_written: u64 = 0;

    while bytes_written + BUFFER_SIZE as u64 <= file_size {
        file.write_all(&buffer)?;
        bytes_written += BUFFER_SIZE as u64;
    }

    let remaining = (file_size - bytes_written) as usize;
    if remaining > 0 {
        file.write_all(&buffer[..remaining])?;
    }

    file.sync_all()?;
    Ok(())
}    

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let filename = &args.file;

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(filename)?;

    let file_size = file.metadata()?.len();

    println!("Shredding '{}' ({} bytes)...", filename, file_size);

    let passes = args.passes;

    for i in 1..=passes {
        let use_random = i < passes;
        if args.verbose {
            let pattern = if use_random { "random" } else { "zeroes" };
            println!("Pass {}/{} ({})...", i, passes, pattern);            
        }
        overwrite_file(&mut file, file_size, use_random);
    }

    if args.verbose {
        println!("Shredding '{}' ({} bytes)...", filename, file_size);
    }
    if args.remove {
        remove_file(filename)?;
        if args.verbose {
        println!("File '{}' removed.", filename);
        }
    }
    Ok(())
}
