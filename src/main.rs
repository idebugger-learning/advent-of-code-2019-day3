use crate::parser::parser;
use std::collections::HashSet;

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

    let closest_intersection = intersections
        .iter()
        .min_by_key(|(x, y)| x.abs() + y.abs())
        .unwrap();
    println!("Closest intersection: {:?}", closest_intersection);
    println!(
        "It's manhattan distance: {}",
        closest_intersection.0.abs() + closest_intersection.1.abs()
    );
}

fn find_intersections(paths: Vec<Path>) -> Vec<(isize, isize)> {
    let mut sets = vec![];
    for path in paths {
        let mut x = 0isize;
        let mut y = 0isize;
        let mut set = HashSet::new();
        for step in path {
            let (direction, count) = step;
            for _ in 0..count {
                match direction {
                    Direction::Down => y -= 1,
                    Direction::Right => x += 1,
                    Direction::Up => y += 1,
                    Direction::Left => x -= 1,
                }
                set.insert((x, y));
            }
        }
        sets.push(set);
    }

    sets[0]
        .intersection(&sets[1])
        .map(|r| *r)
        .collect::<Vec<_>>()
}
