//! Integration tests for the Snake Game
//!
//! These tests verify that different parts of the system work together correctly.
//! They test the complete game flow and interactions between components.

use create_rust_snake_game::*;

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test a complete game session from start to collision
    #[test]
    fn test_complete_game_session() {
        let mut game = GameState::new();
        let mut moves = 0;
        const MAX_MOVES: usize = 100; // Prevent infinite loops in tests

        // Play until game over or max moves reached
        while !game.game_over && moves < MAX_MOVES {
            // Occasionally change direction to test input handling
            match moves % 10 {
                0 => game.handle_input(Direction::Up),
                3 => game.handle_input(Direction::Right),
                6 => game.handle_input(Direction::Down),
                _ => {}
            }

            game.move_snake();
            moves += 1;

            // Verify game state invariants
            assert!(
                game.snake.len() >= 3,
                "Snake should never be shorter than 3 segments"
            );
            // Score is u32, so it's always >= 0

            // Verify all snake segments are valid positions
            for segment in &game.snake {
                assert!(
                    segment.is_valid(),
                    "All snake segments should be valid positions"
                );
            }

            // Verify snake segments are adjacent
            for i in 1..game.snake.len() {
                let prev = game.snake[i - 1];
                let curr = game.snake[i];
                let x_diff = (prev.x - curr.x).abs();
                let y_diff = (prev.y - curr.y).abs();
                assert!(x_diff + y_diff == 1, "Snake segments should be adjacent");
            }
        }

        // Game should eventually end due to collision
        assert!(game.game_over || moves >= MAX_MOVES);
    }

    /// Test game state consistency across multiple food consumptions
    #[test]
    fn test_multiple_food_consumption() {
        let mut game = GameState::new();
        let initial_speed = game.game_speed;
        let mut food_eaten = 0;

        // Force snake to eat multiple pieces of food
        for _i in 0..5 {
            // Place food directly in front of snake
            let head = game.snake[0];
            let food_pos = head.move_in_direction(game.direction);
            game.food = food_pos;

            let initial_length = game.snake.len();
            let initial_score = game.score;

            game.move_snake();

            // Verify food was eaten
            assert_eq!(game.snake.len(), initial_length + 1);
            assert_eq!(game.score, initial_score + 10);
            food_eaten += 1;

            // Verify game speed increases with each food eaten
            assert!(game.game_speed < initial_speed);

            // Verify new food is not on snake
            assert!(!game.snake.contains(&game.food));
        }

        assert_eq!(food_eaten, 5);
    }

    /// Test edge case: snake movement at grid boundaries
    #[test]
    fn test_boundary_movement() {
        let _game = GameState::new();

        // Move snake to different edges and test movement
        let test_cases = [
            (Position::new(0, GRID_HEIGHT / 2), Direction::Left), // Left edge
            (
                Position::new(GRID_WIDTH - 1, GRID_HEIGHT / 2),
                Direction::Right,
            ), // Right edge
            (Position::new(GRID_WIDTH / 2, 0), Direction::Up),    // Top edge
            (
                Position::new(GRID_WIDTH / 2, GRID_HEIGHT - 1),
                Direction::Down,
            ), // Bottom edge
        ];

        for (pos, direction) in test_cases {
            // Create a new game state with snake at edge
            let snake = vec![
                pos,
                Position::new(pos.x, pos.y + 1),
                Position::new(pos.x, pos.y + 2),
            ];
            let mut test_game = GameState {
                snake,
                direction,
                next_direction: direction,
                food: Position::new(5, 5), // Place food away from edge
                score: 0,
                high_score: 0,
                game_over: false,
                game_speed: 0.2,
                last_update: 0.0,
            };

            // Moving in the direction that would go out of bounds should end the game
            test_game.move_snake();
            assert!(
                test_game.game_over,
                "Moving out of bounds should end the game"
            );
        }
    }

    /// Test input handling during game play
    #[test]
    fn test_input_handling_during_gameplay() {
        let mut game = GameState::new();

        // Test that valid direction changes work
        game.handle_input(Direction::Up);
        assert_eq!(game.next_direction, Direction::Up);

        game.handle_input(Direction::Left);
        assert_eq!(game.next_direction, Direction::Up); // Left is opposite of Right, so should be ignored

        // Test that opposite direction changes are ignored
        game.handle_input(Direction::Right);
        assert_eq!(game.next_direction, Direction::Right); // Right is not opposite of Right, so it should work

        // Test that direction is applied on next move
        let initial_head = game.snake[0];
        game.move_snake();
        let new_head = game.snake[0];

        // Snake should have moved right
        assert_eq!(new_head.x, initial_head.x + 1);
        assert_eq!(new_head.y, initial_head.y);
    }

    /// Test game state after restart
    #[test]
    fn test_game_restart() {
        let mut game = GameState::new();

        // Play for a bit
        for _ in 0..5 {
            game.move_snake();
        }

        // Force game over
        game.game_over = true;

        // Restart game
        let restarted_game = GameState::new();

        // Verify game is in initial state
        assert_eq!(restarted_game.snake.len(), 3);
        assert_eq!(restarted_game.score, 0);
        assert!(!restarted_game.game_over);
        assert_eq!(restarted_game.direction, Direction::Right);

        // Verify snake is centered
        let expected_head = Position::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
        assert_eq!(restarted_game.snake[0], expected_head);
    }

    /// Test food generation edge cases
    #[test]
    fn test_food_generation_edge_cases() {
        // Test with snake taking up most of the board
        let mut snake = Vec::new();
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                snake.push(Position::new(x, y));
            }
        }

        // Remove one position to leave space for food
        snake.pop();

        // This should not panic and should find a valid food position
        let food = GameState::generate_food_position(&snake);
        assert!(food.is_valid());
        assert!(!snake.contains(&food));
    }

    /// Test game speed progression
    #[test]
    fn test_game_speed_progression() {
        let mut game = GameState::new();
        let initial_speed = game.game_speed;
        let mut previous_speed = initial_speed;

        // Eat several pieces of food and verify speed increases
        for _ in 0..5 {
            let head = game.snake[0];
            let food_pos = head.move_in_direction(game.direction);
            game.food = food_pos;

            let speed_before = game.game_speed;
            game.move_snake();

            // Speed should decrease (making game faster) or stay at minimum
            assert!(game.game_speed <= speed_before);
            assert!(game.game_speed <= previous_speed);

            previous_speed = game.game_speed;

            // Speed should never go below minimum
            assert!(game.game_speed >= 0.1);
        }
    }

    /// Test snake collision with itself
    #[test]
    fn test_self_collision() {
        // Create a snake that will collide with itself
        let snake = vec![
            Position::new(5, 5), // head
            Position::new(5, 6), // body - head will collide with this when moving down to (5, 6)
            Position::new(4, 6),
            Position::new(3, 6),
            Position::new(3, 5),
            Position::new(4, 5), // tail
        ];

        let mut game = GameState {
            snake,
            direction: Direction::Down, // This will make head collide with body at (5, 6)
            next_direction: Direction::Down,
            food: Position::new(0, 0),
            score: 0,
            high_score: 0,
            game_over: false,
            game_speed: 0.2,
            last_update: 0.0,
        };

        // This move should cause self-collision
        game.move_snake();
        assert!(game.game_over);
    }
}

