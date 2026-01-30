# shred

A Rust implementation of the Unix `shred(1)` utility for securely overwriting files.

## Overview

`shred` overwrites files multiple times with random data, making it difficult to recover the original contents. The final pass uses zeros to hide that shredding occurred.

## Usage
```
shred [OPTIONS] <FILE(S)>
```

### Arguments

- `<FILE(S)>` — File(s) to shred

### Options

| Flag | Long | Description |
|------|------|-------------|
| `-n` | `--iterations <N>` | Number of overwrite passes (default: 3) |
| `-q` | `--quiet` | Suppress progress information |
| `-u` | `--remove[=HOW]` | Remove the file after shredding [HOW: unlink, wipe, wipesync] (default: unlink) |
| `s` | `--size <N>` | Size of overwrite in bytes (default: 4096) |
| `-f` | `--force` | Skip confirmation prompt |
| `-h` | `--help` | Print help |

## Examples
```bash
# Basic shred (3 passes)
shred secret.txt

# Suppress progress output with 5 passes
shred -q -n 5 secret.txt

# Shred and delete
shred -qu secret.txt
```

## Building
```bash
cargo build --release
```

The binary will be at `target/release/shred`.

## Limitations

- May not be effective on journaling filesystems, SSDs, or RAID arrays
- Does not shred filenames or directory entries
- Single file only (no directory recursion)

## License

MIT
