use crate::{Direction, Path};

pub fn parser(input: &str) -> Vec<Path> {
    input
        .split('\n')
        .map(|r| {
            r.split(',')
                .map(|el| {
                    let (raw_dir, raw_steps) = el.split_at(1);
                    let direction = match raw_dir {
                        "L" => Direction::Left,
                        "U" => Direction::Up,
                        "R" => Direction::Right,
                        "D" => Direction::Down,
                        raw_dir => panic!("Unknown direction {}", raw_dir),
                    };
                    let steps = raw_steps.parse::<usize>().unwrap();
                    (direction, steps)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
