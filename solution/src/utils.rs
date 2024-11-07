use std::fs::File;
use std::io::Write;

pub fn get_coordinate(
    file: &mut File,
    lines: Vec<String>,
    player: (char, char),
    anfield: (i32, i32),
    piece_size: (i32, i32),
) {
    let map = get_map(&lines, anfield);
    let piece = get_piece(&lines, piece_size);
    let all_possibilities = get_all_possible_place(&map, &piece, player);
    let opp_point = get_all_opponent_point(&map, &player);
    let close = find_closest_pair(&all_possibilities, &opp_point);
    writeln!(file, "Map: {:?}", map).expect("Writing error");
    writeln!(file, "Piece: {:?}", piece).expect("Writing error");
    writeln!(file, "Possibility: {:?}", all_possibilities).expect("Writing error");
    writeln!(file, "Opp: {:?}", opp_point).expect("Writing error");
    writeln!(file, "Close: {:?}", close).expect("Writing error");
}

pub fn find_closest_pair(
    possibilities: &[(usize, usize)],
    opponents: &[(usize, usize)],
) -> ((usize, usize), (usize, usize)) {
    let mut min_distance = usize::MAX;
    let mut closest_pair = ((0, 0), (0, 0));

    for &possibility in possibilities {
        for &opponent in opponents {
            // Calcul de la distance de Manhattan
            let distance = (possibility.0 as isize - opponent.0 as isize).abs() as usize
                + (possibility.1 as isize - opponent.1 as isize).abs() as usize;

            if distance < min_distance {
                min_distance = distance;
                closest_pair = (possibility, opponent);
            }
        }
    }

    closest_pair
}

pub fn get_all_opponent_point(map: &Vec<Vec<char>>, player: &(char, char)) -> Vec<(usize, usize)> {
    let mut opponent_points = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != player.0 && cell != player.1 && cell != '.' {
                opponent_points.push((i, j));
            }
        }
    }
    opponent_points
}

pub fn get_all_possible_place(
    map: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
    player: (char, char),
) -> Vec<(usize, usize)> {
    let mut all_possibility = Vec::new();
    for (i, line) in map.iter().enumerate() {
        for (j, _point) in line.iter().enumerate() {
            if can_be_place(&map, (i, j), &piece, &player) {
                all_possibility.push((i, j));
            }
        }
    }
    all_possibility
}

pub fn can_be_place(
    map: &Vec<Vec<char>>,
    coordinate: (usize, usize),
    piece: &Vec<Vec<char>>,
    player: &(char, char),
) -> bool {
    let mut nb_of_cross = 0;
    for (i, line) in piece.iter().enumerate() {
        for (j, point) in line.iter().enumerate() {
            // Ignore les cellules vides de la pièce
            if *point == '.' {
                continue;
            }

            // Vérifie que la position (i, j) dans la carte est bien dans les limites
            if coordinate.0 + i >= map.len() || coordinate.1 + j >= map[0].len() {
                return false; // La pièce dépasse les limites de la carte
            }

            match map[coordinate.0 + i][coordinate.1 + j] {
                c if c == player.0 || c == player.1 => {
                    nb_of_cross += 1;
                }
                '.' => {}
                _ => {
                    return false;
                }
            }
        }
    }

    if nb_of_cross != 1 {
        return false;
    }

    true
}

pub fn get_piece(lines: &Vec<String>, piece_size: (i32, i32)) -> Vec<Vec<char>> {
    let mut piece: Vec<Vec<char>> = Vec::new();
    let start_index = lines.len() - piece_size.1 as usize;

    for i in start_index..lines.len() {
        let line: Vec<char> = lines[i].chars().collect();
        piece.push(line[..piece_size.0 as usize].to_vec());
    }
    piece
}

pub fn get_map(lines: &Vec<String>, anfield: (i32, i32)) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 || i > anfield.1 as usize {
            continue;
        }

        let map_line: Vec<char> = line[4..].chars().collect();
        map.push(map_line);
    }
    map
}

pub fn get_piece_size(line: String) -> (i32, i32) {
    if line.contains("Piece") {
        let words: Vec<&str> = line.split_whitespace().collect();
        let x: i32 = words[1].parse().unwrap();
        let y: i32 = words[2][..words[2].len() - 1].parse().unwrap();
        return (x, y);
    }
    (0, 0)
}

pub fn get_player(line: String) -> (char, char) {
    if line.contains("p2") {
        return ('$', 's');
    }

    ('@', 'a')
}

pub fn get_anfield(line: String) -> (i32, i32) {
    if line.contains("Anfield") {
        let words: Vec<&str> = line.split_whitespace().collect();
        let word1: i32 = words[1].parse().unwrap();
        let word2: i32 = words[2][..words[2].len() - 1].parse().unwrap();
        return (word1, word2);
    }

    (0, 0)
}
