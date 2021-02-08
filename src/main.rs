use position::Position;

mod piece;
mod position;

fn main() {
    let pos = Position::new();
    println!("{}", pos);
}
