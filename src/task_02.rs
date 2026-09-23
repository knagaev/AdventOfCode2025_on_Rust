use std::fs;
use std::io::{self, BufRead, BufReader};

pub fn local_main() -> io::Result<()> {
    const FILE_PATH: &str = "data/input_01.txt";
    let file = fs::File::open(FILE_PATH)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line.trim());
    }

    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let first_line = first_line.trim().to_string();

    Ok(())
}
