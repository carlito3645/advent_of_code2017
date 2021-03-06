use utils;
use utils::Part;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Program {
    weight: u32,
    holding: Vec<String>,
}

pub fn solve(part: Part) -> String {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 7).unwrap();

    do_the_thing(input, part)
}

//TODO: split this up into separate functions
fn do_the_thing(input:String, part:Part) -> String {

    let mut out = String::new();

    let program_strings: Vec<String> = input.lines()
        .map(|s| s.replace(",", ""))
        .map(|s| s.replace("->", ""))
        .map(|s| s.replace("(", ""))
        .map(|s| s.replace(")", ""))
        .collect();

    let mut programs = HashMap::new();
    let mut holding_names = HashSet::new();

    for s in program_strings {
        let mut links = Vec::new();

        let mut tokens = s.split_whitespace();

        let prog = String::from(tokens.next().unwrap());
        let weight: u32 = tokens.next().unwrap().parse().unwrap();

        while let Some(name) = tokens.next() {
            holding_names.insert(String::from(name));
            links.push(String::from(name));
        }

        programs.insert(prog, Program { weight: weight, holding: links });
    }

    let root_set =
        &HashSet::from_iter(programs.keys().cloned()) - &holding_names;
    let root: String = root_set.iter().next().unwrap().clone();

    match part {
        Part::PartOne => { out = root; }
        Part::PartTwo => {
            match weigh_tree(&programs, &root) {
                WeighResult::Weight(w) => {out = format!("Total Weight: {}", w);},
                WeighResult::ImbalancedNode(name, adjust) => {
                    out = (programs.get(&name).unwrap().weight as i32 + adjust).to_string();
                },
            }
        }
    }

    out
}

enum WeighResult {
    Weight(u32),
    ImbalancedNode(String, i32),
}

fn weigh_tree(tree:&HashMap<String, Program>, root:&str) -> WeighResult {
    let tree_clone = tree.clone();
    let root_node = tree_clone.get(root).unwrap();
    let branches = &root_node.holding;

    let mut weight:u32 = root_node.weight;

    if branches.is_empty() {return WeighResult::Weight(weight);}

    let mut branch_weights:Vec<u32> = Vec::new();

    for branch in branches {
        match weigh_tree(&tree, branch) {
            WeighResult::Weight(w) => {
                branch_weights.push(w);
            },
            WeighResult::ImbalancedNode(name, adjust) => {
                return WeighResult::ImbalancedNode(name, adjust);
            },
        }
    }

    if branch_weights.iter().all(|&x| x == *branch_weights.first().unwrap()) {
        let sum:u32 = branch_weights.iter().sum();
        weight += sum;
    }
    else {
        let mut culprit = String::new();
        let mut adjust:i32 = 0;

        let mut check_one = branch_weights[0]; //safe because iter.all() above is true if empty
        let mut check_two = branch_weights[1]; //safe due to problem specification
        for (name, weight) in branches.iter().zip(branch_weights.iter()) {
            if weight != &check_one && weight == &check_two {
                culprit = branches[0].to_string();
                adjust = *weight as i32 - branch_weights[0] as i32;
            }
            if weight != &check_one && weight != &check_two {
                culprit = name.to_string();
                adjust = branch_weights[0] as i32 - *weight as i32;
            }
            if weight == &check_one && weight != &check_two {
                culprit = branches[1].to_string();
                adjust = *weight as i32 - branch_weights[1] as i32;
            }
        }

        return WeighResult::ImbalancedNode(culprit, adjust);
    }

    WeighResult::Weight(weight)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = String::from("pbga (66)
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
cntj (57)");
        assert_eq!(do_the_thing(test_input, Part::PartOne), String::from("tknk"));
    }

    #[test]
    fn test_part_two() {
        let test_input = String::from("pbga (66)
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
cntj (57)");
        assert_eq!(do_the_thing(test_input, Part::PartTwo), String::from("60"));
    }
}
