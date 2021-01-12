use crate::parser::Parser;
use crate::music::Music;

mod note;
mod parser;
mod music;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("{} <INPUT_SHEET_FILE> <OUTPUT_WAV_FILE>", args[0]);
        std::process::exit(1);
    }
    let sheet_file = &args[1];
    let filename = &args[2];

    let content = std::fs::read_to_string(sheet_file).unwrap();

    let mut parse = parser::Parser::new(&content);
    let mut sheet_music = parse.parse();
    sheet_music.add_into_blank_note(0.05);

    let mut music = Music::from_sheet_music(sheet_music);
    music.write_to_file(filename);
}
