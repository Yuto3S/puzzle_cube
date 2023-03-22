#[derive(Hash, PartialEq, Ord, Eq, PartialOrd, Debug, Copy, Clone)]
pub struct Coordinates {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(PartialEq, Ord, Eq, PartialOrd, Debug, Copy, Clone)]
pub struct Piece {
    pub block_1: Coordinates,
    pub block_2: Coordinates,
    pub block_3: Coordinates,
    pub block_4: Coordinates,
    pub block_5: Coordinates,
}

pub enum TranslatePieceOver {
    X,
    Y,
    Z,
}

impl Piece {
    pub fn contains(&self, coordinate: &Coordinates) -> bool {
        if &self.block_1 == coordinate
            || &self.block_2 == coordinate
            || &self.block_3 == coordinate
            || &self.block_4 == coordinate
            || &self.block_5 == coordinate
        {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_valid(&self) -> bool {
        return self.block_1.x < 5
            && self.block_1.y < 5
            && self.block_1.z < 5
            && self.block_2.x < 5
            && self.block_2.y < 5
            && self.block_2.z < 5
            && self.block_3.x < 5
            && self.block_3.y < 5
            && self.block_3.z < 5
            && self.block_4.x < 5
            && self.block_4.y < 5
            && self.block_4.z < 5
            && self.block_5.x < 5
            && self.block_5.y < 5
            && self.block_5.z < 5;
    }
}

// Those pieces/coordinates can be visually seen in https://github.com/Yuto3S/puzzle_cube/blob/master/src/assets/24_default.png
const ALL_DEFAULT_PIECES_COORDINATES: [[(u8, u8, u8); 5]; 24] = [
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
    [(0, 1, 1), (0, 0, 0), (0, 0, 1), (0, 0, 2), (0, 0, 3)],
];

pub fn get_all_default_pieces() -> [Piece; 24] {
    let default_pieces: [Piece; 24] = ALL_DEFAULT_PIECES_COORDINATES
        .iter()
        .map(|coordinates| Piece {
            block_1: Coordinates {
                x: coordinates[0].0,
                y: coordinates[0].1,
                z: coordinates[0].2,
            },
            block_2: Coordinates {
                x: coordinates[1].0,
                y: coordinates[1].1,
                z: coordinates[1].2,
            },
            block_3: Coordinates {
                x: coordinates[2].0,
                y: coordinates[2].1,
                z: coordinates[2].2,
            },
            block_4: Coordinates {
                x: coordinates[3].0,
                y: coordinates[3].1,
                z: coordinates[3].2,
            },
            block_5: Coordinates {
                x: coordinates[4].0,
                y: coordinates[4].1,
                z: coordinates[4].2,
            },
        })
        .collect::<Vec<Piece>>()
        .try_into()
        .unwrap();

    return default_pieces;
}

pub fn are_pieces_overlapping(piece_1: Piece, piece_2: Piece) -> bool {
    if piece_1.contains(&piece_2.block_1)
        || piece_1.contains(&piece_2.block_2)
        || piece_1.contains(&piece_2.block_3)
        || piece_1.contains(&piece_2.block_4)
        || piece_1.contains(&piece_2.block_5)
    {
        return true;
    }

    return false;
}

fn maybe_create_new_piece_by_translating_piece_over_(
    piece: Piece,
    translate_over: TranslatePieceOver,
) -> Option<Piece> {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    match translate_over {
        TranslatePieceOver::X => {
            x = 1;
        }
        TranslatePieceOver::Y => {
            y = 1;
        }
        TranslatePieceOver::Z => {
            z = 1;
        }
    }

    let new_piece = Piece {
        block_1: Coordinates {
            x: piece.block_1.x + x,
            y: piece.block_1.y + y,
            z: piece.block_1.z + z,
        },
        block_2: Coordinates {
            x: piece.block_2.x + x,
            y: piece.block_2.y + y,
            z: piece.block_2.z + z,
        },
        block_3: Coordinates {
            x: piece.block_3.x + x,
            y: piece.block_3.y + y,
            z: piece.block_3.z + z,
        },
        block_4: Coordinates {
            x: piece.block_4.x + x,
            y: piece.block_4.y + y,
            z: piece.block_4.z + z,
        },
        block_5: Coordinates {
            x: piece.block_5.x + x,
            y: piece.block_5.y + y,
            z: piece.block_5.z + z,
        },
    };

    if new_piece.is_valid() {
        return Some(new_piece);
    }

    return None;
}

fn maybe_create_new_piece_by_translating_piece_over_x(piece: Piece) -> Option<Piece> {
    return maybe_create_new_piece_by_translating_piece_over_(piece, TranslatePieceOver::X);
}

fn maybe_create_new_piece_by_translating_piece_over_y(piece: Piece) -> Option<Piece> {
    return maybe_create_new_piece_by_translating_piece_over_(piece, TranslatePieceOver::Y);
}

fn maybe_create_new_piece_by_translating_piece_over_z(piece: Piece) -> Option<Piece> {
    return maybe_create_new_piece_by_translating_piece_over_(piece, TranslatePieceOver::Z);
}

fn create_all_new_pieces_by_translating_over_x_y_z(piece: Piece) -> Vec<Piece> {
    let mut translations_over_x_y_z: Vec<Piece> = vec![];

    let new_piece = maybe_create_new_piece_by_translating_piece_over_x(piece);

    if new_piece.is_some() {
        translations_over_x_y_z.push(new_piece.unwrap());
        translations_over_x_y_z.extend(create_all_new_pieces_by_translating_over_x_y_z(
            new_piece.unwrap(),
        ));
        translations_over_x_y_z.extend(create_all_new_pieces_by_translating_over_y_z(
            new_piece.unwrap(),
        ));
        translations_over_x_y_z.extend(create_all_new_pieces_by_translating_over_z(
            new_piece.unwrap(),
        ));
    }

    return translations_over_x_y_z;
}

fn create_all_new_pieces_by_translating_over_y_z(piece: Piece) -> Vec<Piece> {
    let mut translations_over_y_z: Vec<Piece> = vec![];

    let new_piece = maybe_create_new_piece_by_translating_piece_over_y(piece);

    if new_piece.is_some() {
        translations_over_y_z.push(new_piece.unwrap());
        translations_over_y_z.extend(create_all_new_pieces_by_translating_over_y_z(
            new_piece.unwrap(),
        ));
        translations_over_y_z.extend(create_all_new_pieces_by_translating_over_z(
            new_piece.unwrap(),
        ));
    }

    return translations_over_y_z;
}

fn create_all_new_pieces_by_translating_over_z(piece: Piece) -> Vec<Piece> {
    let mut translations_over_z: Vec<Piece> = vec![];

    let new_piece = maybe_create_new_piece_by_translating_piece_over_z(piece);

    if new_piece.is_some() {
        translations_over_z.push(new_piece.unwrap());
        translations_over_z.extend(create_all_new_pieces_by_translating_over_z(
            new_piece.unwrap(),
        ));
    }

    return translations_over_z;
}

pub fn generate_all_pieces_from_translating(piece: Piece) -> Vec<Piece> {
    let mut all_pieces_by_translation: Vec<Piece> = vec![];

    all_pieces_by_translation.extend(create_all_new_pieces_by_translating_over_x_y_z(piece));
    all_pieces_by_translation.extend(create_all_new_pieces_by_translating_over_y_z(piece));
    all_pieces_by_translation.extend(create_all_new_pieces_by_translating_over_z(piece));

    return all_pieces_by_translation;
}
