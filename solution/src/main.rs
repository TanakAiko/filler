use std::io::Write;
use std::{fs::OpenOptions, io};

use utils::{get_anfield, get_piece_size, get_player};

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

    let _player = get_player(first_line);

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

        writeln!(file, "input: {}", input).expect("Writing error");

        let trimmed = input.trim();
        if !trimmed.is_empty() {
            lines.push(trimmed.to_string());
        }

        let size = get_piece_size(trimmed.to_string());
        if size.0 != 0 {
            piece_size = size.clone()
        }

        writeln!(file, "lines.len(): {}, total: {}", lines.len(), anfield.1 + 1 + piece_size.1 + 1).expect("Writing error");
        if byte_read == 0
            || piece_size.1 != 0 && lines.len() == (anfield.1 + 1 + piece_size.1 + 1) as usize
        {
            writeln!(file, "lines: {:?}", lines).expect("Writing error");
            println!("9 12");
            //read(lines);
            lines = Vec::new();
        }

    }
}