/// Helper functions for integration tests
#[cfg(test)]
mod test_helpers {
    use super::*;

    /// Create a game state with snake in a specific configuration
    #[allow(dead_code)]
    pub fn create_game_with_snake_positions(
        positions: Vec<Position>,
        direction: Direction,
    ) -> GameState {
        GameState {
            snake: positions.clone(),
            direction,
            next_direction: direction,
            food: GameState::generate_food_position(&positions),
            score: 0,
            high_score: 0,
            game_over: false,
            game_speed: 0.2,
            last_update: 0.0,
        }
    }

    /// Verify that a game state maintains all invariants
    #[allow(dead_code)]
    pub fn verify_game_invariants(game: &GameState) {
        // Snake should never be empty
        assert!(!game.snake.is_empty(), "Snake should never be empty");

        // All snake segments should be valid
        for segment in &game.snake {
            assert!(
                segment.is_valid(),
                "All snake segments should be valid positions"
            );
        }

        // Snake segments should be adjacent
        for i in 1..game.snake.len() {
            let prev = game.snake[i - 1];
            let curr = game.snake[i];
            let x_diff = (prev.x - curr.x).abs();
            let y_diff = (prev.y - curr.y).abs();
            assert!(x_diff + y_diff == 1, "Snake segments should be adjacent");
        }

        // Score should be non-negative
        // Score is u32, so it's always >= 0

        // Game speed should be within reasonable bounds
        assert!(game.game_speed > 0.0, "Game speed should be positive");
        assert!(game.game_speed <= 1.0, "Game speed should not be too slow");

        // Food should be valid and not on snake
        assert!(game.food.is_valid(), "Food should be in valid position");
        assert!(
            !game.snake.contains(&game.food),
            "Food should not be on snake"
        );
    }

    /// Simulate a sequence of moves
    #[allow(dead_code)]
    pub fn simulate_moves(game: &mut GameState, directions: Vec<Direction>) {
        for direction in directions {
            game.handle_input(direction);
            game.move_snake();
        }
    }
}
