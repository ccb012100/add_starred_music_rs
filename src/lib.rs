use std::{env, fs::File};

mod album;
mod file_handling;
mod user_input;

use album::Album;

pub fn add_album() {
    // TODO: make this configurable and/or an input arg
    let file_path: &str = "ccb012100/starred_music/starredmusic.tsv";
    let args: Vec<String> = env::args().collect();

    let (mut file, album): (File, Album) = match args.len() {
        1 => {
            // Attempt to open the file before trying to get user input
            println!("No args supplied by user");
            let file: File = file_handling::open_file(file_path);
            let album = user_input::get_user_input();

            (file, album)
        }
        _ => {
            // parse the user input before attempting to open the file
            println!("Args supplied by user: {:?}", &args[1..]);
            let album = user_input::parse_input_args(&args[1..]);
            let file: File = file_handling::open_file(file_path);

            (file, album)
        }
    };

    file_handling::write_to_file(&mut file, file_path, album.to_tsv_entry());
}
