use eyre::Result;
use std::collections::HashSet;
use std::fs;

#[derive(Clone)]
struct Group {
    raw_string: String,
}

impl Group {
    fn new(raw_string: String) -> Group {
        Group { raw_string }
    }

    fn compute_set(&self) -> HashSet<char> {
        let mut uniques: HashSet<char> = HashSet::new();
        let cleaned: String = self.raw_string.replace("\n", "");
        let chars: Vec<char> = cleaned.chars().collect();
        for c in &chars {
            uniques.insert(*c);
        }
        uniques
    }

    fn compute_common_set(&self) -> HashSet<char> {
        let mut commons: HashSet<char>;

        let mut sets: Vec<HashSet<char>> = vec![];

        // Read Individual Lines
        for individual in self.raw_string.split('\n').into_iter() {
            if individual.is_empty() {
                continue;
            }
            let chars: Vec<char> = individual.chars().collect();
            let mut iset: HashSet<char> = HashSet::new();
            for c in &chars {
                iset.insert(*c);
            }
            sets.push(iset);
        }

        commons = sets[0].clone();

        for iset in &sets {
            let mut isec = HashSet::new();
            for c in commons.clone().intersection(&iset) {
                isec.insert(*c);
            }
            commons = isec;
        }

        commons
    }
}

fn read_groups(fname: String) -> Result<Vec<Group>> {
    let mut groups: Vec<Group> = vec![];
    for line in fs::read_to_string(fname)?.split("\n\n") {
        if line.is_empty() {
            continue;
        }
        groups.push(Group::new(line.to_string()));
    }
    Ok(groups)
}

fn main() {
    let groups = read_groups("input.txt".to_string()).unwrap();
    let mut sum_uniques: usize = 0;
    for g in groups.iter() {
        let uniques = g.compute_set();
        sum_uniques += uniques.len();
    }
    println!("Sum of Uniques: {}", sum_uniques);

    let mut sum_commons: usize = 0;
    for g in groups.iter() {
        let commons = g.compute_common_set();
        sum_commons += commons.len();
    }

    println!("Sum of Commons: {}", sum_commons);
}
