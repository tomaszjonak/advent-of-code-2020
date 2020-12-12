use lazy_static::lazy_static;
use regex;
use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let input: VecDeque<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let result = count_bags("shiny gold", input.iter().map(|x| x.as_ref()));
    println!("{}", result);
}

fn count_bags<'a, T>(start: &str, input: T) -> usize
where
    T: Iterator<Item = &'a str>,
{
    let tree: HashMap<&str, Vec<(&str, usize)>> = input.filter_map(munch_line).collect();

    let mut stack: VecDeque<(&str, usize)> = VecDeque::new();

    stack.push_back((start, 1));

    let mut sum = 0;
    while let Some((branch, acc)) = stack.pop_front() {
        if let Some(branches) = tree.get(branch) {
            for (nested_branch, amount) in branches {
                let mul  = acc * amount;
                sum += mul;
                stack.push_back((nested_branch, mul));
            }
        }
    }

    sum
}

fn munch_line(input: &str) -> Option<(&str, Vec<(&str, usize)>)> {
    lazy_static! {
        static ref R: regex::Regex = regex::Regex::new("^(.*?)contain[s]?(.*?).$").unwrap();
        static ref R2: regex::Regex = regex::Regex::new("(.*?) bag[s]?").unwrap();
        static ref R3: regex::Regex = regex::Regex::new("([1-9][0-9]*?) (.*?) bag[s]?").unwrap();
    }
    let caps = R.captures(input).unwrap();

    let root = caps.get(0)?.as_str();
    let root = R2.captures(root)?.get(1)?.as_str().trim();

    let parts: Vec<_> = caps
        .get(2)?
        .as_str()
        .split(",")
        .map(|x| x.trim())
        .filter_map(|x| {
            let caps = R3.captures(x)?;
            let name = caps.get(2)?.as_str().trim();
            let count = caps.get(1)?.as_str().parse::<usize>().ok()?;
            Some((name, count))
        })
        .collect();

    if parts.len() == 0 {
        return None;
    }

    Some((root, parts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_munch_line() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        assert_eq!(
            munch_line(input),
            Some(("light red", vec![("bright white", 1), ("muted yellow", 2)]))
        );
    }

    #[test]
    fn test_small() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(count_bags("shiny gold", input.lines()), 32);
    }

    #[test]
    fn test_big() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(count_bags("shiny gold", input.lines()), 126);
    }

    #[test]
    fn test_s1() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bag contain no other bags.";

        assert_eq!(count_bags("shiny gold", input.lines()), 6);
    }
}
