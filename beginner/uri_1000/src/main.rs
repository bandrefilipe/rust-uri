fn main() {
    println!("{}", solution());
}

fn solution() -> &'static str {
    "Hello World!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(), "Hello World!");
    }
}
