use std::collections::HashMap;
use regex::Regex;

type Inventory = (usize, usize, usize, usize);

#[derive(Copy, Clone, Debug)]
struct Blueprint {
    id: usize,
    ore: Inventory,
    clay: Inventory,
    obsidian: Inventory,
    geode: Inventory
}

impl Blueprint {
    fn costs(&self) -> [(usize, usize, usize, usize); 4] {
        [self.ore, self.clay, self.obsidian, self.geode]
    }
    
    fn ore_max(&self) -> usize {
        let costs = self.costs();
        costs.iter().map(|c| c.0).max().unwrap()
    }
    
    fn clay_max(&self) -> usize {
        let costs = self.costs();
        costs.iter().map(|c| c.1).max().unwrap()
    }
    
    fn obsidian_max(&self) -> usize {
        let costs = self.costs();
        costs.iter().map(|c| c.2).max().unwrap()
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    let pattern = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    pattern.captures_iter(input).map(|captures| {
        if let [id, ore, clay, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian ] = captures.iter().skip(1).flatten().map(|m| m.as_str().parse::<usize>().unwrap()).collect::<Vec<_>>()[..] {
            Blueprint {
                id,
                ore: (ore, 0, 0, 0),
                clay: (clay, 0, 0, 0),
                obsidian: (obsidian_ore, obsidian_clay, 0, 0),
                geode: (geode_ore, 0, geode_obsidian, 0),
            }
        } else {
            panic!("Invalid assumptions in input processing");
        }
    }).collect()
}

fn solve(blueprint: &Blueprint, robots: Inventory, inventory: Inventory, minutes_remaining: usize, cache: &mut HashMap<(Inventory, Inventory), (usize, usize)>) -> usize {
    if minutes_remaining == 1 {
        return inventory.3 + robots.3;
    }

    let cache_key = (robots, inventory);
    if cache.contains_key(&cache_key) {
        let (time, res) = *cache.get(&cache_key).unwrap();
        if minutes_remaining <= time {
            return res;
        }
    }

    let (ore, clay, obsidian, geode) = inventory;
    let (ore_robots, clay_robots, obsidian_robots, geode_robots) = robots;
    let new_ore = ore + ore_robots;
    let new_clay = clay + clay_robots;
    let new_obsidian = obsidian + obsidian_robots;
    let new_geode = geode + geode_robots;
    let new_inventory = (new_ore, new_clay, new_obsidian, new_geode);
    let mut options = Vec::new();
    options.push(solve(blueprint, robots, new_inventory, minutes_remaining - 1, cache));
    if ore >= blueprint.geode.0 && obsidian >= blueprint.geode.2 {
        let new_inventory = (new_ore - blueprint.geode.0, new_clay, new_obsidian - blueprint.geode.2, new_geode);
        let new_robots = (ore_robots, clay_robots, obsidian_robots, geode_robots + 1);
        options.push(solve(blueprint, new_robots, new_inventory, minutes_remaining - 1, cache));
    } else if ore >= blueprint.obsidian.0 && clay >= blueprint.obsidian.1 && obsidian_robots < blueprint.obsidian_max() && obsidian + obsidian_robots * minutes_remaining < minutes_remaining * blueprint.obsidian_max() {
        let new_inventory = (new_ore - blueprint.obsidian.0, new_clay - blueprint.obsidian.1, new_obsidian, new_geode);
        let new_robots = (ore_robots, clay_robots, obsidian_robots + 1, geode_robots);
        options.push(solve(blueprint, new_robots, new_inventory, minutes_remaining - 1, cache));
    } else {
        if ore >= blueprint.ore.0 && ore_robots < blueprint.ore_max() && ore + ore_robots * minutes_remaining < minutes_remaining * blueprint.ore_max() {
            let new_inventory = (new_ore - blueprint.ore.0, new_clay, new_obsidian, new_geode);
            let new_robots = (ore_robots + 1, clay_robots, obsidian_robots, geode_robots);
            options.push(solve(blueprint, new_robots, new_inventory, minutes_remaining - 1, cache));
        }

        if ore >= blueprint.clay.0 && clay_robots < blueprint.clay_max() && clay + clay_robots & minutes_remaining < minutes_remaining * blueprint.clay_max() {
            let new_inventory = (new_ore - blueprint.clay.0, new_clay, new_obsidian, new_geode);
            let new_robots = (ore_robots, clay_robots + 1, obsidian_robots, geode_robots);
            options.push(solve(blueprint, new_robots, new_inventory, minutes_remaining - 1, cache));
        }
    }

    let res = *options.iter().max().unwrap();
    cache.insert(cache_key, (minutes_remaining, res));
    res
}

fn part1(blueprints: &Vec<Blueprint>) -> usize {
    blueprints.iter().map(|blueprint| {
        let mut cache = HashMap::new();
        let quality = blueprint.id * solve(blueprint, (1, 0, 0, 0), (0, 0, 0, 0), 24, &mut cache);
        quality
    }).sum()
}

fn part2(blueprints: &Vec<Blueprint>) -> usize {
    blueprints.iter().map(|blueprint| {
        let mut cache = HashMap::new();
        let num_geodes = solve(blueprint, (1, 0, 0, 0), (0, 0, 0, 0), 32, &mut cache);
        num_geodes
    }).product()
}

fn main() {
    let input = aoc::io::get_input(19);
    let blueprints = parse(&input);
    println!("{}", part1(&blueprints));
    println!("{}", part2(&blueprints.into_iter().take(3).collect()));
}