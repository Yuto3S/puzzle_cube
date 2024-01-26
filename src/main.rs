use crate::piece::Piece;

mod find_solution;
mod overlaps;
mod piece;

fn main() {
    let default_pieces = piece::get_all_default_pieces();

    assert_eq!(
        true,
        piece::are_pieces_overlapping(default_pieces[0], default_pieces[2])
    );
    assert_eq!(
        false,
        piece::are_pieces_overlapping(default_pieces[0], default_pieces[21])
    );

    let mut all_generated_pieces: Vec<Piece> = vec![];

    for default_piece in default_pieces.iter() {
        all_generated_pieces.push(*default_piece);
        all_generated_pieces.append(&mut piece::generate_all_pieces_from_translating(
            *default_piece,
        ));
    }

    assert_eq!(960, all_generated_pieces.len());

    let all_overlaps = overlaps::get_all_overlaps(all_generated_pieces);

    // println!("{} - all_overlaps.len()", all_overlaps.len());
    // for overlap in all_overlaps.iter() {
    //     println!("{:?}", overlap);
    // }
    println!("{:?}", all_overlaps[&0]);

    // find_solution::find_solution_from_overlaps(&all_overlaps);
    overlaps::get_all_overlaps_as_pairs(all_overlaps);
}

fn test_benchmark_given_function() -> () {
    const VEC_LENGTH: usize = 10000;

    benchmarking::warm_up();

    let bench_result = benchmarking::measure_function(|measurer| {
        let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

        measurer.measure(|| for i in 0..VEC_LENGTH {});

        vec
    })
    .unwrap();

    println!(
        "are_pieces_overlapping {} times takes {:?}!",
        VEC_LENGTH,
        bench_result.elapsed()
    );
}
