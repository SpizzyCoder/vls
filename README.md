# vls

Small, cross-platform `ls`-like CLI in Rust.

## Install

```bash
cargo build --release
```

Binary:
- `target/release/vls`
- `target\release\vls.exe`

## Usage

```text
vls [OPTIONS] [PATH]
```

`PATH` defaults to `.`.

Options:
- `-c, --creation-date`
- `-m, --modification-date`
- `-a, --access-date`
- `-s, --size`
- `--sys` (include dotfiles)
- `-r, --recursive` (recursive dir size estimate)
- `-f, --format <iec|si>` (default: `iec`)
- `-h, --help`
- `-V, --version`

## Examples

```bash
vls
vls -m -s
vls -s -r -f si ./path
vls --sys
```

## Notes

- Type prefix: `D` dir, `F` file, `S` symlink, `U` unknown.
- Hidden files are filtered by dot-prefix unless `--sys` is set.
- If no entries are visible, output is `Nothing`.

## Dev

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```
