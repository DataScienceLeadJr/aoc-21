use itertools::Itertools;

use crate::util::TaskPart;

pub const DAY: &str = "11th";

fn process_input(input: String) -> Vec<Vec<i32>>{
    input.lines()
        .map(
            |line| 
                line.chars()
                    .map(
                        |c| 
                            c.to_string().parse().unwrap())
                    .collect())
        .collect()
}

struct DumboMap {
    map: Vec<Vec<i32>>,
    filter: Vec<(i32, i32)>,
    height: usize,
    width: usize,
}

impl DumboMap {
    fn new(map_values: Vec<Vec<i32>>, filter: Vec<(i32, i32)>) -> Self {
        let height = map_values.len() + 2;
        let width = map_values[0].len() + 2;
        DumboMap {
            map: map_values,
            filter,
            height,
            width,
        }
    }

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

    fn propagate_flash(&mut self, x: usize, y: usize, flash_locations: &mut Vec<(usize, usize)>, flashes: &mut i32){
        self.map[x][y] = 0;
        flash_locations.push((x,y));
        *flashes += 1;
        for (n, m) in self.filter_locations(x, y) {
            if !flash_locations.contains(&(n,m)) {
                self.map[n][m] += 1;
            }
            if self.map[n][m] > 9 {
                self.propagate_flash(n, m, flash_locations, flashes);
            }
        }
    }
}

pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Eleventh_A!");

    let mut dumbomap = DumboMap::new(
        process_input(load_input(DAY, TaskPart::A)),
        vec![
            (-1,-1), (-1, 0), (-1, 1),
            (0, -1),          ( 0, 1),
            (1, -1), ( 1, 0), ( 1, 1),
            ]
    );
    dumbomap.pad(i32::MIN);

    let mut total_flashes = 0;
    for _ in 0..100 {
        dumbomap.map.iter_mut().for_each(|row| row.iter_mut().for_each(|val| {*val += 1;}));
        let mut flash_locations = Vec::new();
        let mut propagating = true;
        while propagating {
            propagating = false;
            for i in 0..dumbomap.width {
                for j in 0..dumbomap.height {
                    if dumbomap.map[i][j] > 9 && !flash_locations.contains(&(i,j)){
                        propagating = true;
                        dumbomap.propagate_flash(i, j, &mut flash_locations, &mut total_flashes);
                    }
                }
            }
        }
    }

    println!("number of flashes: {}", total_flashes);

    store_output(total_flashes.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Eleventh_B!");

    let mut dumbomap = DumboMap::new(
        process_input(load_input(DAY, TaskPart::B)),
        vec![
            (-1,-1), (-1, 0), (-1, 1),
            (0, -1),          ( 0, 1),
            (1, -1), ( 1, 0), ( 1, 1),
            ]
    );
    dumbomap.pad(i32::MIN);

    let mut sync_step_search = 0;
    let mut total_flashes = 0;
    while !dumbomap.map.iter().all(|row| row.iter().all(|val| *val <= 0)) {
        let mut flash_locations = Vec::new();
        sync_step_search += 1;
            dumbomap.map.iter_mut().for_each(|row| row.iter_mut().for_each(|val| {*val += 1;}));
            let mut propagating = true;
            while propagating {
                propagating = false;
                for i in 0..dumbomap.width {
                    for j in 0..dumbomap.height {
                        if dumbomap.map[i][j] > 9 && !flash_locations.contains(&(i,j)){
                            propagating = true;
                            dumbomap.propagate_flash(i, j, &mut flash_locations, &mut total_flashes);
                        }
                    }
                }
            }
    }

    println!("first sync step: {}", sync_step_search);

    store_output(sync_step_search.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}