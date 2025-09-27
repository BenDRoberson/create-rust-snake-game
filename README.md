# Snake Game

[![CI](https://github.com/Bendroberson/create-rust-snake-game/workflows/CI/badge.svg)](https://github.com/Bendroberson/create-rust-snake-game/actions)
[![Latest Release](https://img.shields.io/github/v/release/Bendroberson/create-rust-snake-game)](https://github.com/Bendroberson/create-rust-snake-game/releases)

A high-performance Snake game built in Rust with comprehensive testing and CI/CD automation. Highly vibe coded - using this to learn some rust and play around with LLMs.

## Download and Play

1. Go to the [Releases](https://github.com/Bendroberson/create-rust-snake-game/releases) page
2. Download the binary for your platform
3. Snake away

### Build from Source

```bash
git clone https://github.com/Bendroberson/create-rust-snake-game.git
cd create-rust-snake-game
cargo run --release
```

## Controls

- **Arrow Keys** or **WASD**: Move the snake
- **R**: Restart the game
- **ESC**: Quit the game

## Development

### Testing

```bash
# Run all tests
cargo test

# Run benchmarks
cargo bench

# Check code quality
cargo clippy
cargo fmt --check
```

See [TESTING_GUIDE.md](TESTING_GUIDE.md) for detailed testing information.
