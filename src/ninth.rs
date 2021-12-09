use itertools::Itertools;

use crate::util::TaskPart;

pub const DAY: &str = "9th";


fn process_input(input: String) -> Vec<Vec<i32>> {
    let lines = input.lines();

    lines.map(|line| line.chars().into_iter().map(|c| c.to_string().parse().unwrap()).collect::<Vec<i32>>()).collect()
}

struct Heightmap {
    map: Vec<Vec<i32>>,
    filter: Vec<(i32, i32)>,
}

impl Heightmap {
    fn pad(&mut self, with: i32) {
        self.map.insert(0, vec![with; self.map.first().unwrap().len()]);
        self.map.push(vec![with; self.map.first().unwrap().len()]);
        self.map.iter_mut().for_each(|row| {
            row.insert(0, with);
            row.push(with);
        });
    }

    fn is_lowpoint(&self, x: usize, y: usize) -> bool {
        self.map[x][y] < self.filter.iter().map(|neighbour| self.map[(x as i32 + neighbour.0) as usize][(y as i32 + neighbour.1) as usize]).min().unwrap()
    }

    fn run_filter(&self) -> Vec<((usize, usize), i32)> {
        let mut filter_result = Vec::new();
        for i in 1..self.map.len() - 1 {
            for j in 1..self.map[i].len() - 1 {
                if self.is_lowpoint(i, j) {
                    filter_result.push(((i, j), self.map[i][j]));
                }
            }
        }

        filter_result
    }
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Ninth_A!");

    let mut height_map = Heightmap {
        map: process_input(load_input(DAY, TaskPart::A)),
        filter: vec![
            (-1, 0),
            (0, -1),
            (1, 0),
            (0, 1),
        ],
    };

    height_map.pad(10);
    let low_points = height_map.run_filter();

    println!("low points: {:?}", low_points);

    let the_answer: i32 = low_points.iter().map(|(_, val)| val + 1).sum();

    println!("the answer is {}", the_answer);

    store_output(the_answer.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Ninth_B!");

    let mut height_map = Heightmap {
        map: process_input(load_input(DAY, TaskPart::B)),
        filter: vec![
            (-1, 0),
            (0, -1),
            (1, 0),
            (0, 1),
        ],
    };

    height_map.pad(10);
    let low_points = height_map.run_filter();

    println!("low points: {:?}", low_points);

    let the_answer: i32 = low_points.iter().map(|(_, val)| val + 1).sum();

    println!("the answer is {}", the_answer);

    store_output(the_answer.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}