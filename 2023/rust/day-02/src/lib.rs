use std::collections::BTreeMap;
pub mod custom_error;

pub mod parsers;
pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Cube<'a> {
    color: &'a str,
    amt: u32,
}

#[derive(Debug)]
pub struct Game<'a> {
    id: u32,
    rounds: Vec<Vec<Cube<'a>>>,
}

impl<'a> Game<'a> {
    fn is_possible(&self, map: &BTreeMap<&str, u32>) -> Option<u32> {
        self.rounds
            .iter()
            .all(|r| {
                r.iter()
                    .all(|c| c.amt <= *map.get(c.color).expect("a valid color"))
            })
            .then_some(self.id)
    }

    fn min_cube_power(&self) -> u32 {
        let mut min_cubes = BTreeMap::from([("red", 0u32), ("green", 0u32), ("blue", 0u32)]);

        self.rounds.iter().for_each(|r| {
            r.iter().for_each(|c| {
                if c.amt > *min_cubes.get(c.color).expect("cube should exist") {
                    min_cubes.entry(c.color).and_modify(|v| *v = c.amt);
                }
            })
        });

        min_cubes.get("red").expect("red should be set")
            * min_cubes.get("green").expect("green should be set")
            * min_cubes.get("blue").expect("blue should be set")
    }
}