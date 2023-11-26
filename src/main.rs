mod cell;
mod board;
use crate::board::Board;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game_board = Board::random();
    loop {
        println!("{}", game_board);
        let input = get_input("Type \"exit\" to quit or press \"ENTER\" to continue")?;
        match input.to_lowercase().as_str() {
            "exit" => break,
            _ => game_board.update(),
        }

    }
    Ok(())
}

fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    use std::io::{stdin, stdout, Write};
    
    stdout().write(prompt.as_bytes())?;
    stdout().flush()?;
    
    let mut input: String = String::new();   
    stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}