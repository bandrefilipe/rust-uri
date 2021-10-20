use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("{}", solution(io::stdin()));
}

fn solution<R: Read>(reader: R) -> String {
    // Wraps the reader in a BufReader. The reason for this is so that we can:
    // a. receive a Stdin in the main function, and
    // b. 'mock' the Stdin functionality with a Cursor in tests
    let mut reader: BufReader<R> = BufReader::new(reader);

    // Capture inputs
    let a: i32 = parse_i32(get_input(&mut reader));
    let b: i32 = parse_i32(get_input(&mut reader));

    // Format the message
    format!("X = {}", (a + b))
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
        let cursor: Cursor<&[u8; 5]> = Cursor::new(b"10\n9\n");
        assert_eq!(solution(cursor), "X = 19");
    }

    #[test]
    fn test_solution_b() {
        let cursor: Cursor<&[u8; 6]> = Cursor::new(b"-10\n4\n");
        assert_eq!(solution(cursor), "X = -6");
    }

    #[test]
    fn test_solution_c() {
        let cursor: Cursor<&[u8; 6]> = Cursor::new(b"15\n-7\n");
        assert_eq!(solution(cursor), "X = 8");
    }
}
