use std::iter::Product;

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

    fn filter_locations(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        self.filter.iter().map(|neighbour| ((x as i32 + neighbour.0) as usize, (y as i32 + neighbour.1) as usize)).collect()
    }

    fn filter_values(&self, x: usize, y: usize) -> Vec<i32> {
        self.filter.iter().map(|neighbour| self.map[(x as i32 + neighbour.0) as usize][(y as i32 + neighbour.1) as usize]).collect()
    }

    fn is_lowpoint(&self, x: usize, y: usize) -> bool {
        self.map[x][y] < *self.filter_values(x, y).iter().min().unwrap()
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

    fn expand_frontline(&self, frontline: &mut Vec<(usize, usize)>, x: usize, y: usize, already_searched: &Vec<(usize, usize)>) {
        frontline.append(
            &mut self.filter_values(x,y)
            .into_iter()
            .zip(
                self.filter_locations(x, y))
            .filter(
                |(value, loc)| *value < 9 && !(already_searched.contains(loc) || frontline.contains(loc)))
            .map(
                |(_, loc)| loc)
            .collect::<Vec<(usize, usize)>>())
    }

    fn bfs_from_lowpoint(&self, lowpoint: (usize, usize)) -> Vec<i32>{
        let mut basin = vec![self.map[lowpoint.0][lowpoint.1]];
        let mut already_searched = vec![lowpoint];
        let mut frontline: Vec<(usize, usize)> = Vec::new();
        self.expand_frontline(&mut frontline, lowpoint.0, lowpoint.1, &already_searched);

        // println!("started expanding basin! \n\tstarting loc-val: {:?}_{}\n\tfrontline len: {}", lowpoint, basin.first().unwrap(), frontline.len());

        while !frontline.is_empty() {
            let loc = frontline.remove(0);
            // println!("\tsearching and pushing: {:?}, {}", loc, self.map[loc.0][loc.1]);
            basin.push(self.map[loc.0][loc.1]);
            already_searched.push(loc);
            // let fl_len = frontline.len();
            self.expand_frontline(&mut frontline, loc.0, loc.1, &already_searched);
            // println!("expanded frontline by: {}", frontline.len() - fl_len);
        }
        // println!("done with basin! len: {}", basin.len());
        basin
    }

    fn grow_into_basins(&self, lowpoints: Vec<(usize, usize)>) -> Vec<i32> {
        lowpoints.into_iter().map(|lowpoint| {
            self.bfs_from_lowpoint(lowpoint).len() as i32
        }).collect()
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

    height_map.pad(9);
    let low_points = height_map.run_filter();

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

    height_map.pad(9);
    let low_points = height_map.run_filter();


    let mut basin_sizes = height_map.grow_into_basins(
        low_points.into_iter().map(|(loc, _)| loc).collect()
    );
    basin_sizes.sort();
    println!("Basin sizes: {:?}", basin_sizes);

    let the_answer: i32 = basin_sizes[basin_sizes.len() - 3..].to_owned().into_iter().product();


    println!("the answer is {}", the_answer);

    store_output(the_answer.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}