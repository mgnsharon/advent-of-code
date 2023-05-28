use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{self, alpha1, digit1, multispace1, newline, u32},
        streaming::space1,
    },
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    IResult, ToUsize,
};

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, parsed_value) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match parsed_value {
        "   " => None,
        value => Some(value.chars().next().unwrap()),
    };
    Ok((input, result))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let lines = separated_list1(tag(" "), parse_crate)(input)?;
    Ok(lines)
}

fn parse_crates(input: &str) -> IResult<&str, Vec<Stack>> {
    let (input, horizontal_crates) = separated_list1(newline, parse_line)(input)?;
    let (input, _) = newline(input)?;
    let (input, stack_numbers) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let mut stacks = stack_numbers
        .iter()
        .map(|_| Stack { crates: vec![] })
        .collect::<Vec<Stack>>();
    horizontal_crates.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(idx, item)| {
            if let Some(c) = item {
                if let Some(stack) = stacks.get_mut(idx) {
                    stack.crates.push(*c);
                }
            }
        })
    });

    Ok((input, stacks))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, qty) = u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, src) = u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, dst) = u32(input)?;
    Ok((
        input,
        Move {
            qty: qty.to_usize(),
            src: src.to_usize(),
            dst: dst.to_usize(),
        },
    ))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, moves) = separated_list1(newline, parse_move)(input)?;
    Ok((input, moves))
}

#[derive(Debug, Clone)]
struct Stack {
    pub crates: Vec<char>,
}

//impl FromIterator for Stack {}

impl Stack {
    pub fn move_to(&mut self, dst: &mut Self, qty: usize) {
        self.crates
            .drain(0..qty)
            .for_each(|c| dst.crates.insert(0, c))
    }
    pub fn move_multiple_to(&mut self, dst: &mut Self, qty: usize) {
        self.crates
            .drain(0..qty)
            .rev()
            .for_each(|c| dst.crates.insert(0, c))
    }
}

#[derive(Debug, Clone)]
struct Move {
    qty: usize,
    src: usize,
    dst: usize,
}

fn parse_input(input: &str) -> String {
    let (input, mut stacks) = parse_crates(input).unwrap();
    let (_, moves) = parse_moves(input).unwrap();
    moves.iter().for_each(|m| {
        let src_idx = m.src - 1;
        let dst_idx = m.dst - 1;
        if let Ok([src, dst]) = stacks.get_many_mut([src_idx, dst_idx]) {
            src.move_to(dst, m.qty);
        }
    });

    get_top_crates(&stacks)
}

fn parse_input_multi(input: &str) -> String {
    let (input, mut stacks) = parse_crates(input).unwrap();
    let (_, moves) = parse_moves(input).unwrap();
    moves.iter().for_each(|m| {
        let src_idx = m.src - 1;
        let dst_idx = m.dst - 1;
        if let Ok([src, dst]) = stacks.get_many_mut([src_idx, dst_idx]) {
            src.move_multiple_to(dst, m.qty);
        }
    });

    get_top_crates(&stacks)
}

fn get_top_crates(stacks: &[Stack]) -> String {
    stacks.iter().fold("".to_string(), |s, stack| {
        if let Some(c) = stack.crates.first() {
            format!("{s}{c}")
        } else {
            s
        }
    })
}

pub fn day_5a(input: &str) -> String {
    parse_input(input)
}

