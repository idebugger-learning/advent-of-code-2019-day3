use crate::parser::parser;
use std::collections::HashMap;

mod parser;

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

type Path = Vec<(Direction, usize)>;

fn main() {
    // let input = include_str!("../data/example1.txt");
    // let input = include_str!("../data/example2.txt");
    // let input = include_str!("../data/example3.txt");
    let input = include_str!("../data/input.txt");
    let paths = parser(input);

    println!("Paths: {:?}", paths);

    let intersections = find_intersections(paths);
    println!("Intersections: {:?}", intersections);

    let closest_intersection = intersections.iter().min_by_key(|(_, steps)| steps).unwrap();
    println!("Closest intersection: {:?}", closest_intersection);
    // println!(
    //     "It's manhattan distance: {}",
    //     closest_intersection.0.abs() + closest_intersection.1.abs()
    // );
}

fn find_intersections(paths: Vec<Path>) -> Vec<((isize, isize), usize)> {
    let mut sets = vec![];
    for path in paths {
        let mut x = 0isize;
        let mut y = 0isize;
        let mut steps = 0usize;
        let mut visited = HashMap::new();
        for step in path {
            let (direction, count) = step;
            for _ in 0..count {
                match direction {
                    Direction::Down => y -= 1,
                    Direction::Right => x += 1,
                    Direction::Up => y += 1,
                    Direction::Left => x -= 1,
                }
                steps += 1;
                visited.entry((x, y)).or_insert(steps);
            }
        }
        sets.push(visited);
    }

    sets[0]
        .iter()
        .filter(|(&coords, _)| sets[1].contains_key(&coords))
        .map(|(&coords, &steps)| {
            let sum_steps = steps + sets[1].get(&coords).unwrap();
            (coords, sum_steps)
        })
        .collect::<Vec<_>>()
}
