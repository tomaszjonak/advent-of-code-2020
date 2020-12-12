use std::collections::HashSet;
use std::io::{self, Read};
fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    let result = count_answers(buf.as_ref());
    println!("{}", result);
    
    Ok(())
}

fn count_answers(input: &str) -> usize {
    input.split("\n\n").map(count_group).sum()
}

fn count_group(group: &str) -> usize {
    let items: HashSet<_> = group
        .split_whitespace()
        .map(|x| x.chars())
        .flatten()
        .collect();
    items.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "abcx
abcy
abcz";
        assert_eq!(count_answers(input), 6);

        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(count_answers(input), 11);
    }
}
