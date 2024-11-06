pub fn read(lines: Vec<String>) {
    println!("lines: {},", lines.len());
    for (_, line) in lines.iter().enumerate() {
        let _ = line;
    }
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

pub fn get_player(line: String) -> (String, String) {
    if line.contains("p2") {
        return ("$".to_string(), "s".to_string());
    }

    ("@".to_string(), "a".to_string())
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
