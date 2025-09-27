//! Performance benchmarks for the Snake Game
//!
//! These benchmarks help identify performance bottlenecks and ensure
//! the game runs smoothly even with complex scenarios.

use create_rust_snake_game::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn benchmark_position_creation(c: &mut Criterion) {
    c.bench_function("position_creation", |b| {
        b.iter(|| {
            for x in 0..20 {
                for y in 0..15 {
                    black_box(Position::new(x, y));
                }
            }
        })
    });
}

fn benchmark_position_validation(c: &mut Criterion) {
    let positions = (0..20)
        .flat_map(|x| (0..15).map(move |y| Position::new(x, y)))
        .collect::<Vec<_>>();

    c.bench_function("position_validation", |b| {
        b.iter(|| {
            for pos in &positions {
                black_box(pos.is_valid());
            }
        })
    });
}

fn benchmark_direction_opposite(c: &mut Criterion) {
    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    c.bench_function("direction_opposite", |b| {
        b.iter(|| {
            for direction in &directions {
                black_box(direction.opposite());
            }
        })
    });
}

fn benchmark_position_movement(c: &mut Criterion) {
    let positions = (0..20)
        .flat_map(|x| (0..15).map(move |y| Position::new(x, y)))
        .collect::<Vec<_>>();
    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    c.bench_function("position_movement", |b| {
        b.iter(|| {
            for pos in &positions {
                for direction in &directions {
                    black_box(pos.move_in_direction(*direction));
                }
            }
        })
    });
}

fn benchmark_game_state_creation(c: &mut Criterion) {
    c.bench_function("game_state_creation", |b| {
        b.iter(|| black_box(GameState::new()))
    });
}

fn benchmark_food_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("food_generation");

    // Test with different snake sizes
    for snake_size in [3, 10, 50, 100, 200] {
        let snake = (0..snake_size)
            .map(|i| Position::new(i % GRID_WIDTH, i / GRID_WIDTH))
            .collect::<Vec<_>>();

        group.bench_with_input(
            BenchmarkId::new("snake_size", snake_size),
            &snake,
            |b, snake| b.iter(|| black_box(GameState::generate_food_position(snake))),
        );
    }
    group.finish();
}

fn benchmark_collision_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("collision_detection");

    for snake_size in [3, 10, 50, 100] {
        let snake = (0..snake_size)
            .map(|i| Position::new(i % GRID_WIDTH, i / GRID_WIDTH))
            .collect::<Vec<_>>();

        let game = GameState {
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: Position::new(5, 5),
            score: 0,
            high_score: 0,
            game_over: false,
            game_speed: 0.2,
            last_update: 0.0,
        };

        let test_positions = vec![
            Position::new(-1, 5),  // Wall collision
            Position::new(5, 5),   // Potential body collision
            Position::new(10, 10), // Valid position
        ];

        group.bench_with_input(
            BenchmarkId::new("snake_size", snake_size),
            &game,
            |b, game| {
                b.iter(|| {
                    for pos in &test_positions {
                        black_box(game.would_collide(*pos));
                    }
                })
            },
        );
    }
    group.finish();
}

fn benchmark_snake_movement(c: &mut Criterion) {
    let mut group = c.benchmark_group("snake_movement");

    for snake_size in [3, 10, 50, 100] {
        let snake = (0..snake_size)
            .map(|i| Position::new(i % GRID_WIDTH, i / GRID_WIDTH))
            .collect::<Vec<_>>();

        let game = GameState {
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: Position::new(0, 0), // Place food away from snake
            score: 0,
            high_score: 0,
            game_over: false,
            game_speed: 0.2,
            last_update: 0.0,
        };

        group.bench_with_input(
            BenchmarkId::new("snake_size", snake_size),
            &game,
            |b, game| {
                b.iter(|| {
                    let mut game_copy = game.clone();

                    // Move snake multiple times
                    for _ in 0..10 {
                        game_copy.move_snake();
                    }
                })
            },
        );
    }
    group.finish();
}

fn benchmark_snake_growth(c: &mut Criterion) {
    let mut group = c.benchmark_group("snake_growth");

    for initial_size in [3, 10, 20, 50] {
        let snake = (0..initial_size)
            .map(|i| Position::new(i % GRID_WIDTH, i / GRID_WIDTH))
            .collect::<Vec<_>>();

        group.bench_with_input(
            BenchmarkId::new("initial_size", initial_size),
            &snake,
            |b, snake| {
                b.iter(|| {
                    let mut game = GameState {
                        snake: snake.clone(),
                        direction: Direction::Right,
                        next_direction: Direction::Right,
                        food: Position::new(0, 0), // Place food away from snake
                        score: 0,
                        high_score: 0,
                        game_over: false,
                        game_speed: 0.2,
                        last_update: 0.0,
                    };

                    // Simulate snake eating food and growing
                    for _ in 0..5 {
                        let head = game.snake[0];
                        let food_pos = head.move_in_direction(game.direction);
                        game.food = food_pos;
                        game.move_snake();
                    }

                    black_box(game);
                })
            },
        );
    }
    group.finish();
}

fn benchmark_input_handling(c: &mut Criterion) {
    let mut game = GameState::new();
    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    c.bench_function("input_handling", |b| {
        b.iter(|| {
            for direction in &directions {
                game.handle_input(*direction);
            }
        })
    });
}

fn benchmark_full_game_simulation(c: &mut Criterion) {
    c.bench_function("full_game_simulation", |b| {
        b.iter(|| {
            let mut game = GameState::new();
            let mut moves = 0;
            const MAX_MOVES: usize = 100;

            // Simulate a complete game session
            while !game.game_over && moves < MAX_MOVES {
                // Occasionally change direction
                match moves % 10 {
                    0 => game.handle_input(Direction::Up),
                    3 => game.handle_input(Direction::Right),
                    6 => game.handle_input(Direction::Down),
                    _ => {}
                }

                game.move_snake();
                moves += 1;
            }

            black_box(game);
        })
    });
}

fn benchmark_edge_case_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");

    // Test with snake at different edges
    let edge_scenarios = vec![
        (
            "left_edge",
            Position::new(0, GRID_HEIGHT / 2),
            Direction::Left,
        ),
        (
            "right_edge",
            Position::new(GRID_WIDTH - 1, GRID_HEIGHT / 2),
            Direction::Right,
        ),
        ("top_edge", Position::new(GRID_WIDTH / 2, 0), Direction::Up),
        (
            "bottom_edge",
            Position::new(GRID_WIDTH / 2, GRID_HEIGHT - 1),
            Direction::Down,
        ),
    ];

    for (name, head_pos, direction) in edge_scenarios {
        group.bench_function(name, |b| {
            b.iter(|| {
                let snake = vec![
                    head_pos,
                    Position::new(head_pos.x, head_pos.y + 1),
                    Position::new(head_pos.x, head_pos.y + 2),
                ];
                let mut game = GameState {
                    snake,
                    direction,
                    next_direction: direction,
                    food: Position::new(5, 5),
                    score: 0,
                    high_score: 0,
                    game_over: false,
                    game_speed: 0.2,
                    last_update: 0.0,
                };

                game.move_snake();
                black_box(game);
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_position_creation,
    benchmark_position_validation,
    benchmark_direction_opposite,
    benchmark_position_movement,
    benchmark_game_state_creation,
    benchmark_food_generation,
    benchmark_collision_detection,
    benchmark_snake_movement,
    benchmark_snake_growth,
    benchmark_input_handling,
    benchmark_full_game_simulation,
    benchmark_edge_case_scenarios
);

criterion_main!(benches);
