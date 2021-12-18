use std::collections::HashMap;
use crate::util::{
    TaskPart,
};

pub const DAY: &str = "15th";

fn process_input(input: String) -> Vec<Vec<u32>>{
    input.lines()
        .map(|line|
            line.chars()
                .map(|num| num.to_digit(10).unwrap())
                .collect::<Vec<u32>>())
        .collect()
}


fn h(cave: &Vec<Vec<u32>>, pos: &(usize, usize), avg_step_cost: f32) -> f32 {
    (cave[(pos.0 + 1).min(cave.len()-1)][pos.1] as f32 + cave[pos.0][(pos.1 + 1).min(cave.len()-1)] as f32) / 2.0
    + (cave.len() - pos.0 + 1) as f32 * avg_step_cost
    + (cave.len() - pos.1 + 1) as f32 * avg_step_cost 
}

fn reconstruct_path(cave: Vec<Vec<u32>>, came_from: HashMap<(usize, usize), (usize, usize)>, current: (usize, usize)) -> Vec<u32>{
    println!("reconstructing path from: {:?}", current);
    let mut path = vec![cave[current.0][current.1]];
    let mut current: (usize, usize) = current;
    while came_from.contains_key(&current) {
        current = *came_from.get(&current).unwrap();
        path.insert(0, cave[current.0][current.1]);
    }

    path
}



pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fifteenth_A!");

    let cave = process_input(load_input(DAY, TaskPart::A));



    let cave_avg_cost = cave.iter().map(|line| line.iter().sum::<u32>()).collect::<Vec<u32>>().iter().sum::<u32>() as f32 / (cave.len() * cave.len()) as f32;
    println!("avg step cost in cave system: {}", cave_avg_cost);
    println!("estimate from start: h((0,0)) => {}", h(&cave, &(0,0), cave_avg_cost));

    const GOAL: (usize, usize) = (99, 99);
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), f32> = HashMap::new();
    let mut f_score: HashMap<(usize, usize), f32> = HashMap::new();
    let mut current: (usize, usize) = (0,0);
    let mut open_set = vec![current];
    g_score.insert(current, 0.0);
    f_score.insert(current, h(&cave, &current, cave_avg_cost));
    let mut print_counter = 0;
    'search: while !open_set.is_empty() {
        current = open_set.pop().unwrap();
        
        // TODO: when adding to open_set do an insert as a binary search for placement in ascending order so that I can pop minimum.
        if current == GOAL {
            println!("reached goal!");
            break 'search;
        }

        let mut neighbours = Vec::new();
        if current.0 > 0 {
            neighbours.push((current.0 - 1, current.1));
        }
        if current.0 < cave.len() - 1 {
            neighbours.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbours.push((current.0, current.1 - 1));
        }
        if current.1 < cave.len() - 1 {
            neighbours.push((current.0, current.1 + 1));
        }

        neighbours.iter().for_each(|neigh_pos| {
            let tent_g_score = g_score[&current] + cave[neigh_pos.0][neigh_pos.1] as f32;
        
            if tent_g_score < *g_score.entry(*neigh_pos).or_insert(f32::MAX) {
                came_from.insert(*neigh_pos, current);
                *g_score.get_mut(&neigh_pos).unwrap() = tent_g_score;
                *f_score.entry(*neigh_pos).or_insert(0.0) = tent_g_score + h(&cave, &current, cave_avg_cost);
                if !open_set.contains(&neigh_pos) {
                    open_set.insert(0, *neigh_pos); // TODO: this in the right way.
                }
            }
        });
        print!(".");
        print_counter += 1;
        if print_counter % 20 == 0 {
            print!("\n");
        }
    }
    print!("\n");

    let path = reconstruct_path(cave, came_from, current);

    let path_cost = path.iter().skip(1).sum::<u32>();
    println!("least cost path found had cost: {}", path_cost);

    store_output(path_cost.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

fn build_cave_map_from_tile(cave_tile: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut cave: Vec<Vec<u32>> = Vec::new();
    (0..5 * cave_tile.len()).for_each(|_| {
        cave.push(vec![0; 5 * cave_tile.len()]);
    });

    for x in 0..cave_tile.len() {
        for y in 0..cave_tile.len() {
            for i in 0..5 {
                let tiled_x = x + cave_tile.len() * i;
                for j in 0..5 {
                    let tiled_y = y + cave_tile.len() * j;
                    cave[tiled_x][tiled_y] = cave_tile[x][y] + (1 * i + 1 * j) as u32;
                    if cave[tiled_x][tiled_y] > 9 {
                        cave[tiled_x][tiled_y] = cave[tiled_x][tiled_y] % 9;
                    }
                }
            }
        }
    }

    println!("cave size: {},{}", cave.len(), cave[0].len());
    println!("THE CAVE!");
    for x in 0..50 {
        if x % 10 == 0 {
            print!("\n");
        }
        print!("\n");
        for y in 0..50 {
            if y % 10 == 0 {
                print!(" ");
            }
            print!("{}", cave[x][y]);
        }
    }

    cave
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fifteenth_B!");

    let cave_tile = process_input(load_input(DAY, TaskPart::B));

    let cave = build_cave_map_from_tile(cave_tile);

    let cave_avg_cost = cave.iter().map(|line| line.iter().sum::<u32>()).collect::<Vec<u32>>().iter().sum::<u32>() as f32 / (cave.len() * cave.len()) as f32;
    println!("avg step cost in cave system: {}", cave_avg_cost);
    println!("estimate from start: h((0,0)) => {}", h(&cave, &(0,0), cave_avg_cost));

    const GOAL: (usize, usize) = (499, 499);
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), f32> = HashMap::new();
    let mut f_score: HashMap<(usize, usize), f32> = HashMap::new();
    let mut current: (usize, usize) = (0,0);
    let mut open_set = vec![current];
    g_score.insert(current, 0.0);
    f_score.insert(current, h(&cave, &current, cave_avg_cost));
    let mut print_counter = 0;
    let mut line_counter = 0;
    'search: while !open_set.is_empty() {
        current = open_set.pop().unwrap();
        
        if current == GOAL {
            println!("reached goal!");
            break 'search;
        }

        let mut neighbours = Vec::new();
        if current.0 > 0 {
            neighbours.push((current.0 - 1, current.1));
        }
        if current.0 < cave.len() - 1 {
            neighbours.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbours.push((current.0, current.1 - 1));
        }
        if current.1 < cave.len() - 1 {
            neighbours.push((current.0, current.1 + 1));
        }

        neighbours.iter().for_each(|neigh_pos| {
            let tent_g_score = g_score[&current] + cave[neigh_pos.0][neigh_pos.1] as f32;
        
            if tent_g_score < *g_score.entry(*neigh_pos).or_insert(f32::MAX) {
                came_from.insert(*neigh_pos, current);
                *g_score.get_mut(&neigh_pos).unwrap() = tent_g_score;
                *f_score.entry(*neigh_pos).or_insert(0.0) = tent_g_score + h(&cave, &current, cave_avg_cost);
                if !open_set.contains(&neigh_pos) {
                    // TODO: when adding to open_set do an insert as a binary search for placement in ascending order so that I can pop minimum.!!
                    // open_set.push(*neigh_pos);
                    open_set.insert(0, *neigh_pos); // TODO: this in the right way.
                    // open_set.sort();
                }
            }
        });
        // print!(".");
        print_counter += 1;
        if print_counter % 200 == 0 {
            line_counter += 1;
            // print!("!\n");
        }
        if line_counter % 2000 == 0 {
            println!("LINE NUMBER {}", line_counter);
            line_counter += 1;
        }
        if current.0 == current.1 {
            println!("Current is {:?}", current);
        }
    }
    print!("\n");

    let path = reconstruct_path(cave, came_from, current);

    let path_cost = path.iter().skip(1).sum::<u32>();
    println!("least cost path found had cost: {}", path_cost);

    store_output(path_cost.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}