use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("{}", solution(io::stdin()));
}

const PI: f64 = 3.14159;

fn solution<R: Read>(reader: R) -> String {
    let mut reader: BufReader<R> = BufReader::new(reader);
    let r: f64 = parse_f64(get_input(&mut reader));
    format!("A={:.4}", (PI * r.powf(2.0)))
}

fn get_input<R: BufRead>(reader: &mut R) -> String {
    let mut input: String = String::new();
    reader.read_line(&mut input).unwrap();
    input
}

fn parse_f64(string: String) -> f64 {
    string.trim().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_solution_a() {
        let cursor: Cursor<&[u8; 5]> = Cursor::new(b"2.00\n");
        assert_eq!(solution(cursor), "A=12.5664");
    }

    #[test]
    fn test_solution_b() {
        let cursor: Cursor<&[u8; 7]> = Cursor::new(b"100.64\n");
        assert_eq!(solution(cursor), "A=31819.3103");
    }

    #[test]
    fn test_solution_c() {
        let cursor: Cursor<&[u8; 7]> = Cursor::new(b"150.00\n");
        assert_eq!(solution(cursor), "A=70685.7750");
    }
}
