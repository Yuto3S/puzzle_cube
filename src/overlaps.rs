use std::collections::{HashMap, HashSet};

use crate::piece::{are_pieces_overlapping, Piece};

pub fn get_all_overlaps(pieces: Vec<Piece>) -> HashMap<usize, Vec<usize>> {
    let mut overlaps: HashMap<usize, Vec<usize>> = HashMap::new();

    for (index_piece_1, piece_1) in pieces.iter().enumerate() {
        let mut piece_1_overlaps: Vec<usize> = vec![];

        for (index_piece_2, piece_2) in pieces.iter().enumerate().skip(index_piece_1 + 1) {
            if are_pieces_overlapping(*piece_1, *piece_2) {
                piece_1_overlaps.push(index_piece_2);
            }
        }

        piece_1_overlaps.sort();
        overlaps.insert(index_piece_1, piece_1_overlaps);
    }

    return overlaps;
}

pub fn get_all_overlaps_as_pairs(overlaps: HashMap<usize, Vec<usize>>) -> HashMap<String, HashSet<String>> {
    let mut pair_overlaps: HashMap<String, HashSet<String>> = HashMap::new();

    for index_piece_1 in 0..overlaps.len() {
        for index_piece_2 in index_piece_1 + 1..overlaps.len() {
            if !overlaps[&index_piece_1].contains(&index_piece_2) {
                println!("{}{}", index_piece_1, index_piece_2);
            }
        }
    }

    return pair_overlaps;
}