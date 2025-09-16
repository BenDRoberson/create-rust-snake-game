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

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Specific Test
```bash
cargo test test_direction_opposite
```

### Run Property-Based Tests
```bash
cargo test --features proptest
```

### Run Benchmarks
```bash
cargo bench
```

### Run Benchmarks with Specific Target
```bash
cargo bench --bench snake_performance
```

## Test Coverage Areas

### 1. Core Game Logic
- ✅ Position validation and movement
- ✅ Direction handling and opposites
- ✅ Snake movement mechanics
- ✅ Collision detection (walls and self)
- ✅ Food consumption and growth
- ✅ Score tracking

### 2. Game State Management
- ✅ Initial state setup
- ✅ State transitions
- ✅ Game over conditions
- ✅ Input handling
- ✅ Speed progression

### 3. Edge Cases
- ✅ Boundary conditions
- ✅ Full board scenarios
- ✅ Self-collision patterns
- ✅ Input validation
- ✅ Food generation constraints

### 4. Performance
- ✅ Basic operation speed
- ✅ Scalability with large snakes
- ✅ Memory usage patterns
- ✅ Edge case performance

## Testing Best Practices Demonstrated

### 1. Test Isolation
Each test is independent and doesn't rely on external state.

### 2. Descriptive Test Names
Test names clearly describe what is being tested:
```rust
#[test]
fn test_snake_movement_and_growth() {
    // Test implementation
}
```

### 3. Arrange-Act-Assert Pattern
```rust
#[test]
fn test_position_move_in_direction() {
    // Arrange
    let pos = Position::new(10, 10);
    
    // Act
    let moved = pos.move_in_direction(Direction::Up);
    
    // Assert
    assert_eq!(moved, Position::new(10, 9));
}
```

### 4. Property-Based Testing
Testing mathematical properties rather than specific values:
```rust
proptest! {
    #[test]
    fn test_direction_opposite_property(direction in prop::sample::select(vec![...])) {
        assert_eq!(direction.opposite().opposite(), direction);
    }
}
```

### 5. Test Helpers
Reusable helper functions for common test scenarios:
```rust
fn create_custom_game_state(snake: Vec<Position>, direction: Direction) -> GameState {
    // Helper implementation
}
```

### 6. Comprehensive Edge Case Testing
Testing boundary conditions and error scenarios:
```rust
#[test]
fn test_boundary_movement() {
    // Test movement at all four edges
    let test_cases = [
        (Position::new(0, GRID_HEIGHT / 2), Direction::Left),
        // ... more cases
    ];
}
```

## Performance Testing Insights

The benchmarks help identify:

1. **Bottlenecks**: Which operations are slowest
2. **Scalability**: How performance degrades with size
3. **Optimization Opportunities**: Where to focus improvements
4. **Regression Detection**: Performance doesn't degrade over time

## Continuous Integration

For CI/CD pipelines, consider running:

```bash
# Run all tests
cargo test --all-features

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html

# Run benchmarks
cargo bench --no-run
```

## Test Maintenance

### Adding New Tests
1. Identify the component/feature to test
2. Choose appropriate test type (unit/integration/property)
3. Follow naming conventions
4. Ensure test isolation
5. Add to appropriate test module

### Updating Tests
When modifying game logic:
1. Update existing tests to match new behavior
2. Add tests for new features
3. Ensure all tests still pass
4. Update benchmarks if performance characteristics change

## Common Testing Patterns

### Testing Private Methods
Use `pub` visibility for testable methods or test through public interfaces.

### Mocking Dependencies
For more complex scenarios, consider using mock objects or dependency injection.

### Test Data Generation
Use property-based testing or helper functions for consistent test data.

### Error Testing
Test both success and failure scenarios:
```rust
#[test]
fn test_invalid_position() {
    assert!(!Position::new(-1, 5).is_valid());
}
```

This testing strategy ensures the Snake Game is robust, performant, and maintainable while demonstrating comprehensive Rust testing practices.
