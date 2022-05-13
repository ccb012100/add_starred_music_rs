use chrono::Utc;

use std::{
    fs::File,
    io::{self, Write}, path::Path,
};

#[derive(Debug)]
struct Album {
    name: String,
    artist: String,
    tracks: u16,
    release_date: u16,
    date_added: String,
}

impl Album {
    fn new(
        name: String,
        artist: String,
        tracks: u16,
        release_date: u16,
        date_added: String,
    ) -> Self {
        Self {
            artist,
            tracks,
            name,
            release_date,
            date_added,
        }
    }

    fn to_tsv_entry(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}",
            self.name, self.artist, self.tracks, self.release_date, self.date_added
        )
    }
}

fn main() {
    // TODO: make this configurable and/or an input arg
    let mut home: std::path::PathBuf = dirs::home_dir().unwrap();
    home.push("ccb012100/starred_music/starredmusic.tsv");
    let file_path = home.as_path();
    // try to open the file so that any failures happen _before_ we try to get user input
    let mut file: File = open_file(file_path.to_str().unwrap());

    let album = Album::new(
        get_album_name(),
        get_artist(),
        get_tracks(),
        get_release_date(),
        get_date_added(),
    );

    let album_entry = album.to_tsv_entry();
    println!("{:?}", album_entry);
    write_to_file(&mut file, file_path, album_entry);
}

fn open_file(path: &str) -> File {
    File::options()
        .append(true)
        .create(false)
        .open(path)
        .unwrap_or_else(|error| panic!("Failed opening file '{}': {}", path, error))
}

fn write_to_file(file: &mut File, file_path: &Path, data: String) {
    let data = format!("\n{}\n",data);
    let _ = match file.write(data.as_bytes()) {
        Ok(b) => println!(
            "Wrote {} bytes to file {:?}:\n{}",
            b,
            file,
            &data
        ),
        Err(error) => panic!("Failure writing to file '{:?}': {}", file_path, error),
    };
}

fn get_artist() -> String {
    get_input_string("Enter artist name: ")
}

fn get_album_name() -> String {
    get_input_string("Enter album name: ")
}

fn get_release_date() -> u16 {
    let release: u16;

    loop {
        let input: String = get_input("Enter album release year: ").trim().to_string();

        if input.is_empty() {
            continue;
        }

        release = if let Ok(year) = input.parse::<u16>() {
            // 1928 is arbitrary, but should encompass almost anything I'll add
            // TODO: get current year dynamically instead of hardcoding it
            if !(1928..=2022).contains(&year) {
                println!("Input '{}' is outside the range 1928-2022", input);
                continue;
            }

            year
        } else {
            println!("Input '{}' is not a valid number", input);
            continue;
        };

        break;
    }

    release
}

fn get_tracks() -> u16 {
    let tracks: u16;

    loop {
        let input: String = get_input("Enter number of tracks: ").trim().to_string();

        if input.is_empty() {
            continue;
        }

        tracks = if let Ok(t) = input.parse::<u16>() {
            // 750 is arbitrary, but should accomodate any album/compilation
            if t > 750 {
                println!("Input '{}' is greater than the max 750", input);
                continue;
            }

            t
        } else {
            println!("Input '{}' is not a valid number", input);
            continue;
        };

        break;
    }

    tracks
}

// Get local Now formatted as YYYY-MM-DD HH:MM:SS
fn get_date_added() -> String {
    Utc::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

fn get_input_string(prompt: &str) -> String {
    let mut input: String;

    loop {
        // strip out tabs since this is going to be written to a .tsv
        input = get_input(prompt).trim().replace('\t', " ").to_string();

        if input.is_empty() {
            continue;
        }

        break;
    }

    input
}

fn get_input(prompt: &str) -> String {
    // TODO: better separation of prompt and user input (i.e. indentation, colors, etc.)
    println!("{}", prompt);
    let mut buf = String::new();

    let _ = io::stdin()
        .read_line(&mut buf)
        .unwrap_or_else(|error| panic!("Problem reading from stdin: {}", error));

    buf
}
