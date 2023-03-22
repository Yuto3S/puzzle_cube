#[derive(Debug, Copy, Clone)]
pub struct Coordinates {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub block_1: Coordinates,
    pub block_2: Coordinates,
    pub block_3: Coordinates,
    pub block_4: Coordinates,
    pub block_5: Coordinates,
}

const ALL_DEFAULT_PIECES_COORDINATES: [[(i8, i8, i8); 5] ; 24] = [
    [(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 0, 1), (3, 0, 0)],
    [(0, 1, 0), (1, 1, 0), (2, 0, 0), (2, 1, 0), (3, 1, 0)],
    [(0, 0, 1), (1, 0, 1), (2, 0, 1), (2, 0, 0), (3, 0, 1)],
    [(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0), (3, 0, 0)],
    [(0, 0, 0), (1, 0, 0), (1, 0, 1), (2, 0, 0), (3, 0, 0)],
    [(0, 1, 0), (1, 1, 0), (1, 0, 0), (2, 1, 0), (3, 1, 0)],
    [(0, 0, 1), (1, 0, 0), (1, 0, 1), (2, 0, 1), (3, 0, 1)],
    [(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 0, 0), (3, 0, 0)],
    [(0, 0, 0), (0, 1, 0), (0, 2, 0), (0, 2, 1), (0, 3, 0)],
    [(0, 0, 0), (0, 1, 0), (0, 2, 0), (1, 2, 0), (0, 3, 0)],
    [(0, 0, 1), (0, 1, 1), (0, 2, 0), (0, 2, 1), (0, 3, 1)],
    [(0, 2, 0), (1, 0, 0), (1, 1, 0), (1, 2, 0), (1, 3, 0)],
    [(0, 0, 0), (0, 1, 0), (0, 1, 1), (0, 2, 0), (0, 3, 0)],
    [(0, 0, 0), (0, 1, 0), (0, 2, 0), (0, 3, 0), (1, 1, 0)],
    [(0, 1, 0), (0, 0, 1), (0, 1, 1), (0, 2, 1), (0, 3, 1)],
    [(0, 1, 0), (1, 0, 0), (1, 1, 0), (1, 2, 0), (1, 3, 0)],
    [(0, 0, 0), (0, 0, 1), (0, 0, 2), (0, 0, 3), (1, 0, 2)],
    [(0, 0, 2), (0, 1, 0), (0, 1, 1), (0, 1, 2), (0, 1, 3)],
    [(0, 0, 2), (1, 0, 0), (1, 0, 1), (1, 0, 2), (1, 0, 3)],
    [(0, 1, 2), (0, 0, 0), (0, 0, 1), (0, 0, 2), (0, 0, 3)],
    [(1, 0, 1), (0, 0, 0), (0, 0, 1), (0, 0, 2), (0, 0, 3)],
    [(0, 0, 1), (0, 1, 0), (0, 1, 1), (0, 1, 2), (0, 1, 3)],
    [(0, 0, 1), (1, 0, 0), (1, 0, 1), (1, 0, 2), (1, 0, 3)],
    [(0, 1, 1), (0, 0, 0), (0, 0, 1), (0, 0, 2), (0, 0, 3)]
];

pub fn get_all_default_pieces() -> [Piece; 24] {
    let default_pieces: [Piece; 24] = ALL_DEFAULT_PIECES_COORDINATES.iter().map(
        |coordinates| 
        Piece {
            block_1: Coordinates { x: coordinates[0].0, y: coordinates[0].1, z: coordinates[0].2 },
            block_2: Coordinates { x: coordinates[1].0, y: coordinates[1].1, z: coordinates[1].2 },
            block_3: Coordinates { x: coordinates[2].0, y: coordinates[2].1, z: coordinates[2].2 },
            block_4: Coordinates { x: coordinates[3].0, y: coordinates[3].1, z: coordinates[3].2 },
            block_5: Coordinates { x: coordinates[4].0, y: coordinates[4].1, z: coordinates[4].2 }
        }
    ).collect::<Vec<Piece>>().try_into().unwrap();
    
    return default_pieces;
}