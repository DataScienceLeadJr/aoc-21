
use crate::util::TaskPart;

pub const DAY: &str = "5th";

#[derive(Debug, Copy, Clone)]
enum Axis{
    Vertical,
    Horizontal,
    Diagoal,
}


#[derive(Debug, Clone, Copy)]
struct Vent {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    along_axis: Axis,
}

impl Vent {
    fn new(x1: &str, x2: &str, y1: &str, y2: &str, along_axis: Axis) -> Self {
        Vent {
            x1: x1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y1: y1.parse().unwrap(),
            y2: y2.parse().unwrap(),
            along_axis,
        }
    }

    fn valid(x1: &str, x2: &str, y1: &str, y2: &str) -> Option<Self> {
        if x1 == x2 {
            Some(Vent::new(x1, x2, y1, y2, Axis::Horizontal))
        } else if y1 == y2{
            Some(Vent::new(x1, x2, y1, y2, Axis::Vertical))
        } else {
            None
        }
    }

    fn valid_with_diag(x1: &str, x2: &str, y1: &str, y2: &str) -> Option<Self> {
        if x1 == x2 {
            Some(Vent::new(x1, x2, y1, y2, Axis::Horizontal))
        } else if y1 == y2{
            Some(Vent::new(x1, x2, y1, y2, Axis::Vertical))
        } else {
            Some(Vent::new(x1, x2, y1, y2, Axis::Diagoal))
        }
    }

    fn max_x(&self) -> i32 {
        self.x1.max(self.x2)
    }

    fn max_y(&self) -> i32 {
        self.y1.max(self.y2)
    }

    fn rasterized(&self) -> Vec<(usize, usize)> {
        let mut coords = Vec::new();
        let traversal;
        match self.along_axis {
            Axis::Vertical => {
                if self.x2 > self.x1 {
                    traversal = self.x1..=self.x2;
                } else {
                    traversal = self.x2..=self.x1;
                }
                traversal.into_iter().for_each(|x| coords.push((x as usize, self.y1 as usize)));
            }
            Axis::Horizontal => {
                if self.y2 > self.y1 {
                    traversal = self.y1..=self.y2;
                } else {
                    traversal = self.y2..=self.y1;
                }
                traversal.into_iter().for_each(|y| coords.push((self.x1 as usize, y as usize)));
            }
            Axis::Diagoal => {
                let traversal_x: Box<dyn Iterator<Item = i32>> = if self.x2 > self.x1 {Box::new(self.x1..=self.x2)} else {Box::new((self.x2..=self.x1).rev())};
                let traversal_y: Box<dyn Iterator<Item = i32>> = if self.y2 > self.y1 {Box::new(self.y1..=self.y2)} else {Box::new((self.y2..=self.y1).rev())};
                traversal_x.into_iter().zip(traversal_y.into_iter()).for_each(|(x, y)| {
                    coords.push((x as usize, y as usize));
                });
            }
        }

        coords
    }
}

fn process_input_day_5<'a>(input: &'a String, task_part: TaskPart) -> Vec<Vent> {
    let maybe_vents: Vec<Option<Vent>> = 
            input
            .lines()
            .map(|line| {
                let end_points: Vec<&str> = line.split(" -> ").collect();

                let start: Vec<_> = end_points[0].split(",").collect();
                let end: Vec<_> = end_points[1].split(",").collect();
                match task_part {
                    TaskPart::A => Vent::valid(start[0], end[0], start[1], end[1]),
                    TaskPart::B => Vent::valid_with_diag(start[0], end[0], start[1], end[1]),
                }
                
            })
            .collect();

    let mut vents: Vec<Vent> = Vec::new();

    maybe_vents.iter().for_each(|option| if option.is_some() { vents.push(option.unwrap())});

    vents
}

trait Output {
    fn print(&self);
}

impl Output for Vec<Vec<i32>> {
    fn print(&self) {
        let empty = ".".to_string();
        self.iter().for_each(
          |row| {
              row.iter().for_each(|val| {
                if *val == 0 {
                    print!("{}", empty);
                } else {
                    print!("{}", val.to_string());
                }
              });
              print!("\n");
          }  
        );
    }
}


pub fn a(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fifh_A!");

    let vents = process_input_day_5(&load_input(DAY, TaskPart::A), TaskPart::A);

    let (world_width, world_height) = (
        vents.iter().map(|vent| vent.max_x()).max().unwrap() as usize + 1,
        vents.iter().map(|vent| vent.max_y()).max().unwrap() as usize + 1,
    );

    println!("ocean size: ({},{})", world_height, world_width);

    let mut ocean = vec![vec![0; world_height]; world_width];

    let mut overlapp_amount = 0;

    vents.iter().for_each(|vent| {
        vent.rasterized().iter().for_each(|coord| {

            if ocean[coord.1][coord.0] == 1{
                overlapp_amount += 1;
            }

            ocean[coord.1][coord.0] += 1;
        })
    });

    //ocean.print();

    println!("overlap amount: {}", overlapp_amount);

    store_output(overlapp_amount.to_string(), DAY, TaskPart::A).expect("funky task not built right... yet?");
}

pub fn b(load_input: &dyn Fn(&str, TaskPart) -> String, store_output: &dyn Fn(String, &str, TaskPart) -> Result<(), std::io::Error>) {
    println!("Fifh_B!");

    let vents = process_input_day_5(&load_input(DAY, TaskPart::B), TaskPart::B);

    let (world_width, world_height) = (
        vents.iter().map(|vent| vent.max_x()).max().unwrap() as usize + 1,
        vents.iter().map(|vent| vent.max_y()).max().unwrap() as usize + 1,
    );

    println!("ocean size: ({},{})", world_height, world_width);

    let mut ocean = vec![vec![0; world_height]; world_width];

    let mut overlapp_amount = 0;

    vents.iter().for_each(|vent| {
        vent.rasterized().iter().for_each(|coord| {

            if ocean[coord.1][coord.0] == 1{
                overlapp_amount += 1;
            }

            ocean[coord.1][coord.0] += 1;
        })
    });

    // ocean.print();

    println!("overlap amount: {}", overlapp_amount);

    store_output(overlapp_amount.to_string(), DAY, TaskPart::B).expect("funky task not built right... yet?");
}