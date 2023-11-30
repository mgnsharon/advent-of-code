fn parse_elves(s: &str) -> Vec<u32> {
    s.split("\n\n").map(get_elf_total).collect()
}

fn get_elf_total(s: &str) -> u32 {
    s.lines().map(|c| c.parse::<u32>().unwrap()).sum()
}

pub fn day_1a(s: &str) -> String {
    parse_elves(s).iter().max().unwrap().to_string()
}

pub fn day_1b(s: &str) -> String {
    let mut elves = parse_elves(s);

    elves.sort_by(|a, b| b.cmp(a));
    elves.iter().take(3).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_day_1a() {
        let input = fs::read_to_string("./src/day_1/short.txt").unwrap();
        let s = day_1a(&input);
        assert_eq!(s, "24000");
    }
    #[test]
    fn test_day_1a_long() {
        let input = fs::read_to_string("./src/day_1/long.txt").unwrap();
        let s = day_1a(&input);
        assert_eq!(s, "71023");
    }

    #[test]
    fn test_day_1b() {
        let input = fs::read_to_string("./src/day_1/short.txt").unwrap();
        let s = day_1b(&input);
        assert_eq!(s, "45000");
    }

    #[test]
    fn test_day_1b_long() {
        let input = fs::read_to_string("./src/day_1/long.txt").unwrap();
        let s = day_1b(&input);
        println!("{}", s);
        assert_eq!(s, "206289");
    }
}
