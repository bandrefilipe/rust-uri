use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() {
    // `print!`, NOT `println!`, because `solution` already returns a string ended by `\n`
    print!("{}", solution(io::stdin()));
}

fn solution<R: Read>(reader: R) -> String {
    let mut reader: BufReader<R> = BufReader::new(reader);

    // capture the 1st input, representing how many test cases there will be
    let test_cases: u32 = parse_u32(get_input(&mut reader));

    // a growable string to hold the result of this function
    let mut result: String = String::new();

    // loop through each test case
    for _ in 1..=test_cases {

        // how many recursive calls there were
        let mut recursive_calls: u32 = 0;

        let n: u32 = parse_u32(get_input(&mut reader));
        let fib: u32 = fib(n, &mut recursive_calls);

        // push to the result string the result of this test case
        result.push_str(&format!("fib({}) = {} calls = {}\n", n, recursive_calls, fib));
    }

    result
}

fn get_input<R: BufRead>(reader: &mut R) -> String {
    let mut input: String = String::new();
    reader.read_line(&mut input).unwrap();
    input
}

fn parse_u32(string: String) -> u32 {
    string.trim().parse().unwrap()
}

fn fib(n: u32, recursive_calls: &mut u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            *recursive_calls += 2;
            fib(n - 1, recursive_calls) + fib(n - 2, recursive_calls)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_solution() {
        let cursor = Cursor::new(b"2\n5\n4\n");
        assert_eq!(solution(cursor), "fib(5) = 14 calls = 5\nfib(4) = 8 calls = 3\n");
    }
}
