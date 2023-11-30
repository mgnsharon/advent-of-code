use nom::{
    bytes::complete::{is_a, tag},
    character::complete::alpha1,
    sequence::separated_pair,
    IResult,
};

pub fn parse_filename(input: &str) -> IResult<&str, &str> {
    let (input, filename) = is_a("qwertyuiopasdfghjklzxcvbnm.")(input)?;

    Ok((input, filename))
}

pub fn parse_file_listing(input: &str) -> IResult<&str, (u32, &str)> {
    let (input, (size, name)) =
        separated_pair(nom::character::complete::u32, tag(" "), parse_filename)(input)?;
    Ok((input, (size, name)))
}

pub fn parse_directory_listing(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, name))
}

pub fn parse_ls_output(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filename() {
        assert_eq!(parse_filename("test.txt "), Ok((" ", "test.txt")));
        assert_eq!(parse_filename("test "), Ok((" ", "test")));
    }

    #[test]
    fn test_parse_file_listing() {
        assert_eq!(
            parse_file_listing("123 test.txt "),
            Ok((" ", (123, "test.txt")))
        );
        assert_eq!(parse_file_listing("123 test "), Ok((" ", (123, "test"))));
    }

    #[test]
    fn test_parse_directory_listing() {
        assert_eq!(parse_directory_listing("dir test "), Ok((" ", "test")));
    }

    #[test]
    fn test_parse_ls_output() {
        assert_eq!(parse_ls_output("dir test "), Ok((" ", "test")));
    }
}
