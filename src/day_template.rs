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
    fn test_day_xa() {
        let s = day_xa(SHORT_INPUT);
        assert_eq!(s, "");
    }
    #[test]
    fn test_day_xa_long() {
        let s = day_xa(LONG_INPUT);
        assert_eq!(s, "");
    }
    #[test]
    fn test_day_xb() {
        let s = day_xb(SHORT_INPUT);
        assert_eq!(s, "");
    }
    #[test]
    fn test_day_xb_long() {
        let s = day_xb(LONG_INPUT);
        assert_eq!(s, "");
    }

    const SHORT_INPUT: &str = "";
    const LONG_INPUT: &str = "";
}
