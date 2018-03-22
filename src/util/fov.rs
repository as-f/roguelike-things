use util::grid;
use util::grid::{Direction, Pos};

const NORMALS: [Direction; 6] = grid::DIRECTIONS;
const TANGENTS: [Direction; 6] = [
    Direction::Southeast,
    Direction::Southwest,
    Direction::West,
    Direction::Northwest,
    Direction::Northeast,
    Direction::East,
];

pub fn fov<F, G>(center: Pos, transparent: F, reveal: G) 
        where F: Fn(Pos) -> bool, G: Fn(Pos) -> () {
    for i in 0..6 {
        let transform = |x: u32, y: u32| -> Pos {
            center + TANGENTS[i] * x as i32 + NORMALS[i] * y as i32
        };
        let transformed_transparent = |x: u32, y: u32| -> bool {
            transparent(transform(x, y))
        };
        let transformed_reveal = |x: u32, y: u32| -> () {
            reveal(transform(x, y))
        };
        scan(1, 0.0, 1.0, &transformed_transparent, &transformed_reveal);
    }
}

fn scan<F, G>(y: u32, mut start: f32, end: f32, transparent: &F, reveal: &G)
        where F: Fn(u32, u32) -> bool, G: Fn(u32, u32) -> () {
    let mut fov_exists = false;
    let x_min = round_high(y as f32 * start);
    let x_max = round_low(y as f32 * end);
    for x in x_min..1+x_max {
        if transparent(x, y) {
            if x as f32 >= y as f32 * start && x as f32 <= y as f32 * end {
                reveal(x, y);
                fov_exists = true;
            }
        } else {
            let end = (x as f32 - 0.5) / y as f32;
            if fov_exists && start < end {
                scan(y + 1, start, end, transparent, reveal);
            }
            reveal(x, y);
            fov_exists = false;
            start = (x as f32 + 0.5) / y as f32;
            if start >= end { return; }
        }
    }
    if fov_exists && start < end {
        scan(y + 1, start, end, transparent, reveal);
    }
}

fn round_high(n: f32) -> u32 {
    n.round() as u32
}

fn round_low(n: f32) -> u32 {
    if n % 1.0 == 0.5 {
        n.round() as u32 - 1
    } else {
        n.round() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_high() {
        assert_eq!(round_high(15.0), 15);
        assert_eq!(round_high(15.49), 15);
        assert_eq!(round_high(15.5), 16);
        assert_eq!(round_high(15.51), 16);
    }

    #[test]
    fn test_round_low() {
        assert_eq!(round_low(15.0), 15);
        assert_eq!(round_low(15.49), 15);
        assert_eq!(round_low(15.5), 15);
        assert_eq!(round_low(15.51), 16);
    }
}
