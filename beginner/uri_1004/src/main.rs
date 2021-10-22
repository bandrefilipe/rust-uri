use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("{}", solution(io::stdin()));
}

fn solution<R: Read>(reader: R) -> String {
    let mut reader: BufReader<R> = BufReader::new(reader);
    let multiplicand: i32 = parse_i32(get_input(&mut reader));
    let multiplier: i32 = parse_i32(get_input(&mut reader));
    let product: i32 = multiplicand * multiplier;
    format!("PROD = {}", product)
}

fn get_input<R: BufRead>(reader: &mut R) -> String {
    let mut input: String = String::new();
    reader.read_line(&mut input).unwrap();
    input
}

fn parse_i32(string: String) -> i32 {
    string.trim().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_solution_a() {
        let cursor: Cursor<&[u8; 4]> = Cursor::new(b"3\n9\n");
        assert_eq!(solution(cursor), "PROD = 27");
    }

    #[test]
    fn test_solution_b() {
        let cursor: Cursor<&[u8; 7]> = Cursor::new(b"-30\n10\n");
        assert_eq!(solution(cursor), "PROD = -300");
    }

    #[test]
    fn test_solution_c() {
        let cursor: Cursor<&[u8; 4]> = Cursor::new(b"0\n9\n");
        assert_eq!(solution(cursor), "PROD = 0");
    }
}
