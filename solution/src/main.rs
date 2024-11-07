use std::io::Write;
use std::{fs::OpenOptions, io};

use utils::{get_anfield, get_coordinate, get_piece_size, get_player};

pub mod utils;

fn main() {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // Réinitialise le fichier à chaque exécution
        .open("output.txt")
        .expect("Erreur lors de l'ouverture du fichier");

    let mut lines = Vec::new();
    let mut piece_size = (0, 0);

    let mut first_line = String::new();
    io::stdin()
        .read_line(&mut first_line)
        .expect("Erreur lors de la lecture");

    writeln!(file, "first_line: {}", first_line).expect("Writing error");

    let player = get_player(first_line);

    let mut second_line = String::new();
    io::stdin()
        .read_line(&mut second_line)
        .expect("Erreur lors de la lecture");
    writeln!(file, "second_line: {}", second_line).expect("Writing error");

    let anfield = get_anfield(second_line);

    loop {
        let mut input = String::new();

        let byte_read = io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture");

        write!(file, "input: {}", input).expect("Writing error");

        let trimmed = input.trim();
        if !trimmed.is_empty() && !trimmed.contains("Anfield") {
            lines.push(trimmed.to_string());
        }

        let size = get_piece_size(trimmed.to_string());
        if size.0 != 0 {
            piece_size = size.clone()
        }

        //writeln!(file, "lines.len(): {}, total: {}", lines.len(), anfield.1 + 1 + piece_size.1 + 1).expect("Writing error");
        if byte_read == 0
            || piece_size.1 != 0 && lines.len() == (anfield.1 + 1 + piece_size.1 + 1) as usize
        {
            //writeln!(file, "lines: {:?}", lines).expect("Writing error");
            get_coordinate(&mut file, lines, player.clone(), anfield, piece_size);
            lines = Vec::new();
            piece_size = (0, 0);
        }
    }
}
