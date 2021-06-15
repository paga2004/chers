use chers::ParsedMove;
use chers::Position;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut pos = Position::new();
    loop {
        println!("{}", pos);
        print!("Enter move: ");
        loop {
            let mut line = String::new();
            io::stdout().flush()?;
            io::stdin().read_line(&mut line)?; // including '\n'
            match ParsedMove::from_coordinate_notation(line.trim()) {
                Ok(m) => {
                    if pos.make_move(m) {
                        break;
                    }
                    print!("Illegal move. Try again: ");
                }
                Err(e) => print!("Invalid move ({}). Try again: ", e),
            }
        }
    }
}
