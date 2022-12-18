use std::{
    cmp,
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::{graph::Graph, util};

static TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

fn parse_line(s: &str) -> (&str, usize, Vec<&str>) {
    let blah: Vec<_> = s.split(" ").collect();
    let name = blah[1];
    let rate: Vec<_> = blah[4].split("=").collect();
    let rate = rate[1].strip_suffix(';').unwrap().parse().unwrap();
    let next = blah[9..]
        .iter()
        .map(|x| match x.strip_suffix(',') {
            None => x,
            Some(x) => x,
        })
        .collect();
    (name, rate, next)
}

#[derive(Clone, Debug)]
struct Flow {
    open: HashSet<String>,
    flow: Vec<(/* rate */ usize, /* minute */ usize)>,
}

impl Flow {
    fn new() -> Self {
        Self {
            open: HashSet::new(),
            flow: Vec::new(),
        }
    }

    fn open(&mut self, valve: &str, rate: usize, minute: usize) {
        self.open.insert(valve.to_owned());
        self.flow.push((rate, minute))
    }

    fn is_open(&self, valve: &str) -> bool {
        self.open.contains(valve)
    }

    fn total(&self, minutes: usize) -> usize {
        self.flow
            .iter()
            .map(|(rate, minute)| rate * (minutes - minute + 1))
            .sum()
    }
}

struct Volcano {
    rates: HashMap<String, usize>,
    graph: Graph<String>,
}

impl Volcano {
    fn new(s: &str) -> Self {
        let mut rates = HashMap::new();
        let mut g1 = Graph::new();

        for (from, rate, tos) in util::read_lines(s).map(|s| parse_line(s)) {
            for to in tos {
                g1.add_bidirectional_edge(from.to_owned(), to.to_owned(), 1);
            }
            rates.insert(from.to_owned(), rate);
        }

        // we are only interested in AA (the start point) and valves with rate > 0
        let mut interesting: Vec<_> = rates
            .iter()
            .filter_map(|(value, rate)| {
                if *rate > 0 {
                    Some(value.to_owned())
                } else {
                    None
                }
            })
            .collect();
        interesting.push("AA".to_owned());

        // get the shortest paths between the points of interest
        // and make a new graph
        let mut graph = Graph::new();
        for i in 0..interesting.len() - 1 {
            for j in i + 1..interesting.len() {
                let v1 = &interesting[i];
                let v2 = &interesting[j];
                if let Some(x) = g1.shortest_path(v1, v2) {
                    graph.add_bidirectional_edge(v1.to_owned(), v2.to_owned(), x);
                }
            }
        }

        Self { rates, graph }
    }

    fn paths(&self, start: &String, minute: usize, flow: Flow, minutes: usize) -> Vec<Flow> {
        let mut flows = Vec::new();

        let next: Vec<_> = self
            .graph
            .next(start)
            .iter()
            .map(|e| (self.graph.node_data(&e.to), e.cost))
            .filter(|&(valve, cost)| {
                // we only want to move to a valve if it is:
                // - not open
                // - we have time to open it
                // - it has a non-zero rate
                !flow.is_open(valve)
                    && minute + cost + 1 <= minutes
                    && self.rates[&valve.to_owned()] > 0
            })
            .collect();

        if next.is_empty() {
            return vec![flow];
        }

        for (valve, cost) in next {
            let next_minute = minute + cost + 1;
            // println!(
            //     "{} -> {}: cost: {}, minutes: {} -> remaining: {}",
            //     start, valve, cost, minutes, remaining
            // );

            let mut flow = flow.clone();
            flow.open(valve, self.rates[&valve.to_owned()], next_minute);

            flows.append(&mut self.paths(valve, next_minute, flow, minutes))
        }

        flows
    }
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let volcano = Volcano::new(input);

    // volcano.graph.dump();

    let start = "AA".to_owned();

    let flows = volcano.paths(&start, 1, Flow::new(), 30);

    // for flow in flows.iter() {
    //     println!("{:?}: {}", flow, flow.total());
    // }

    println!("flows: {}", flows.len());

    let best = flows.iter().map(|f| f.total(30)).max();

    println!("{:?}", best);

    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let volcano = Volcano::new(input);
    let start = "AA".to_owned();

    // works out routes for me
    let flows = volcano.paths(&start, 1, Flow::new(), 26);

    println!("{} flows", flows.len());

    let mut best = 0;

    // work out for elephant
    for (i, flow) in flows.iter().enumerate() {
        if i % 1000 == 0 {
            println!("flow #{}", i);
        }
        let with_elephant = volcano.paths(&start, 1, flow.clone(), 26);

        let el_best = with_elephant.iter().map(|f| f.total(26)).max().unwrap();

        if el_best > best {
            best = el_best;
            println!("New best: {}", best);
        }
    }

    println!("best: {}", best);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let line = "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE";
        assert_eq!(parse_line(line), ("DD", 20, vec!["CC", "AA", "EE"]));
    }
}
