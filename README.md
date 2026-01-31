# shred

A Rust implementation of the Unix `shred(1)` utility for securely overwriting files.

## Overview

`shred` overwrites files multiple times with random data, making it difficult to recover the original contents. The final pass uses zeros to hide that shredding occurred.

## Usage
```
shred [OPTIONS] <FILE(S)> <DIR(S)>
```

### Arguments

- `<FILE(S)>` — File(s) to shred
- `<DIR(S)>` — Directory(s) to shred

### Options

| Flag | Long | Description |
|------|------|-------------|
| `-d` | `--dry-run` | Show what would happen without actually shredding |
| `-f` | `--force` | Skip confirmation prompt |
| `-h` | `--help` | Print help |
| `-n` | `--iterations <N>` | Number of overwrite passes (default: 3) |
| `-q` | `--quiet` | Suppress progress information |
| `-r` | `--recursive` | Recursively shred all files in a directory |
| `-s` | `--size <N>` | Size of overwrite in bytes |
| `-u` | `--remove[=HOW]` | Remove the file after shredding [HOW: unlink, wipe, wipesync] (default: unlink) |
| `-z` | `--zero` | Add a final pass with zeroes to hide shredding |

## Examples
```bash
# Basic shred (3 passes)
shred secret.txt

# Suppress progress output with 5 passes
shred -q -n 5 secret.txt

# Shred and delete
shred -qu secret.txt

# Recursive shred (force, remove, wipe)
shred -r -f -u=wipe secret_dir
```

## Building
```bash
cargo build --release
```

The binary will be at `target/release/shred`.

## Limitations

- May not be effective on journaling filesystems, SSDs, or RAID arrays
- Does not shred filenames or directory entries

## License

MIT
