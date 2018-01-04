/// This code is heavily inspired from Peter Norvig's solution.  I was
/// having a lot of trouble with this problem, because I thought we
/// were supposed to find the correct weight for a program in the
/// second level of the tower. What we are supposed to do is try to
/// "balance" as hight up in the tower as possible.
extern crate regex;
extern crate advent;

use advent::get_path_or_exit;
use advent::read_file;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
struct Tower<'a> {
    weight: HashMap<&'a str, u32>,
    above: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Tower<'a> {
    fn base(&self) -> &str {
        // The base is the program that doesn't appear above anything
        let above: HashSet<&str> = self.above.values().flat_map(|&ref a| a.clone()).collect();
        self.above.keys().find(|&key| !above.contains(key)).unwrap()
    }

    // TODO: error handling if given bad name
    fn tower_weight(&self, name: &str) -> u32 {
        let weight = self.weight.get(name).unwrap();
        let above_weight: u32 = self.above
            .get(name)
            .unwrap()
            .iter()
            .map(|above| self.tower_weight(above))
            .sum();
        weight + above_weight
    }

    // TODO: error handling if given bad name
    fn siblings(&self, name: &str) -> HashSet<&str> {
        match self.above.values().find(|above| above.contains(&name)) {
            None => HashSet::new(),
            Some(above) => above.iter().filter(|&n| *n != name).map(|n| *n).collect(),
        }
    }

    fn wrong(&self, name: &str) -> bool {
        if name == self.base() {
            false
        } else {
            // weight is not the weight of its siblings
            self.siblings(name)
                .iter()
                .map(|sibling| self.tower_weight(sibling))
                .find(|&weight| self.tower_weight(name) == weight)
                .is_none()
        }
    }

    fn wrongest(&self) -> &str {
        let programs: Vec<&str> = self.programs();
        programs
            .into_iter()
            .find(|name| {
                self.wrong(name)
                    && !self.above
                        .get(name)
                        .unwrap()
                        .iter()
                        .any(|above| self.wrong(above))
            })
            .unwrap()
    }

    fn correct_weight(&self, name: &str) -> u32 {
        let sibling = self.siblings(name)
            .into_iter()
            .take(1)
            .collect::<Vec<&str>>()[0];
        let delta = self.tower_weight(sibling) as i32 - self.tower_weight(name) as i32;
        ((*self.weight.get(name).unwrap() as i32) + delta) as u32
    }

    fn programs(&self) -> Vec<&str> {
        self.weight.keys().map(|s| *s).collect()
    }
}

// TODO: error handling
fn parse(input: &str) -> Tower {
    let mut tower = Tower {
        weight: HashMap::new(),
        above: HashMap::new(),
    };
    let re = Regex::new(r"\w+").unwrap();
    for line in input.lines() {
        let mut iter = re.find_iter(&line);
        let name = iter.next().unwrap().as_str();
        let weight = iter.next().unwrap().as_str().parse().unwrap();
        tower.weight.insert(name, weight);

        let above = iter.map(|m| m.as_str()).collect();
        tower.above.insert(name, above);
    }
    tower
}

fn main() {
    let path = get_path_or_exit();
    let input = read_file(&path).unwrap();
    let tower = parse(&input);
    println!("part 1: {}", tower.base());
    println!("part 2: {:?}", tower.correct_weight(tower.wrongest()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tower() -> Tower<'static> {
        parse(
            "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
        )
    }

    #[test]
    fn test_base() {
        assert_eq!(tower().base(), "tknk");
    }

    #[test]
    fn test_weight() {
        let tower = tower();
        assert_eq!(tower.tower_weight("cntj"), 57);
        assert_eq!(tower.tower_weight("ugml"), 251);
    }

    #[test]
    fn test_wrongest() {
        let tower = tower();
        assert_eq!(tower.wrongest(), "ugml");
    }

    #[test]
    fn test_correct_weight() {
        let tower = tower();
        assert_eq!(tower.correct_weight("ugml"), 60);
    }
}
