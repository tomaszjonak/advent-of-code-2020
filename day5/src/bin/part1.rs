use std::collections::HashSet;
use std::io::{self, BufRead};
fn main() {
    let elems: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok().and_then(|x| decode_id(x.as_ref())))
        .map(|(l, w)| l * 8 + w)
        .collect();
    let max = elems.iter().max().unwrap_or(&0);

    let lookup: HashSet<_> = elems.iter().collect();

    for i in 0..(max - 3) {
        if lookup.contains(&i) && !lookup.contains(&(i+1)) && lookup.contains(&(i+2)) {
            println!("{}", i+1);
        }
    }
}

fn decode_id(encoded: &str) -> Option<(u32, u32)> {
    if encoded.len() != 10 {
        return None;
    }

    let (coded_l, coded_w) = encoded.split_at(7);
    Some((decode_l(coded_l), decode_w(coded_w)))
}

fn decode_l(coded_l: &str) -> u32 {
    coded_l
        .char_indices()
        .map(|(pos, c)| decode_single(pos, c, 'F', coded_l.len() - 1))
        .fold(0, |acc, x| acc | x)
}

fn decode_single(pos: usize, c: char, low: char, max: usize) -> u32 {
    if c == low {
        return 0;
    }

    1 << max - pos
}

fn decode_w(coded_w: &str) -> u32 {
    coded_w
        .char_indices()
        .map(|(pos, c)| decode_single(pos, c, 'L', coded_w.len() - 1))
        .fold(0, |acc, x| acc | x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(decode_id("BFFFBBFRRR"), Some((70, 7))); // seat ID 567
        assert_eq!(decode_id("FFFBBBFRRR"), Some((14, 7))); // seat ID 119
        assert_eq!(decode_id("BBFFBBFRLL"), Some((102, 4))); // seat ID 820
    }
}
