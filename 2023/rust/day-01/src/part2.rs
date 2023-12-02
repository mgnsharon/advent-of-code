use crate::custom_error::AocError;

#[tracing::instrument]
fn get_first_digit(input: &str) -> miette::Result<char, &str> {
    input.chars().find(|c| c.is_numeric()).ok_or("")
}

#[tracing::instrument]
fn process_line(input: &str) -> u64 {
    let mut it = (0..input.len()).filter_map(|idx| {
        let reduced_line = &input[idx..];

        let result = if reduced_line.starts_with("zero") {
            '0'
        } else if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };
        result.to_digit(10)
    });

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
    let output: u64 = input.lines().map(process_line).sum();
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("twone", 21)]
    fn line_test(#[case] line: &str, #[case] expected: u64) {
        assert_eq!(expected, process_line(line))
    }
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
