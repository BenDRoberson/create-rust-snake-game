use create_rust_snake_game::run_game;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_game()?;
    Ok(())
}
