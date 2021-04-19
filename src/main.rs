use chers::Move;
use chers::Position;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut pos = Position::new();
    loop {
        let legal_moves = pos.generate_legal_moves();
        println!("{}", pos);
        print!("Enter move: ");
        let m = loop {
            let mut line = String::new();
            io::stdout().flush()?;
            io::stdin().read_line(&mut line)?; // including '\n'
            match Move::from_coordinate_notation(line.trim()) {
                Ok(m) => {
                    if legal_moves.contains(&m) {
                        break m;
                    }
                    print!("Illegal move. Try again: ");
                }
                Err(e) => print!("Invalid move ({}). Try again: ", e),
            }
        };
        pos.make_move(&m);
    }
}
