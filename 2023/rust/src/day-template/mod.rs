pub fn day_xa(s: &str) -> String {
    s.to_string()
}

pub fn day_xb(s: &str) -> String {
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_day_xa_short() {
        let input = fs::read_to_string("./src/day_x/short.txt").unwrap();
        assert_eq!(day_xa(&input), "");
    }
    #[test]
    fn test_day_xa_long() {
        let input = fs::read_to_string("./src/day_x/long.txt").unwrap();
        assert_eq!(day_xa(&input), "");
    }
    #[test]
    fn test_day_xb_short() {
        let input = fs::read_to_string("./src/day_x/short.txt").unwrap();
        assert_eq!(day_xb(&input), "");
    }
    #[test]
    fn test_day_xb_long() {
        let input = fs::read_to_string("./src/day_x/long.txt").unwrap();
        assert_eq!(day_xb(&input), "");
    }
}
