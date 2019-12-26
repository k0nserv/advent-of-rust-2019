use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use crate::parse_lines;

#[derive(Debug, Clone)]
struct Component {
    reagent: String,
    count: usize,
}

impl Component {
    fn new(reagent: String, count: usize) -> Self {
        Self { reagent, count }
    }
}

#[derive(Clone)]
struct Reaction {
    inputs: Vec<Component>,
    output: Component,
}

impl fmt::Debug for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, input) in self.inputs.iter().enumerate() {
            if idx < self.inputs.len() - 1 {
                write!(f, "{} {}, ", input.count, input.reagent)?;
            } else {
                write!(f, "{} {}  ", input.count, input.reagent)?;
            }
        }

        write!(f, "=> {} {}", self.output.count, self.output.reagent)
    }
}

impl Reaction {
    fn is_ore_reaction(&self) -> bool {
        self.inputs.len() == 1 && self.inputs[0].reagent == "ORE"
    }

    fn output_count(&self) -> usize {
        self.output.count
    }

    fn output_reagent(&self) -> &str {
        &self.output.reagent
    }
}

impl FromStr for Reaction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let clean = input.trim();
        let parts: Vec<_> = clean
            .split("=>")
            .map(str::trim)
            .filter(|p| p.len() > 0)
            .collect();
        if parts.len() != 2 {
            return Err(format!(
                "Each reaction should have excatly two parts, {} had {}",
                input,
                parts.len()
            ));
        }

        let inputs = parts[0]
            .split(",")
            .map(str::trim)
            .map(|i| {
                let segments: Vec<_> = i.split_whitespace().map(str::trim).collect();
                Component::new(
                    segments[1].to_owned(),
                    segments[0].parse::<usize>().unwrap(),
                )
            })
            .collect();

        let output_segments: Vec<_> = parts[1].split_whitespace().map(str::trim).collect();
        if output_segments.len() != 2 {
            return Err(format!(
                "Each reaction should have an output segment with two parts, {} had {}",
                input,
                output_segments.len()
            ));
        }

        Ok(Self {
            inputs,
            output: Component::new(
                output_segments[1].trim().to_owned(),
                output_segments[0].trim().parse::<usize>().unwrap(),
            ),
        })
    }
}

struct Reactor {
    leftovers: HashMap<String, usize>,
    known_reactions: HashMap<String, Reaction>,
    ore_used: usize,
}

impl Reactor {
    fn new(known_reactions: HashMap<String, Reaction>) -> Self {
        Self {
            leftovers: HashMap::new(),
            known_reactions,
            ore_used: 0,
        }
    }

    fn reset(&mut self) {
        self.ore_used = 0;
        self.leftovers.drain();
    }

    fn react(&mut self, reaction: Reaction, mut required_output_count: usize) {
        // Leftover for this output reagent which we can use
        let leftover_for_output = *self.leftovers.get(&reaction.output.reagent).unwrap_or(&0);

        if required_output_count <= leftover_for_output {
            // Satisfy the whole reaction using leftovers
            self.leftovers
                .entry(reaction.output.reagent.clone())
                .and_modify(|e| *e -= required_output_count)
                .or_insert(0);

            return;
        } else if leftover_for_output > 0 {
            // If we have some leftovers, but not enough to fully satisfy the output
            // use them before continuing.

            self.leftovers
                .entry(reaction.output.reagent.clone())
                .and_modify(|e| {
                    *e = 0;
                });

            required_output_count -= leftover_for_output;
        }

        let required_reactions =
            ((required_output_count as f64) / (reaction.output_count() as f64)).ceil() as usize;

        if reaction.is_ore_reaction() {
            let output_count = required_reactions * reaction.output_count();
            let leftover = output_count - required_output_count;

            self.leftovers
                .entry(reaction.output.reagent.clone())
                .and_modify(|e| *e += leftover)
                .or_insert(leftover);

            self.ore_used += required_reactions * reaction.inputs[0].count;
        } else {
            for input in reaction.inputs.iter() {
                let reaction = self.find_known_reaction(&input.reagent).clone();
                self.react(reaction, input.count * required_reactions);
            }

            let leftover = reaction.output_count() * required_reactions - required_output_count;
            self.leftovers
                .entry(reaction.output.reagent.clone())
                .and_modify(|e| *e += leftover)
                .or_insert(leftover);
        }
    }

    fn find_known_reaction(&self, output_reagent: &str) -> &Reaction {
        self.known_reactions.get(output_reagent).unwrap()
    }
}

pub fn star_one(input: &str) -> usize {
    let reactions = parse_lines::<Reaction>(input);
    let mut reactor = Reactor::new(
        reactions
            .map(|r| (r.output_reagent().to_owned(), r))
            .collect(),
    );

    reactor.react(reactor.find_known_reaction("FUEL").clone(), 1);

    reactor.ore_used
}

pub fn star_two(input: &str, available_ore: usize) -> usize {
    let reactions = parse_lines::<Reaction>(input);
    let mut reactor = Reactor::new(
        reactions
            .into_iter()
            .map(|r| (r.output_reagent().to_owned(), r))
            .collect(),
    );
    let mut fuel: f64 = available_ore as f64;
    let mut bound = (0.0, available_ore as f64);
    let mut ore_use: Vec<(usize, f64)> = vec![];

    loop {
        reactor.react(
            reactor.find_known_reaction("FUEL").clone(),
            fuel.floor() as usize,
        );

        ore_use.push((reactor.ore_used, fuel));
        if ore_use
            .iter()
            .rev()
            .nth(2)
            .map(|&p| p.1.floor() as usize == fuel.floor() as usize)
            .unwrap_or(false)
        {
            break;
        }

        if reactor.ore_used > available_ore {
            bound = (bound.0, fuel);
            fuel = bound.0 + (bound.1 - bound.0) / 2.0;
        } else if reactor.ore_used < available_ore {
            bound = (fuel, bound.1);
            fuel = bound.0 + (bound.1 - bound.0) / 2.0;
        }

        reactor.reset();
    }

    let mut last_two: Vec<_> = ore_use.iter().rev().take(2).collect();
    last_two.sort_by_key(|v| v.0);

    if last_two[0].0 > available_ore {
        last_two[1].1.floor() as usize
    } else {
        last_two[0].1.floor() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const TEST_REACTIONS: &'static [(&'static str, usize, Option<usize>)] = &[
        (
            "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
            31,
            None,
        ),
        (
            "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
            165,
            None,
        ),
        (
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
            13312,
            Some(82892753),
        ),
        (
            "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
",
            180697,
            Some(5586022),
        ),
        (
            "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
            2210736,
            Some(460664),
        ),
    ];

    #[test]
    fn test_star_one() {
        for &(input, count, _) in TEST_REACTIONS {
            let result = star_one(input);
            assert_eq!(
                result,
                count,
                "Expected {} to yield output {}. A difference of {}",
                input,
                count,
                (result as isize) - (count as isize),
            );
        }
    }

    #[test]
    fn test_star_two() {
        for &(input, _, fuel) in TEST_REACTIONS {
            match fuel {
                None => continue,
                Some(fuel) => {
                    let result = star_two(input, 1_000_000_000_000);
                    assert_eq!(
                        result,
                        fuel,
                        "Expected {} to yield output {}. A difference of {}",
                        input,
                        fuel,
                        (result as isize) - (fuel as isize),
                    );
                }
            }
        }
    }
}
