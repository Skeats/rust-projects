use std::{error::Error, fmt::{Display, Formatter, Result as FmtResult}, fs};

pub fn install_app(file_path: &String, install_location: &String) -> Result<(), Box<dyn Error>> {
    let file_name = match get_file_name_from_path(&file_path) {
        Some(path) => Ok(path),
        None => Err(InvalidFilenameError {}),
    }?;

    fs::rename(format!("{file_path}/{file_name}"), format!("{install_location}/{file_name}"))?;
    Ok(())
}

fn get_file_name_from_path(file_path: &str) -> Option<String> {
    Some(String::from(file_path.split_terminator("/").last()?))
}

#[derive(Debug)]
struct InvalidFilenameError {}

impl Display for InvalidFilenameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Invalid filename")
    }
}

impl Error for InvalidFilenameError {}

#[cfg(test)]
mod tests {
    use crate::get_file_name_from_path;

    #[test]
    fn get_file() {
        let file_path = "path/to/file.txt";
        assert_eq!(String::from("file.txt"), get_file_name_from_path(file_path).expect("This filepath should return a valid string"))
    }
}