pub fn day_5b(input: &str) -> String {
    parse_input_multi(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_5a_short() {
        let s = day_5a(SHORT_INPUT);
        assert_eq!(s, "CMZ");
    }
    #[test]
    fn test_day_5a_long() {
        let s = day_5a(LONG_INPUT);
        assert_eq!(s, "SPFMVDTZT");
    }
    #[test]
    fn test_day_5b() {
        let s = day_5b(SHORT_INPUT);
        assert_eq!(s, "MCD");
    }
    #[test]
    fn test_day_5b_long() {
        let s = day_5b(LONG_INPUT);
        assert_eq!(s, "ZFSJBPRFP");
    }

    const SHORT_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    const LONG_INPUT: &str = "    [H]         [D]     [P]
[W] [B]         [C] [Z] [D]
[T] [J]     [T] [J] [D] [J]
[H] [Z]     [H] [H] [W] [S]     [M]
[P] [F] [R] [P] [Z] [F] [W]     [F]
[J] [V] [T] [N] [F] [G] [Z] [S] [S]
[C] [R] [P] [S] [V] [M] [V] [D] [Z]
[F] [G] [H] [Z] [N] [P] [M] [N] [D]
 1   2   3   4   5   6   7   8   9

move 2 from 8 to 2
move 3 from 9 to 2
move 1 from 3 to 8
move 5 from 1 to 7
move 2 from 9 to 2
move 8 from 2 to 4
move 6 from 7 to 2
move 2 from 1 to 7
move 4 from 5 to 9
move 4 from 5 to 6
move 1 from 8 to 3
move 1 from 8 to 5
move 2 from 9 to 8
move 8 from 6 to 4
move 4 from 3 to 6
move 10 from 2 to 3
move 1 from 5 to 1
move 1 from 7 to 4
move 2 from 9 to 8
move 18 from 4 to 8
move 1 from 1 to 6
move 4 from 7 to 3
move 12 from 8 to 4
move 4 from 7 to 9
move 5 from 6 to 9
move 2 from 2 to 7
move 3 from 9 to 5
move 3 from 5 to 9
move 1 from 2 to 8
move 10 from 3 to 1
move 2 from 7 to 8
move 10 from 1 to 9
move 1 from 3 to 5
move 16 from 9 to 8
move 1 from 3 to 2
move 3 from 8 to 3
move 1 from 5 to 9
move 3 from 6 to 7
move 2 from 7 to 2
move 1 from 3 to 8
move 5 from 4 to 1
move 4 from 9 to 5
move 2 from 2 to 5
move 2 from 1 to 9
move 23 from 8 to 4
move 6 from 5 to 2
move 5 from 2 to 6
move 1 from 9 to 6
move 2 from 2 to 4
move 35 from 4 to 9
move 1 from 6 to 1
move 2 from 8 to 7
move 1 from 6 to 8
move 3 from 1 to 7
move 1 from 7 to 1
move 3 from 6 to 2
move 4 from 3 to 7
move 6 from 7 to 9
move 1 from 6 to 9
move 1 from 8 to 1
move 2 from 2 to 9
move 2 from 8 to 2
move 3 from 7 to 3
move 2 from 1 to 9
move 5 from 9 to 3
move 1 from 4 to 2
move 1 from 1 to 4
move 7 from 3 to 9
move 1 from 3 to 4
move 2 from 4 to 7
move 24 from 9 to 4
move 12 from 9 to 3
move 1 from 3 to 1
move 1 from 1 to 2
move 2 from 2 to 6
move 1 from 6 to 5
move 1 from 6 to 8
move 3 from 2 to 4
move 1 from 7 to 4
move 1 from 5 to 3
move 1 from 9 to 8
move 23 from 4 to 8
move 17 from 8 to 5
move 12 from 9 to 8
move 10 from 8 to 7
move 1 from 8 to 6
move 5 from 4 to 3
move 3 from 5 to 1
move 3 from 1 to 6
move 6 from 5 to 4
move 10 from 3 to 1
move 9 from 1 to 7
move 2 from 4 to 9
move 1 from 1 to 6
move 4 from 8 to 1
move 4 from 3 to 7
move 4 from 6 to 5
move 1 from 9 to 6
move 1 from 9 to 2
move 1 from 1 to 7
move 1 from 2 to 7
move 3 from 1 to 7
move 9 from 5 to 9
move 7 from 9 to 7
move 2 from 9 to 1
move 3 from 5 to 9
move 3 from 4 to 8
move 1 from 1 to 2
move 1 from 2 to 6
move 1 from 1 to 6
move 5 from 8 to 7
move 1 from 8 to 1
move 1 from 3 to 9
move 1 from 1 to 6
move 2 from 9 to 5
move 2 from 3 to 9
move 4 from 6 to 3
move 1 from 9 to 4
move 2 from 4 to 8
move 1 from 4 to 8
move 1 from 9 to 5
move 1 from 6 to 8
move 23 from 7 to 8
move 27 from 8 to 2
move 2 from 8 to 1
move 23 from 2 to 6
move 3 from 5 to 3
move 4 from 2 to 5
move 2 from 3 to 1
move 2 from 9 to 3
move 4 from 1 to 4
move 13 from 7 to 9
move 1 from 5 to 6
move 2 from 5 to 9
move 1 from 5 to 3
move 3 from 9 to 3
move 5 from 9 to 5
move 2 from 4 to 2
move 1 from 4 to 9
move 11 from 6 to 9
move 9 from 6 to 1
move 17 from 9 to 5
move 3 from 7 to 4
move 3 from 6 to 3
move 14 from 5 to 2
move 5 from 3 to 1
move 2 from 9 to 4
move 2 from 3 to 8
move 5 from 5 to 9
move 2 from 5 to 4
move 7 from 1 to 8
move 2 from 9 to 5
move 3 from 9 to 8
move 8 from 4 to 2
move 2 from 7 to 8
move 10 from 2 to 9
move 10 from 2 to 6
move 8 from 9 to 7
move 2 from 3 to 9
move 3 from 9 to 8
move 5 from 3 to 9
move 7 from 7 to 9
move 3 from 2 to 9
move 10 from 8 to 5
move 1 from 7 to 6
move 1 from 2 to 3
move 4 from 1 to 6
move 2 from 8 to 4
move 1 from 4 to 6
move 2 from 6 to 3
move 2 from 3 to 1
move 1 from 4 to 9
move 4 from 1 to 5
move 2 from 5 to 2
move 2 from 8 to 4
move 1 from 3 to 5
move 3 from 5 to 7
move 2 from 2 to 9
move 3 from 7 to 6
move 3 from 8 to 5
move 10 from 5 to 7
move 3 from 6 to 4
move 11 from 6 to 1
move 3 from 6 to 2
move 12 from 1 to 3
move 1 from 7 to 5
move 9 from 7 to 3
move 5 from 5 to 1
move 4 from 4 to 6
move 2 from 1 to 7
move 1 from 2 to 6
move 2 from 7 to 8
move 1 from 2 to 4
move 1 from 9 to 5
move 3 from 6 to 7
move 1 from 5 to 2
move 9 from 9 to 5
move 1 from 2 to 8
move 1 from 4 to 8
move 1 from 1 to 8
move 1 from 4 to 2
move 1 from 7 to 2
move 1 from 6 to 2
move 1 from 6 to 8
move 6 from 9 to 6
move 1 from 3 to 4
move 9 from 3 to 5
move 1 from 1 to 3
move 2 from 2 to 6
move 1 from 3 to 5
move 14 from 5 to 1
move 1 from 2 to 6
move 5 from 6 to 4
move 3 from 8 to 2
move 5 from 6 to 1
move 5 from 4 to 6
move 1 from 7 to 1
move 3 from 9 to 3
move 7 from 5 to 7
move 1 from 4 to 6
move 2 from 7 to 5
move 3 from 6 to 1
move 3 from 8 to 1
move 14 from 3 to 4
move 8 from 4 to 2
move 1 from 6 to 1
move 15 from 1 to 6
move 7 from 1 to 6
move 6 from 1 to 3
move 3 from 3 to 1
move 2 from 4 to 5
move 1 from 4 to 2
move 19 from 6 to 8
move 2 from 1 to 8
move 4 from 5 to 4
move 7 from 8 to 2
move 2 from 3 to 1
move 13 from 8 to 6
move 4 from 4 to 9
move 2 from 4 to 8
move 2 from 1 to 6
move 1 from 3 to 5
move 19 from 2 to 3
move 13 from 3 to 1
move 1 from 4 to 9
move 1 from 2 to 8
move 3 from 7 to 1
move 14 from 6 to 9
move 2 from 6 to 4
move 18 from 9 to 4
move 3 from 7 to 2
move 15 from 1 to 4
move 2 from 1 to 8
move 5 from 3 to 1
move 1 from 3 to 6
move 5 from 8 to 9
move 3 from 9 to 5
move 1 from 9 to 5
move 1 from 8 to 9
move 1 from 6 to 2
move 3 from 9 to 4
move 2 from 6 to 7
move 30 from 4 to 6
move 22 from 6 to 9
move 6 from 9 to 4
move 4 from 6 to 7
move 1 from 1 to 6
move 1 from 9 to 8
move 1 from 7 to 6
move 3 from 5 to 3
move 5 from 6 to 5
move 2 from 7 to 9
move 4 from 1 to 5
move 1 from 6 to 4
move 1 from 8 to 7
move 2 from 6 to 4
move 17 from 9 to 8
move 2 from 2 to 7
move 2 from 3 to 1
move 8 from 4 to 8
move 1 from 3 to 8
move 8 from 4 to 2
move 2 from 1 to 2
move 1 from 4 to 6
move 4 from 7 to 1
move 1 from 6 to 8
move 19 from 8 to 3
move 5 from 5 to 1
move 5 from 5 to 9
move 2 from 9 to 3
move 6 from 1 to 9
move 1 from 7 to 5
move 1 from 7 to 4
move 2 from 5 to 7
move 2 from 2 to 4
move 4 from 9 to 8
move 12 from 8 to 7
move 2 from 1 to 9
move 1 from 7 to 4
move 4 from 4 to 5
move 3 from 9 to 3
move 9 from 2 to 6
move 2 from 7 to 5
move 1 from 1 to 9
move 5 from 9 to 7
move 9 from 6 to 2
move 6 from 2 to 8
move 21 from 3 to 2
move 12 from 2 to 9
move 3 from 5 to 9
move 3 from 3 to 8
move 5 from 9 to 6
move 13 from 2 to 3
move 3 from 6 to 2
move 10 from 9 to 8
move 6 from 3 to 1
move 3 from 2 to 9
move 2 from 6 to 7
move 5 from 3 to 9
move 4 from 1 to 9
move 3 from 8 to 5
move 1 from 1 to 7
move 6 from 5 to 7
move 12 from 9 to 7
move 1 from 1 to 8
move 11 from 8 to 5
move 9 from 5 to 7
move 1 from 3 to 1
move 4 from 8 to 7
move 1 from 1 to 7
move 2 from 8 to 3
move 42 from 7 to 4
move 3 from 7 to 9
move 4 from 7 to 5
move 1 from 7 to 8
move 1 from 8 to 5
move 1 from 7 to 5
move 1 from 3 to 4
move 1 from 3 to 9
move 1 from 9 to 6
move 1 from 6 to 4
move 1 from 3 to 5
move 3 from 9 to 2
move 16 from 4 to 8
move 3 from 2 to 4
move 1 from 5 to 4
move 30 from 4 to 6
move 15 from 8 to 3
move 2 from 4 to 5
move 1 from 8 to 7
move 13 from 3 to 6
move 1 from 7 to 8
move 1 from 3 to 8
move 1 from 3 to 8
move 4 from 5 to 2
move 6 from 5 to 2
move 2 from 8 to 6
move 43 from 6 to 2
move 1 from 6 to 1
move 18 from 2 to 4
move 24 from 2 to 6
move 19 from 6 to 3
move 4 from 6 to 3
move 2 from 6 to 3
move 3 from 3 to 2
move 1 from 1 to 3
move 23 from 3 to 6
move 12 from 4 to 3
move 7 from 3 to 9
move 13 from 2 to 9
move 1 from 8 to 4
move 4 from 3 to 8
move 6 from 4 to 2
move 10 from 9 to 3
move 6 from 2 to 9
move 8 from 3 to 5
move 3 from 5 to 3
move 13 from 6 to 5
move 4 from 3 to 9
move 1 from 4 to 2
move 4 from 8 to 3
move 1 from 2 to 5
move 14 from 9 to 5
move 2 from 5 to 4
move 2 from 4 to 3
move 1 from 9 to 5
move 4 from 6 to 1
move 1 from 6 to 2
move 6 from 3 to 2
move 5 from 6 to 8
move 2 from 3 to 7
move 1 from 8 to 1
move 25 from 5 to 7
move 3 from 7 to 9
move 5 from 2 to 9
move 12 from 9 to 8
move 3 from 1 to 6
move 16 from 8 to 2
move 1 from 9 to 2
move 1 from 6 to 2
move 1 from 1 to 3
move 21 from 7 to 3
move 2 from 7 to 1
move 1 from 7 to 8
move 2 from 2 to 1
move 2 from 6 to 3
move 18 from 2 to 9
move 2 from 5 to 1
move 1 from 2 to 1
move 3 from 5 to 2
move 13 from 9 to 1
move 3 from 9 to 2
move 1 from 8 to 7
move 3 from 2 to 6
move 2 from 5 to 1
move 17 from 3 to 8
move 3 from 3 to 8
move 2 from 9 to 1
move 1 from 7 to 5
move 1 from 5 to 3
move 2 from 6 to 4
move 1 from 6 to 1
move 15 from 8 to 2
move 2 from 3 to 6
move 1 from 8 to 5
move 2 from 6 to 8
move 13 from 2 to 9
move 4 from 9 to 8
move 9 from 8 to 9
move 3 from 3 to 4
move 4 from 9 to 7
move 1 from 8 to 6
move 1 from 7 to 5
move 2 from 5 to 1
move 1 from 6 to 3
move 4 from 4 to 5
move 1 from 4 to 6
move 1 from 3 to 7
move 1 from 5 to 6
move 2 from 7 to 2
move 4 from 2 to 3
move 3 from 2 to 7
move 1 from 3 to 6
move 1 from 9 to 6
move 2 from 5 to 2
move 3 from 9 to 5
move 1 from 6 to 1
move 3 from 5 to 4
move 12 from 1 to 2
move 2 from 2 to 4
move 2 from 7 to 8
move 2 from 3 to 9
move 1 from 4 to 7
move 1 from 5 to 2
move 1 from 8 to 3
move 2 from 3 to 6
move 7 from 2 to 8
move 3 from 4 to 1
move 7 from 8 to 5
move 7 from 9 to 2
move 1 from 4 to 5
move 3 from 7 to 6
move 5 from 6 to 9
move 6 from 9 to 5
move 4 from 9 to 6
move 1 from 8 to 5
move 1 from 7 to 4
move 1 from 4 to 2
move 2 from 2 to 9
move 2 from 9 to 2
move 11 from 5 to 3
move 2 from 5 to 2
move 1 from 2 to 9
move 4 from 6 to 9
move 1 from 2 to 9
move 4 from 3 to 7
move 3 from 6 to 4
move 1 from 5 to 7
move 18 from 1 to 3
move 11 from 3 to 2
move 1 from 7 to 9
move 1 from 5 to 9
move 14 from 3 to 6
move 15 from 2 to 4
move 5 from 2 to 5
move 1 from 2 to 5
move 1 from 1 to 9
move 8 from 4 to 1
move 5 from 5 to 9
move 9 from 4 to 9
move 4 from 7 to 4
move 5 from 4 to 8
move 2 from 9 to 6
move 8 from 1 to 8
move 1 from 5 to 3
move 1 from 3 to 4
move 1 from 1 to 8
move 13 from 6 to 3
move 9 from 9 to 5
move 1 from 2 to 8
move 8 from 5 to 1
move 1 from 2 to 7";
}
