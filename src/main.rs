use std::process::ExitCode;

#[cfg(test)]
mod tests;

#[cfg(doctest)]
mod tests;

fn main() -> ExitCode {
    // let path = Path::new("test.yk");
    // let path_display = path.display();
    //
    // let file = match File::open(path) {
    //     Ok(file) => file,
    //     Err(why) => panic!("Failed to open file {}: {}", path_display, why)
    // };

    // let file_reader = BufReader::new(file);
    // let mut bytes = file_reader.bytes().peekable();

    return ExitCode::from(0);
}
