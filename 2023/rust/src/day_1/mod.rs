pub fn day_1a(s: &str) -> String {
    s.to_string()
}

pub fn day_1b(s: &str) -> String {
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_day_1a_short() {
        let input = fs::read_to_string("./src/day_1/short.txt").unwrap();
        assert_eq!(day_1a(&input), "1\n2");
    }
    #[test]
    fn test_day_1a_long() {
        let input = fs::read_to_string("./src/day_1/long.txt").unwrap();
        assert_eq!(day_1a(&input), "1\n2\n3\n4\n5\n");
    }
    #[test]
    fn test_day_1b_short() {
        let input = fs::read_to_string("./src/day_1/short.txt").unwrap();
        assert_eq!(day_1b(&input), "1\n2");
    }
    #[test]
    fn test_day_1b_long() {
        let input = fs::read_to_string("./src/day_1/long.txt").unwrap();
        assert_eq!(day_1b(&input), "1\n2\n3\n4\n5\n");
    }
}
