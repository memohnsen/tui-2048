# TUI 2048

A terminal-based implementation of the classic 2048 game, written in Rust using [ratatui](https://github.com/ratatui-org/ratatui) and [crossterm](https://github.com/crossterm-rs/crossterm).

## Features
- **Classic 2048 gameplay** right in your terminal.
- **High Scores**: Automatically saves your scores and highest tile to `HOME/.config/2048/scores.txt`.
- **Vim-like controls**: Support for both arrow keys and Vim bindings (`h`, `j`, `k`, `l`).

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed. Then, you can run the game with:

```bash
cargo run --release
```

To build a release binary:

```bash
cargo build --release
```

The compiled binary will be available at `target/release/tui-2048`.

## Controls

| Key | Action |
| --- | --- |
| `↑` or `k` | Move Up |
| `↓` or `j` | Move Down |
| `←` or `h` | Move Left |
| `→` or `l` | Move Right |
| `n` | New Game |
| `s` | View High Scores |
| `q` | Quit |
