use lazy_static::lazy_static;
use regex;
use std::collections::{HashMap, HashSet, VecDeque};
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
    let mut tree: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.filter_map(munch_line).for_each(|(root, branches)| {
        branches.into_iter().for_each(|branch| {
            tree.entry(branch)
                .or_insert({
                    let mut h = HashSet::new();
                    h.insert(root);
                    h
                })
                .insert(root);
        })
    });

    dfs(start, tree).unwrap_or(0)
}

fn dfs(start: &str, tree: HashMap<&str, HashSet<&str>>) -> Option<usize> {
    let mut stack: VecDeque<&HashSet<&str>> = VecDeque::new();
    let mut traversed: HashSet<&str> = HashSet::new();

    let start_branches = tree.get(start)?;
    stack.push_back(start_branches);

    let mut count = 0;
    while let Some(branches) = stack.pop_front() {
        for branch in branches {
            // insert returns false if set already has this value
            if !traversed.insert(branch) {
                continue;
            }
            
            count += 1;
            if let Some(s) = tree.get(*branch) {
                stack.push_back(s);
            }
        }
    }
    Some(count)
}

fn munch_line(input: &str) -> Option<(&str, Vec<&str>)> {
    lazy_static! {
        static ref R: regex::Regex = regex::Regex::new("^(.*?)contain[s]?(.*?).$").unwrap();
        static ref R2: regex::Regex = regex::Regex::new("(.*?) bag[s]?").unwrap();
        static ref R3: regex::Regex = regex::Regex::new("[0-9]+? (.*?) bag[s]?").unwrap();
    }
    let caps = R.captures(input).unwrap();

    let root = caps.get(0)?.as_str();
    let root = R2.captures(root)?.get(1)?.as_str().trim();

    let parts: Vec<_> = caps
        .get(2)?
        .as_str()
        .split(",")
        .map(|x| x.trim())
        .filter_map(|x| Some(R3.captures(x)?.get(1)?.as_str().trim()))
        .collect();

    Some((root, parts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(count_bags("shiny gold", input.lines()), 4);
    }

    #[test]
    fn test_capture() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        assert_eq!(
            munch_line(input),
            Some(("light red", vec!["bright white", "muted yellow"]))
        );
    }
}
