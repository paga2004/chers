use chers::Move;
use chers::Position;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut pos = Position::new();
    loop {
        println!("{}", pos);
        let m = loop {
            print!("Enter move: ");
            let mut line = String::new();
            io::stdout().flush()?;
            io::stdin().read_line(&mut line)?; // including '\n'
            match Move::from_smith_notation(line.trim()) {
                Ok(m) => break m,
                Err(e) => println!("Invalid move ({}). Try again", e),
            }
        };
        pos.make_move(&m);
    }
}
