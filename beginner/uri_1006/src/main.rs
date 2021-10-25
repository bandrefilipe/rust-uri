use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("{}", solution(io::stdin()));
}

fn solution<R: Read>(reader: R) -> String {
    let mut reader: BufReader<R> = BufReader::new(reader);

    let a: f64 = parse_f64(get_input(&mut reader));
    let b: f64 = parse_f64(get_input(&mut reader));
    let c: f64 = parse_f64(get_input(&mut reader));

    format!("MEDIA = {:.1}", average(a, b, c))
}

fn get_input<R: BufRead>(reader: &mut R) -> String {
    let mut input: String = String::new();
    reader.read_line(&mut input).unwrap();
    input
}

fn parse_f64(string: String) -> f64 {
    string.trim().parse().unwrap()
}

fn average(a: f64, b: f64, c: f64) -> f64 {
    const A_WEIGHT: f64 = 2.0;
    let a = a * A_WEIGHT;

    const B_WEIGHT: f64 = 3.0;
    let b = b * B_WEIGHT;

    const C_WEIGHT: f64 = 5.0;
    let c = c * C_WEIGHT;

    (a + b + c) / (A_WEIGHT + B_WEIGHT + C_WEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_solution_a() {
        let cursor = Cursor::new(b"5.0\n6.0\n7.0\n");
        assert_eq!(solution(cursor), "MEDIA = 6.3");
    }

    #[test]
    fn test_solution_b() {
        let cursor = Cursor::new(b"5.0\n10.0\n10.0\n");
        assert_eq!(solution(cursor), "MEDIA = 9.0");
    }

    #[test]
    fn test_solution_c() {
        let cursor = Cursor::new(b"10.0\n10.0\n5.0\n");
        assert_eq!(solution(cursor), "MEDIA = 7.5");
    }
}
