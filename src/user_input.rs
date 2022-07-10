use ansi_term::Colour;
use exitcode::DATAERR;
use std::{
    io::{self, Error, Write},
    process::exit,
    str::FromStr,
    vec::Vec,
};

use crate::album::{Album, AlbumString, ReleaseYear, TrackCount};

pub(crate) enum UserChoice {
    Yes,
    No,
}

pub(crate) fn parse_input_args(input: &[String]) -> Album {
    /* Input should be in one of the following formats:
    4 args -> "artist" "name" track_count release_year
    1 arg  -> "artist;;name;;track_count;;release_year" */
    let input_string: String = match input.len() {
        4 => input.join(";;"),
        1 => input[0].to_string(),
        _ => {
            let error = format!("INVALID DATA! {:?}", input);
            println!("{}", Colour::Red.paint(error));
            exit(DATAERR);
        }
    };

    let album_fields: Vec<&str> = input_string.split(";;").map(str::trim).collect();

    let mut errors = Vec::<String>::new();

    let artist: Result<AlbumString, Error> = AlbumString::from_str(album_fields[0]);
    let album_name: Result<AlbumString, Error> = AlbumString::from_str(album_fields[1]);
    let track_count: Result<TrackCount, Error> = TrackCount::from_str(album_fields[2]);
    let release_year: Result<ReleaseYear, Error> = ReleaseYear::from_str(album_fields[3]);

    add_error(&mut errors, &album_name);
    add_error(&mut errors, &artist);
    add_error(&mut errors, &track_count);
    add_error(&mut errors, &release_year);

    if errors.is_empty() {
        return Album::new(
            album_name.unwrap(),
            artist.unwrap(),
            track_count.unwrap(),
            release_year.unwrap(),
        );
    }

    let error: String = format!(
        "Input contained invalid data:\n{}",
        errors
            .into_iter()
            .enumerate()
            .map(|(i, e)| format!("\t{}: {}\n", i, e))
            .collect::<String>()
    );

    println!("{}", Colour::Red.paint(error));

    exit(DATAERR);
}

pub(crate) fn get_user_input() -> Album {
    Album::new(
        get_album_name(),
        get_artist(),
        get_tracks(),
        get_release_year(),
    )
}

pub(crate) fn get_user_choice() -> UserChoice {
    loop {
        // strip out tabs since this is going to be written to a .tsv
        print!("{}", Colour::Fixed(100).paint("[Y]es/[N]o "));
        io::stdout().lock().flush().unwrap();

        // todo: single keypress instead of readline
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim().to_lowercase().as_str() {
            "y" => return UserChoice::Yes,
            "yes"  => return UserChoice::Yes,
            "n" => return UserChoice::No,
            "no" => return UserChoice::No,
            _ => continue,
        }
    }
}

fn add_error<T>(errs: &mut Vec<std::string::String>, result: &Result<T, Error>) {
    if let Err(e) = result {
        errs.push(e.to_string());
    }
}

fn get_artist() -> AlbumString {
    get_input_string("    artist name: ")
}

fn get_album_name() -> AlbumString {
    get_input_string("Enter\n    album name: ")
}

fn get_release_year() -> ReleaseYear {
    loop {
        match ReleaseYear::from_str(&get_input("    album release year: ")) {
            Ok(ry) => return ry,
            Err(err) => {
                print_validation_error(&err);
                continue;
            }
        }
    }
}

fn get_tracks() -> TrackCount {
    loop {
        match TrackCount::from_str(&get_input("    number of tracks: ")) {
            Ok(tc) => return tc,
            Err(err) => {
                print_validation_error(&err);
                continue;
            }
        }
    }
}

fn get_input_string(prompt: &str) -> AlbumString {
    loop {
        // strip out tabs since this is going to be written to a .tsv
        let input: String = get_input(prompt).replace('\t', " ").trim().to_string();

        if input.is_empty() {
            continue;
        }

        return AlbumString(input);
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", Colour::Yellow.paint(prompt));
    io::stdout().lock().flush().unwrap(); // print! won't print to screen until flush() is called

    let mut buf = String::new();

    let _ = io::stdin().read_line(&mut buf).unwrap();

    buf
}

fn print_validation_error(err: &Error) {
    let e = format!("{}", err);
    println!("{}", Colour::Red.paint(e));
}
