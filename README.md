# Rust Snake Game with Comprehensive Testing

A complete Snake game implementation in Rust with extensive testing coverage, demonstrating best practices for testing in Rust applications.

## Features

- **Complete Snake Game**: Full implementation with graphics, input handling, and game logic
- **Comprehensive Testing**: Unit tests, integration tests, property-based tests, and performance benchmarks
- **Modular Design**: Clean separation between game logic and rendering
- **Performance Optimized**: Benchmarked for optimal performance

## Game Controls

- **Arrow Keys** or **WASD**: Move the snake
- **R**: Restart the game
- **Ctrl+R**: Force restart

## Testing Strategy

This project demonstrates a comprehensive testing approach with multiple levels:

### 1. Unit Tests
- Test individual components in isolation
- Cover core game logic (Position, Direction, GameState)
- Verify edge cases and error conditions

### 2. Integration Tests
- Test component interactions
- Verify complete game flows
- Test real-world scenarios

### 3. Property-Based Tests
- Test mathematical properties and invariants
- Use `proptest` for randomized testing
- Ensure correctness across many inputs

### 4. Performance Benchmarks
- Measure operation performance
- Identify bottlenecks
- Track performance regressions

## Running the Game

```bash
# Run the game
cargo run

# Run in release mode for better performance
cargo run --release
```

## Running Tests

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test integration_tests

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_direction_opposite
```

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench snake_movement
```

## Project Structure

```
src/
├── lib.rs          # Core game logic and library interface
├── main.rs         # Application entry point
tests/
├── integration_tests.rs  # Integration tests
benches/
├── snake_performance.rs  # Performance benchmarks
TESTING_GUIDE.md          # Comprehensive testing documentation
```

## Key Learning Points

### Testing Best Practices Demonstrated

1. **Test Isolation**: Each test is independent
2. **Descriptive Names**: Clear test names that explain intent
3. **Arrange-Act-Assert**: Structured test organization
4. **Property-Based Testing**: Testing invariants, not just specific cases
5. **Performance Testing**: Ensuring good performance characteristics
6. **Edge Case Coverage**: Testing boundary conditions and error scenarios

### Rust-Specific Testing Techniques

1. **Module Organization**: Using `#[cfg(test)]` for unit tests
2. **Integration Testing**: Separate test files for integration tests
3. **Property Testing**: Using `proptest` for randomized testing
4. **Benchmarking**: Using `criterion` for performance measurement
5. **Test Helpers**: Reusable functions for common test scenarios

### Game Development Patterns

1. **State Management**: Clean separation of game state
2. **Event Handling**: Proper input processing
3. **Collision Detection**: Efficient boundary and self-collision checks
4. **Game Loop**: Time-based updates and rendering

## Dependencies

- **ggez**: Game framework for graphics and input
- **rand**: Random number generation for food placement
- **proptest**: Property-based testing
- **criterion**: Performance benchmarking

## Performance Results

The benchmarks show excellent performance:
- Position operations: ~150ns
- Collision detection: ~7-108ns (scales with snake size)
- Snake movement: ~65-70ns per move
- Game state creation: ~92ns

## Testing Coverage

The test suite covers:
- ✅ Core game logic (Position, Direction, GameState)
- ✅ Snake movement and growth mechanics
- ✅ Collision detection (walls and self)
- ✅ Food consumption and score tracking
- ✅ Input handling and validation
- ✅ Game over conditions
- ✅ Edge cases and boundary conditions
- ✅ Performance characteristics
- ✅ Property invariants

## Contributing

When adding new features:
1. Write unit tests for new functionality
2. Add integration tests for component interactions
3. Update benchmarks if performance characteristics change
4. Follow the existing test patterns and naming conventions

## License

This project is provided as an educational example of comprehensive testing in Rust.
