pub fn day_xa(s: &str) -> String {
    s.to_string()
}

pub fn day_xb(s: &str) -> String {
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_xa_short() {
        assert_eq!(day_xa(SHORT_INPUT), "");
    }
    #[test]
    fn test_day_xa_long() {
        assert_eq!(day_xa(LONG_INPUT), "");
    }
    #[test]
    fn test_day_xb_short() {
        assert_eq!(day_xb(SHORT_INPUT), "");
    }
    #[test]
    fn test_day_xb_long() {
        assert_eq!(day_xb(LONG_INPUT), "");
    }

    const SHORT_INPUT: &str = "";
    const LONG_INPUT: &str = "";
}
