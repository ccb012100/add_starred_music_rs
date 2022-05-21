use std::{
    io::{self, Error},
    process::exit,
    str::FromStr,
    vec::Vec,
};

use crate::album::{Album, AlbumString, ReleaseYear, TrackCount};

pub(crate) fn parse_input_args(input: &[String]) -> Album {
    /* Input should be in one of the following formats:
    4 args -> name artist track_count release_year
    1 arg  -> "name;;artist;;track_count;;release_year" */
    let input_string: String = match input.len() {
        4 => input.join(";;"),
        1 => input[0].to_string(),
        _ => {
            println!("INVALID DATA! {:?}", input);
            exit(exitcode::DATAERR);
        }
    };

    let album_fields: Vec<&str> = input_string.split(";;").map(str::trim).collect();
    println!("album_fields: {:?}", album_fields);

    let mut errors = Vec::<String>::new();

    println!("input: {:?}", input);
    let album_name: Result<AlbumString, Error> = AlbumString::from_str(album_fields[0]);
    let artist: Result<AlbumString, Error> = AlbumString::from_str(album_fields[1]);
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

    println!(
        "Input contained invalid data:\n{}",
        errors.into_iter().enumerate().map(|(i, e)| format!("\t{}: {}\n",i, e)).collect::<String>()
    );

    exit(exitcode::DATAERR);
}

fn add_error<T>(errs: &mut Vec<std::string::String>, result: &Result<T, Error>) {
    if let Err(e) = result {
        errs.push(e.to_string());
    }
}

pub(crate) fn get_user_input() -> Album {
    Album::new(
        get_album_name(),
        get_artist(),
        get_tracks(),
        get_release_year(),
    )
}

fn get_artist() -> AlbumString {
    get_input_string("Enter artist name: ")
}

fn get_album_name() -> AlbumString {
    get_input_string("Enter album name: ")
}

fn get_release_year() -> ReleaseYear {
    loop {
        match ReleaseYear::from_str(&get_input("Enter album release year: ")) {
            Ok(ry) => return ry,
            Err(err) => {
                println!("{}", &err);
                continue;
            }
        }
    }
}

fn get_tracks() -> TrackCount {
    loop {
        match TrackCount::from_str(&get_input("Enter number of tracks: ")) {
            Ok(tc) => return tc,
            Err(err) => {
                println!("{}", &err);
                continue;
            }
        }
    }
}

fn get_input_string(prompt: &str) -> AlbumString {
    let mut input: String;

    loop {
        // strip out tabs since this is going to be written to a .tsv
        input = get_input(prompt).replace('\t', " ").trim().to_string();

        if input.is_empty() {
            continue;
        }

        break;
    }

    AlbumString(input)
}

fn get_input(prompt: &str) -> String {
    // TODO: better visual separation of prompt and user input (i.e. indentation, colors, etc.)
    println!("{}", prompt);
    let mut buf = String::new();

    let _ = io::stdin()
        .read_line(&mut buf)
        .unwrap_or_else(|error| panic!("Problem reading from stdin: {}", error));

    buf
}
