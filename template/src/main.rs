fn main() {
    println!("{}", solution());
}

fn solution() -> &'static str {
    "Example"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(), "Example");
    }
}
