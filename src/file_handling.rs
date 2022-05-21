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
        .unwrap_or_else(|error| panic!("Failed opening file '{}': {}", file_path, error))
}

pub(crate) fn write_to_file(file: &mut File, file_path: &str, data: String) {
    let data = format!("{}\n", data);

    let _ = match file.write(data.as_bytes()) {
        Ok(b) => println!("Wrote {} bytes to {:?}:\n{}", b, file, &data),
        Err(error) => panic!("Failure writing to file '{:?}': {}", file_path, error),
    };
}
