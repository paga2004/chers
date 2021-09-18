use chers::Color;
use chers::ParsedMove;
use chers::Position;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut pos = Position::new();
    while !(pos.is_draw() || pos.is_checkmate()) {
        println!("{}", pos);
        if pos.side_to_move() == Color::WHITE {
            loop {
                print!("Enter move: ");
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
        } else {
            println!("Thinking...");
            let m = pos.search(4);
            dbg!(m);
            pos.make_bit_move(m);
        }
    }
    if pos.is_draw() {
        println!("Draw!");
    } else {
        println!("{} won!", pos.side_to_move());
    }

    Ok(())
}
