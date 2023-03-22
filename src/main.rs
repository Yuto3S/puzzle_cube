mod piece;

fn main() {
    println!("Hello, world!");
    let default_pieces = piece::get_all_default_pieces();
    // for piece in default_pieces.iter(){
    //     println!("{}", piece.block_1.x);
    // }

    assert_eq!(true, piece::are_pieces_overlapping(default_pieces[0], default_pieces[2]));
    assert_eq!(false, piece::are_pieces_overlapping(default_pieces[0], default_pieces[21]));
}

