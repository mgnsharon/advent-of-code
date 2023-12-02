use crate::custom_error::AocError;

#[tracing::instrument]
fn process_line(input: &str) -> u64 {
    let mut it = input.chars().filter_map(|c| c.to_digit(10));

    let first = it.next().expect("should be a number");

    match it.last() {
        Some(n) => format!("{first}{n}"),
        None => format!("{first}{first}"),
    }
    .parse::<u64>()
    .expect("should be a valid number")
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input.lines().map(process_line).sum::<u64>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn line_test(#[case] line: &str, #[case] expected: u64) {
        assert_eq!(expected, process_line(line))
    }
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
