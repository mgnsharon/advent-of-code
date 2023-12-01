use crate::custom_error::AocError;

#[tracing::instrument]
fn get_first_digit(input: &str) -> miette::Result<char, &str> {
    input.chars().find(|c| c.is_numeric()).ok_or("")
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: u64 = input
        .lines()
        .map(|line| {
            let first = get_first_digit(line).unwrap();
            let last = get_first_digit(line.chars().rev().collect::<String>().as_str()).unwrap();

            format!("{}{}", first, last).parse::<u64>().unwrap()
        })
        .sum();
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
