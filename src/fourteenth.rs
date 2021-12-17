use crate::util::TaskPart;
use std::collections::HashMap;
use itertools::Itertools;

pub const DAY: &str = "14th";

fn process_input(input: String) -> (Vec<(char, char)>, HashMap<(char, char), char>){
    let mut lines = input.lines();
    let starting_template = lines.next().unwrap().chars().collect::<Vec<char>>();
    let mut starting_pairs = Vec::new();
    for (i, j) in (0..starting_template.len() - 1).zip(1..starting_template.len()) {
        starting_pairs.push((starting_template[i], starting_template[j]));
    }
    //skip empty line
    lines.next();
    
    let insertion_rules = lines.map(|line| {
        let rule: Vec<_> = line.split(" -> ").collect();

        let pattern = (rule[0].chars().nth(0).unwrap(), rule[0].chars().nth(1).unwrap());

        let insert = rule[1].chars().nth(0).unwrap();

        (pattern, insert)
    })
    .collect();

    (starting_pairs, insertion_rules)
}

fn grow_polymer(poly: Vec<(char, char)>, rules: HashMap<(char, char), char>, step: usize, max_step: usize) -> Vec<(char, char)> {
    if step == max_step {
        return poly.clone();
    } else {
        println!("at step.... {}", step + 1);
        grow_polymer(
            poly.iter().flat_map(|pattern| {
                if rules.contains_key(&pattern) {
                    vec![(pattern.0, *rules.get(&pattern).unwrap()),
                    (*rules.get(&pattern).unwrap(), pattern.1)]
                } else {
                    vec![*pattern]
                }
            }).collect(),
            rules,
            step + 1 as usize,
            max_step)
    }
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fourteenth_A!");

    let (starting_template, insertion_rules) = process_input(load_input(DAY, TaskPart::A));
    println!("starting_pairs amount: {}", starting_template.len());
    println!("rule quantity: {}", insertion_rules.keys().len());
    
    let mut counts = HashMap::new();
    starting_template.iter().for_each(|(c1, c2)| {counts.insert(c1.clone(), 0); counts.insert(c2.clone(), 0);});
    
    let mut growth_result = grow_polymer(starting_template, insertion_rules, 0, 10);
    println!("growth_result: {:?}", growth_result.len() + 1);

    *counts.entry(growth_result.remove(0).0.clone()).or_insert(1) += 1;
    *counts.entry(growth_result.pop().unwrap().1.clone()).or_insert(1) += 1;

    growth_result.iter().for_each(|(c1, _c2)| {
        *counts.entry(*c1).or_insert(0) += 1;
    });

    println!("counts after 10 steps:");
    counts.iter().for_each(|(key, value)| {
        println!("{:?} -> {}", key, value);
    });

    let most = counts.values().max().unwrap();
    let least = counts.values().min().unwrap();

    println!("most: {} subtract least: {} => {}", most, least, most - least);

    store_output((most - least).to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

fn insert_into(pair: &(char,char), counts: &mut HashMap<char, u64>, with_prior_amount: u64, into_insertion_rules: &mut HashMap<(char, char), (char, u64)>) {
    let (insert_char, _insert_amount) = into_insertion_rules.get(pair).unwrap().clone();
    *counts.get_mut(&insert_char).unwrap() += with_prior_amount;
    into_insertion_rules.get_mut(pair).unwrap().1 -= with_prior_amount;

    let c = insert_char.clone();
    into_insertion_rules.get_mut(&(pair.0, c)).unwrap().1 += with_prior_amount;
    into_insertion_rules.get_mut(&(c, pair.1)).unwrap().1 += with_prior_amount;
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fourteenth_B!");

    let (starting_template, insertion_rules) = process_input(load_input(DAY, TaskPart::B));

    let mut counts = HashMap::new();
    insertion_rules.iter().for_each(|(key, value)| {counts.insert(key.0.clone(), 0); counts.insert(key.1.clone(), 0); counts.insert(value.clone(), 0);});
    *counts.get_mut(&starting_template.last().unwrap().1).unwrap() += 1;

    let mut insert_counts: HashMap<(char, char), (char, u64)> =
            insertion_rules.iter()
                .map(|(key, value)| {
                    (*key, (*value, 0))
                })
                .collect();

    starting_template.iter().for_each(|(key)| {
        insert_counts.get_mut(key).unwrap().1 += 1;
    });

    // println!("starting insert amounts:");
    insert_counts.iter().for_each(|(key, value)| {
        if value.1 > 0 {
            println!("{:?},{:?}", key, value);
        }
    });

    for step in 0..40 {
        // println!("at step {}..", step + 1 as usize);
        let mut next_insertion_rules = insert_counts.clone();
        insert_counts.iter().for_each(|(key, value)| {
            if value.1 > 0 {
                insert_into(&key, &mut counts, value.1, &mut next_insertion_rules);
            }
        });
        insert_counts = next_insertion_rules.clone();
    }

    println!("counts after 40 steps:");
    counts.iter().for_each(|(key, value)| {
        println!("{:?} -> {}", key, value);
    });
    




    let most = counts.values().max().unwrap();
    let least = counts.values().min().unwrap();

    println!("most: {} subtract least: {} => {}", most, least, most - least);

    store_output((most - least).to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}