use std::io::{self, BufRead};

fn main() {
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok().map(|x| x.chars().collect::<Vec<_>>()))
        .collect();

    let result = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, v| acc * count_down(input.as_slice(), *v));

    println!("{:?}", result);
}

fn count_down(x: &[Vec<char>], v: (usize, usize)) -> usize {
    let length = x.len();
    if length < 1 {
        return 0;
    }

    let width = x[0].len();
    if width == 0 {
        return 0;
    }

    let mut curr_pos: (usize, usize) = (0, 0);
    let mut count = 0;
    while curr_pos.1 < length {
        let c = x[curr_pos.1][curr_pos.0];
        if c == '#' {
            count += 1;
        }
        curr_pos = ((curr_pos.0 + v.0) % width, curr_pos.1 + v.1);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let input = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '#'],
            vec!['.', '.', '#', '.'],
        ];
        assert_eq!(count_down(input.as_slice(), (3, 1)), 2);
    }
}
