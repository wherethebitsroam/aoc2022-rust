use std::error::Error;

use crate::util;

static TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    fn new(ore: usize, clay: usize, obsidian: usize, geode: usize) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

struct Blueprint {
    id: usize,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

impl From<&str> for Blueprint {
    fn from(s: &str) -> Self {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();

        Self {
            id: parts[1].strip_suffix(':').unwrap().parse().unwrap(),
            ore_robot: Resources::new(parts[6].parse().unwrap(), 0, 0, 0),
            clay_robot: Resources::new(parts[12].parse().unwrap(), 0, 0, 0),
            obsidian_robot: Resources::new(
                parts[18].parse().unwrap(),
                parts[21].parse().unwrap(),
                0,
                0,
            ),
            geode_robot: Resources::new(
                parts[27].parse().unwrap(),
                0,
                parts[30].parse().unwrap(),
                0,
            ),
        }
    }
}

impl Blueprint {
    // Blueprint 1:
    // Each ore robot costs 4 ore.
    // Each clay robot costs 2 ore.
    // Each obsidian robot costs 3 ore and 14 clay.
    // Each geode robot costs 2 ore and 7 obsidian.
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        let blueprint = Blueprint::from(s);
        assert_eq!(1, blueprint.id);
        assert_eq!(4, blueprint.ore_robot.ore);
        assert_eq!(2, blueprint.clay_robot.ore);
        assert_eq!(3, blueprint.obsidian_robot.ore);
        assert_eq!(14, blueprint.obsidian_robot.clay);
        assert_eq!(2, blueprint.geode_robot.ore);
        assert_eq!(7, blueprint.geode_robot.obsidian);
    }
}
