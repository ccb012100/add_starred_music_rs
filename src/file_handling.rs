use ansi_term::{ANSIString, ANSIStrings, Colour};
use std::{fs::File, io::Write};

fn get_full_file_path(relative_file_path: &str) -> String {
    let mut home: std::path::PathBuf = dirs::home_dir().unwrap();
    home.push(relative_file_path);
    String::from(home.as_path().to_str().unwrap())
}

pub(crate) fn open_file(file_path: &str) -> File {
    let full_file_path: String = get_full_file_path(file_path);

    File::options()
        .append(true)
        .create(false)
        .open(full_file_path)
        .unwrap()
}

pub(crate) fn write_to_file(file: &mut File, file_path: &str, data: String) {
    let data = format!("{}\n", data);

    let _ = match file.write(data.as_bytes()) {
        Ok(b) => {
            let strings: &[ANSIString<'static>] = &[
                Colour::Green.paint("\nWrote "),
                Colour::Fixed(134).paint(format!("{}", b)),
                Colour::Green.paint(" bytes\n    destination=<"),
                Colour::Fixed(208).paint(format!("{:?}", file)),
                Colour::Green.paint(">\n    data=<"),
                Colour::Fixed(199).paint((&data).to_string()),
                Colour::Green.paint(">"),
            ];

            println!("{}", ANSIStrings(strings));
        }
        Err(error) => panic!("Failure writing to '{:?}': {}", file_path, error),
    };
}
