use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

// Game constants
const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 15;
const CELL_SIZE: f32 = 30.0;

// Direction enum for snake movement
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// useful so the snake can't reverse into itself
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// Position struct for grid coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Move position in a given direction
    fn move_in_direction(&self, direction: Direction) -> Self {
        match direction {
            // note coordinate system starts at top left and increases down/right in the grid
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }

    // Check if position is within bounds
    fn is_valid(&self) -> bool {
        self.x >= 0 && self.x < GRID_WIDTH && self.y >= 0 && self.y < GRID_HEIGHT
    }
}

// Game state struct - track all the game state
struct GameState {
    snake: Vec<Position>,
    direction: Direction,
    next_direction: Direction,
    food: Position,
    score: u32,
    game_over: bool,
    game_speed: f64, // Time between moves in seconds
    last_update: f64,
}

impl GameState {
    fn new() -> Self {
        // Initialize snake in the center, moving right
        // the snake is a vector of positions, have to consider how I update this if the snake grows!
        let initial_snake: Vec<Position> = vec![
            Position::new(GRID_WIDTH / 2, GRID_HEIGHT / 2),
            Position::new(GRID_WIDTH / 2 - 1, GRID_HEIGHT / 2),
            Position::new(GRID_WIDTH / 2 - 2, GRID_HEIGHT / 2),
        ];

        Self {
            snake: initial_snake.clone(),
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: Self::generate_food_position(&initial_snake),
            score: 0,
            game_over: false,
            game_speed: 0.2, // Start with 5 moves per second
            last_update: 0.0,
        }
    }

    // Generate a random food position that doesn't overlap with snake
    fn generate_food_position(snake: &[Position]) -> Position {
        let mut rng = rand::thread_rng();
        loop {
            let food: Position = Position::new(
                rng.gen_range(0..GRID_WIDTH),
                rng.gen_range(0..GRID_HEIGHT),
            );
            
            // Make sure food doesn't spawn on snake
            if !snake.contains(&food) {
                return food;
            }
        }
    }

    // Update game state (called every frame)
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_over {
            return Ok(());
        }

        let current_time: f64 = ctx.time.time_since_start().as_secs_f64();
        
        // Only move snake if enough time has passed
        if current_time - self.last_update >= self.game_speed {
            self.direction = self.next_direction;
            self.move_snake();
            self.last_update = current_time;
        }

        Ok(())
    }

    // Move the snek
    fn move_snake(&mut self) {
        let head: Position = self.snake[0];
        let new_head: Position = head.move_in_direction(self.direction);

        // Check for collisions
        if !new_head.is_valid() || self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        // Update head location
        self.snake.insert(0, new_head);

        // Check if food was chomped
        if new_head == self.food {
            self.score += 10;
            self.food = Self::generate_food_position(&self.snake);
            
            // Increase game speed
            self.game_speed = (self.game_speed * 0.95).max(0.1);
        } else {
            // Remove tail if the snake is still hungry
            self.snake.pop();
        }
    }

    // Handle input to change direction
    fn handle_input(&mut self, direction: Direction) {
        // Prevent snake from reversing into itself
        if direction != self.direction.opposite() {
            self.next_direction = direction;
        }
    }

    // Draw the game
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Draw snake
        for segment in &self.snake {
            let rect = Rect::new(
                segment.x as f32 * CELL_SIZE,
                segment.y as f32 * CELL_SIZE,
                CELL_SIZE - 2.0,
                CELL_SIZE - 2.0,
            );
            let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?;
            canvas.draw(&mesh, graphics::DrawParam::default());
        }

        // Draw food
        let food_rect = Rect::new(
            self.food.x as f32 * CELL_SIZE,
            self.food.y as f32 * CELL_SIZE,
            CELL_SIZE - 2.0,
            CELL_SIZE - 2.0,
        );
        let food_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), food_rect, Color::RED)?;
        canvas.draw(&food_mesh, graphics::DrawParam::default());

        // Draw score
        let score_text = graphics::Text::new(format!("Score: {}", self.score));
        canvas.draw(&score_text, graphics::DrawParam::default());

        canvas.finish(ctx)?;
        Ok(())
    }
}

// Implement EventHandler trait for ggez. Required for event::run.
impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        key_input: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        if let Some(keycode) = key_input.keycode {
            match keycode {
                KeyCode::Up | KeyCode::W => {
                    self.handle_input(Direction::Up);
                }
                KeyCode::Down | KeyCode::S => {
                    self.handle_input(Direction::Down);
                }
                KeyCode::Left | KeyCode::A => {
                    self.handle_input(Direction::Left);
                }
                KeyCode::Right | KeyCode::D => {
                    self.handle_input(Direction::Right);
                }
                KeyCode::R => {
                    // Reset game
                    *self = GameState::new();
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    // Create ggez context
    let (ctx, event_loop) = ContextBuilder::new("snake_game", "ben!")
        .window_setup(ggez::conf::WindowSetup::default().title("Super Sick Snake Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            GRID_WIDTH as f32 * CELL_SIZE,
            GRID_HEIGHT as f32 * CELL_SIZE + 50.0, // Extra space for score
        ))
        .build()?;

    // Create game state
    let game_state = GameState::new();

    // Run the game
    event::run(ctx, event_loop, game_state)
}
