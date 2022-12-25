use std::{
    collections::HashMap,
    error::Error,
    fmt::Debug,
    ops::{Index, IndexMut},
};

use crate::util;

static TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

#[derive(Debug, Clone, Copy)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Resources {
    geode: usize,
    obsidian: usize,
    clay: usize,
    ore: usize,
}

impl IndexMut<Resource> for Resources {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        match index {
            Resource::Ore => &mut self.ore,
            Resource::Clay => &mut self.clay,
            Resource::Obsidian => &mut self.obsidian,
            Resource::Geode => &mut self.geode,
        }
    }
}

impl Index<Resource> for Resources {
    type Output = usize;

    fn index(&self, index: Resource) -> &Self::Output {
        match index {
            Resource::Ore => &self.ore,
            Resource::Clay => &self.clay,
            Resource::Obsidian => &self.obsidian,
            Resource::Geode => &self.geode,
        }
    }
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
    robots: [Resources; 4],
}

impl From<&str> for Blueprint {
    fn from(s: &str) -> Self {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();

        Self {
            id: parts[1].strip_suffix(':').unwrap().parse().unwrap(),
            robots: [
                Resources::new(parts[6].parse().unwrap(), 0, 0, 0),
                Resources::new(parts[12].parse().unwrap(), 0, 0, 0),
                Resources::new(parts[18].parse().unwrap(), parts[21].parse().unwrap(), 0, 0),
                Resources::new(parts[27].parse().unwrap(), 0, parts[30].parse().unwrap(), 0),
            ],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Production {
    robots: Resources,
    resources: Resources,
}

impl Production {
    fn can_build(&self, robot: &Resources) -> bool {
        [Resource::Ore, Resource::Clay, Resource::Obsidian]
            .into_iter()
            .all(|r| self.resources[r] >= robot[r])
    }

    fn build(&mut self, robot: &Resources, kind: Resource) {
        for r in [Resource::Ore, Resource::Clay, Resource::Obsidian] {
            self.resources[r] -= robot[r];
        }

        self.robots[kind] += 1;
    }

    fn produce(&mut self) {
        for r in [
            Resource::Ore,
            Resource::Clay,
            Resource::Obsidian,
            Resource::Geode,
        ] {
            self.resources[r] += self.robots[r];
        }
    }
}

impl Blueprint {
    // Blueprint 1:
    // Each ore robot costs 4 ore.
    // Each clay robot costs 2 ore.
    // Each obsidian robot costs 3 ore and 14 clay.
    // Each geode robot costs 2 ore and 7 obsidian.
    fn best(&self, rounds: usize) -> usize {
        let mut prod = vec![Production {
            robots: Resources::new(1, 0, 0, 0),
            resources: Resources::new(0, 0, 0, 0),
        }];

        for _ in 0..rounds {
            // println!("{}: {}", i, prod.len());
            // for p in prod.iter() {
            //     println!("  {:?}", p);
            // }
            let mut next = Vec::new();
            for p in prod {
                let mut p = p;

                // first, check which robots we can create
                let can_build: Vec<_> = [
                    Resource::Ore,
                    Resource::Clay,
                    Resource::Obsidian,
                    Resource::Geode,
                ]
                .into_iter()
                .filter(|r| p.can_build(&self.robots[*r as usize]))
                .collect();

                // produce
                p.produce();

                // create robots
                next.push(p.clone());
                for r in can_build {
                    let mut n = p.clone();
                    n.build(&self.robots[r as usize], r);
                    next.push(n);
                }
            }

            // prune
            let mut hm: HashMap<Resources, Vec<Resources>> = HashMap::new();
            for n in next {
                hm.entry(n.robots)
                    .and_modify(|x| x.push(n.resources))
                    .or_insert(vec![n.resources]);
            }

            let mut next = Vec::new();
            for (robots, mut resources) in hm {
                resources.sort();

                for r in resources.iter().rev().take(4) {
                    next.push(Production {
                        robots: robots.clone(),
                        resources: r.clone(),
                    });
                }
            }

            prod = next;
        }

        prod.iter().map(|x| x.resources.geode).max().unwrap_or(0)
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let quality: usize = util::read_lines(input)
        .map(Blueprint::from)
        .map(|b| b.id * b.best(24))
        .sum();

    println!("quality: {}", quality);

    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let quality: usize = util::read_lines(input)
        .take(3)
        .map(Blueprint::from)
        .map(|b| b.best(32))
        .product();

    println!("quality: {}", quality);
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
        assert_eq!(4, blueprint.robots[Resource::Ore as usize].ore);
        assert_eq!(2, blueprint.robots[Resource::Clay as usize].ore);
        assert_eq!(3, blueprint.robots[Resource::Obsidian as usize].ore);
        assert_eq!(14, blueprint.robots[Resource::Obsidian as usize].clay);
        assert_eq!(2, blueprint.robots[Resource::Geode as usize].ore);
        assert_eq!(7, blueprint.robots[Resource::Geode as usize].obsidian);
    }
}
