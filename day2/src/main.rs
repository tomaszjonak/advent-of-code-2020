#![feature(bool_to_option)]
use std::io::{self, BufRead};

fn main() {
    let input: Vec<_> = io::stdin().lock().lines().filter_map(|l| l.ok()).collect();

    let result = parse_passwords2(input.as_slice());
    println!("{}", result);
}

fn parse_passwords(passwords: &[String]) -> usize {
    passwords
        .into_iter()
        .filter_map(|x| parse_password(x))
        .count()
}

fn parse_password(password: &str) -> Option<&str> {
    let parts: Vec<_> = password.split(": ").collect();
    let ctrl_parts: Vec<_> = parts[0].split(" ").collect();
    let min_max: Vec<_> = ctrl_parts[0].split("-").collect();
    let c = ctrl_parts[1].chars().nth(0).unwrap();
    let lower_bound = min_max[0].parse::<usize>().unwrap();
    let upper_bound = min_max[1].parse::<usize>().unwrap();

    let count = parts[1]
        .chars()
        .filter_map(|x| (x == c).then_some(()))
        .count();
    if count < lower_bound || count > upper_bound {
        return None;
    }

    Some(password)
}

fn parse_passwords2(passwords: &[String]) -> usize {
    passwords
        .into_iter()
        .filter_map(|x| parse_password2(x))
        .count()
}

fn parse_password2(password: &str) -> Option<&str> {
    let parts: Vec<_> = password.split(": ").collect();
    let ctrl_parts: Vec<_> = parts[0].split(" ").collect();
    let min_max: Vec<_> = ctrl_parts[0].split("-").collect();
    let c = ctrl_parts[1].chars().nth(0).unwrap();
    let lower_bound = min_max[0].parse::<usize>().unwrap() - 1;
    let upper_bound = min_max[1].parse::<usize>().unwrap() - 1;
    if upper_bound >= parts[1].len() {
        return None;
    }

    let v: Vec<_> = parts[1].chars().collect();
    let mut cc = 0;
    if v[lower_bound] == c {
        cc += 1;
    }

    if v[upper_bound] == c {
        cc += 1;
    }

    if cc != 1 {
        return None;
    }

    Some(password)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        // let input: Vec<String> = vec!["1-2 k: kk".to_owned()];
        // assert_eq!(parse_passwords(input.as_slice()), 1);
        let input: Vec<String> = vec![
            "1-3 a: abcde".to_owned(),
            "1-3 b: cdefg".to_owned(),
            "2-9 c: ccccccccc".to_owned(),
        ];
        assert_eq!(parse_passwords(input.as_slice()), 2);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = vec![
            "1-3 a: abcde".to_owned(),
            "1-3 b: cdefg".to_owned(),
            "2-9 c: ccccccccc".to_owned(),
        ];
        assert_eq!(parse_passwords2(input.as_slice()), 1);
    }
}
