# Testing Guide

This project includes comprehensive testing across multiple levels to ensure reliability and performance - or at least that is what the LLM promised.

## Test Types

- **Unit Tests** - Individual component testing in `src/lib.rs`
- **Integration Tests** - Component interaction testing in `tests/integration_tests.rs`
- **Property-Based Tests** - Mathematical property verification using `proptest`
- **Performance Benchmarks** - Performance regression detection in `benches/snake_performance.rs`

## Running Tests

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test integration_tests

# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench snake_performance
```

## Test Coverage

### Unit Tests

- `test_direction_opposite()` - Direction reversal logic
- `test_position_move_in_direction()` - Movement in all directions
- `test_position_is_valid()` - Boundary validation
- `test_would_collide_wall()` - Wall collision detection
- `test_would_collide_self()` - Self-collision detection
- `test_handle_input_prevents_reversal()` - Input validation
- `test_snake_movement_and_growth()` - Snake growth mechanics
- `test_game_over_on_collision()` - Game over conditions

### Integration Tests

- `test_complete_game_session()` - Full game flow simulation
- `test_multiple_food_consumption()` - Food eating mechanics
- `test_boundary_movement()` - Edge case handling
- `test_game_restart()` - Game reset functionality
- `test_self_collision()` - Self-collision scenarios

### Property-Based Tests

- `test_position_move_direction_property()` - Movement distance property
- `test_direction_opposite_property()` - Opposite direction symmetry
- `test_snake_growth_invariant()` - Snake length preservation
