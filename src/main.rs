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
}

fn test_benchmark_given_function() -> () {
    const VEC_LENGTH: usize = 100000;

    benchmarking::warm_up();

    let bench_result = benchmarking::measure_function(|measurer| {
        let mut vec: Vec<usize> = Vec::with_capacity(VEC_LENGTH);

        measurer.measure(|| {
            for i in 0..VEC_LENGTH {
                // WRITE CODE FOR GIVEN FUNCTION HERE
            }
        });

        vec
    })
    .unwrap();

    println!(
        "are_pieces_overlapping {} times takes {:?}!",
        VEC_LENGTH,
        bench_result.elapsed()
    );
}