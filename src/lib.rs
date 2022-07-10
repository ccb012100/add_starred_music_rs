use ansi_term::{ANSIString, ANSIStrings, Colour};
use exitcode::OK;
use std::{env, fs::File, process::exit};

mod album;
mod file_handling;
mod user_input;

use album::Album;

pub fn add_album() {
    if cfg!(windows) {
        // FIXME: why isn't this working on Ubuntu?
        // let _enabled = ansi_term::enable_ansi_support();
    }

    // TODO: make this configurable and/or an input arg
    let file_path: &str = "ccb012100/starred_music/starredmusic.tsv";
    let args: Vec<String> = env::args().collect();

    let (mut file, album): (File, Album) = match args.len() {
        1 => {
            // Attempt to open the file before trying to get user input
            let file: File = file_handling::open_file(file_path);
            let album = user_input::get_user_input();

            (file, album)
        }
        _ => {
            // parse the user input before attempting to open the file
            let s = format!("Args supplied by user: {:?}\n", &args[1..]);
            println!("Args:\t{}\n", Colour::Fixed(126).paint(s));

            let album = user_input::parse_input_args(&args[1..]);
            let file: File = file_handling::open_file(file_path);

            (file, album)
        }
    };

    let strings: &[ANSIString<'static>] = &[
        Colour::Fixed(205).paint(format!("\n{}", album)),
        Colour::Fixed(100).paint("\n\nAdd album to "),
        Colour::Blue.bold().paint(file_path.to_string()),
        Colour::Fixed(100).paint("? "),
    ];

    print!("{}", ANSIStrings(strings));
    // io::stdout().lock().flush().unwrap();

    match user_input::get_user_choice() {
        user_input::UserChoice::Yes => {
            file_handling::write_to_file(&mut file, file_path, album.to_tsv_entry())
        }
        user_input::UserChoice::No => exit(OK),
    }
}
