mod piece;

fn main() {
    println!("Hello, world!");
    let default_pieces = piece::get_all_default_pieces();
    for piece in default_pieces.iter(){
        println!("{}", piece.block_1.x);
    }
}

