use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Orbits<'a> = HashMap<&'a str, Option<&'a str>>;

fn parse(input: &str) -> Orbits {
    let orbits = input.lines().map(str::trim).filter(|l| l.len() > 0);
    let mut known_orbits: Orbits = HashMap::with_capacity(input.len() / 2);

    for orbit in orbits {
        let parts: Vec<_> = orbit.split(")").map(str::trim).collect();
        assert!(
            parts.len() == 2,
            "Each orbit definition should have exactly two parts"
        );

        let inner_id = parts[0];
        let outer_id = parts[1];

        known_orbits.entry(inner_id).or_insert(None);

        known_orbits
            .entry(outer_id)
            .and_modify(|e| {
                *e = Some(inner_id);
            })
            .or_insert(Some(inner_id));
    }

    known_orbits
}

thread_local!(static MEMORY: RefCell<HashMap<String, usize>> = RefCell::new(HashMap::default()));
fn orbit_count(id: &str, orbits: &Orbits, to_target: Option<&str>) -> usize {
    let saved = if to_target.is_some() {
        None
    } else {
        MEMORY.with(|memory| memory.borrow().get(id).map(|v| *v))
    };

    match saved {
        Some(v) => v,
        None => {
            let count = orbits
                .get(id)
                .unwrap()
                .map(|inner| {
                    if to_target.is_none() {
                        1 + orbit_count(inner, orbits, to_target)
                    } else {
                        let is_target = to_target.map(|target| target == inner).unwrap_or(false);

                        if is_target {
                            1
                        } else {
                            1 + orbit_count(inner, orbits, to_target)
                        }
                    }
                })
                .unwrap_or(0);

            if to_target.is_none() {
                MEMORY.with(|memory| {
                    memory.borrow_mut().insert(id.to_owned(), count);
                });
            }

            count
        }
    }
}

fn all_orbits<'a, 'b>(id: &'a str, orbits: &'b Orbits) -> Vec<&'b str> {
    let mut next = orbits.get(id);
    let mut result = vec![];

    while let Some(inner) = next {
        match inner {
            Some(inner) => result.push(*inner),
            None => (),
        };

        next = inner.and_then(|inner| orbits.get(inner));
    }

    result.reverse();

    result
}

pub fn star_one(input: &str) -> usize {
    MEMORY.with(|memory| {
        // Reset memory to prevent different inputs interfering with eachother
        memory.borrow_mut().drain();
    });
    let orbits = parse(input);

    orbits.keys().fold(0, |acc, key| {
        let count = orbit_count(key, &orbits, None);

        acc + count
    })
}

pub fn star_two(input: &str) -> usize {
    let orbits = parse(input);
    let you_orbits = all_orbits("YOU", &orbits);
    let san_orbits = all_orbits("SAN", &orbits);
    let last_common = you_orbits
        .into_iter()
        .zip(san_orbits.into_iter())
        .take_while(|(a, b)| a == b)
        .map(|v| v.0)
        .last()
        .unwrap();

    (orbit_count("YOU", &orbits, Some(last_common)) - 1)
        + (orbit_count("SAN", &orbits, Some(last_common)) - 1)
}

#[cfg(test)]
mod tests {
    use super::{orbit_count, parse, star_one, star_two, Orbits};
    use std::collections::HashMap;
    use std::rc::Rc;

    const TEST_INPUT_PART_1: &str = "
COM)B
B)C
D)E
E)F
B)G
D)I
G)H
E)J
J)K
K)L
C)D
    ";

    const TEST_INPUT_PART_2: &str = "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";

    #[test]
    fn test_orbit_count() {
        let orbits: Orbits = parse(TEST_INPUT_PART_1);
        dbg!(&orbits.get("E").unwrap());
        dbg!(&orbits.get("D").unwrap());

        assert_eq!(orbit_count("L", &orbits, None), 7);
        assert_eq!(orbit_count("COM", &orbits, None), 0);
        assert_eq!(orbit_count("D", &orbits, None), 3);
        assert_eq!(orbit_count("C", &orbits, None), 2);
    }

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(TEST_INPUT_PART_1), 42);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(TEST_INPUT_PART_2), 4);
    }
}
