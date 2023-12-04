use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u32},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::{Cube, Game};

pub fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, parse_game)(input)?;
    Ok((input, games))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), u32)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), parse_cubes))(input)?;
    Ok((input, Game { id, rounds }))
}

pub fn parse_cubes(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), parse_cube)(input)?;
    Ok((input, cubes))
}

pub fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amt, color)) = separated_pair(u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { amt, color }))
}
