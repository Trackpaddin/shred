use std::env;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};


fn overwrite_file(file: &mut std::fs::File, file_size: u64) {
    const BUFFER_SIZE: usize = 4096;
    let buffer = [0u8; BUFFER_SIZE];

    file.seek(SeekFrom::Start(0)).expect("Failed to seek");

    let mut bytes_written: u64 = 0;

    while bytes_written + BUFFER_SIZE as u64 <= file_size {
        file.write_all(&buffer).expect("Failed to write");
        bytes_written += BUFFER_SIZE as u64;
    }

    let remaining = (file_size - bytes_written) as usize;
    if remaining > 0 {
        file.write_all(&buffer[..remaining]).expect("Failed to write");
    }

    file.sync_all().expect("Failed to sync to disk");
}    

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(filename)
        .expect("Failed to open file");

    let file_size = file.metadata().expect("Failed to get metadata").len();

    println!("Shredding '{}' ({} bytes)", filename, file_size);

    let passes = 3;

    for i in 1..=passes {
        println!("Pass {}/{}", i, passes);
        overwrite_file(&mut file, file_size);
    }

    println!("File overwritten {} times.", passes);
}
