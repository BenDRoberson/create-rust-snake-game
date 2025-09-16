# Snake Game Testing Guide

This guide explains the comprehensive testing strategy implemented for the Rust Snake Game, demonstrating various testing techniques and best practices in Rust.

## Testing Strategy Overview

The testing approach covers multiple levels of testing:

1. **Unit Tests** - Test individual components in isolation
2. **Integration Tests** - Test component interactions
3. **Property-Based Tests** - Test invariants and properties
4. **Performance Benchmarks** - Ensure good performance
5. **Edge Case Testing** - Handle boundary conditions

## Test Structure

### Unit Tests (`src/lib.rs`)

Unit tests are embedded in the library code using `#[cfg(test)]` modules. They test:

#### Direction Tests

- `test_direction_opposite()` - Verifies opposite direction logic
- `test_direction_opposite_is_symmetric()` - Ensures symmetry property

#### Position Tests

- `test_position_creation()` - Basic position creation
- `test_position_move_in_direction()` - Movement in all directions
- `test_position_is_valid()` - Boundary validation

#### GameState Tests

- `test_game_state_new()` - Initial state verification
- `test_would_collide_wall()` - Wall collision detection
- `test_would_collide_self()` - Self-collision detection
- `test_handle_input_prevents_reversal()` - Input validation
- `test_snake_movement_and_growth()` - Snake growth mechanics
- `test_game_over_on_collision()` - Game over conditions

### Integration Tests (`tests/integration_tests.rs`)

Integration tests verify that components work together:

- `test_complete_game_session()` - Full game flow simulation
- `test_multiple_food_consumption()` - Food eating mechanics
- `test_boundary_movement()` - Edge case handling
- `test_input_handling_during_gameplay()` - Input during game
- `test_game_restart()` - Game reset functionality
- `test_food_generation_edge_cases()` - Food generation with full board
- `test_game_speed_progression()` - Speed increase mechanics
- `test_self_collision()` - Self-collision scenarios

### Property-Based Tests

Using the `proptest` crate, these tests verify mathematical properties:

- `test_position_move_direction_property()` - Movement distance property
- `test_direction_opposite_property()` - Opposite direction symmetry
- `test_snake_growth_invariant()` - Snake length preservation

### Performance Benchmarks (`benches/snake_performance.rs`)

Comprehensive performance testing:

- **Basic Operations**: Position creation, validation, movement
- **Game Logic**: State creation, food generation, collision detection
- **Scalability**: Performance with different snake sizes
- **Edge Cases**: Boundary conditions and special scenarios

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Unit Tests Only

```bash
cargo test --lib
```

### Run Integration Tests Only

```bash
cargo test --test integration_tests
```

### Run Benchmarks

```bash
cargo bench
```

### Run Benchmarks with Specific Target

```bash
cargo bench --bench snake_performance
```
