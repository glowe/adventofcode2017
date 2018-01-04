use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::process;

pub fn read_file(path: &str) -> Result<String> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn get_path_or_exit() -> String {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <filepath>", args[0]);
        process::exit(1);
    }
    args.remove(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_file_path() {
        let result = read_file("bad_file_path.txt");
        assert!(result.is_err());
    }

    #[test]
    fn good_file_path() {
        let result = read_file("ok.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::from("ok\n"));
    }
}
