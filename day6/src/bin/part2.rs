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
    let mut max = 0u32;
    let mut agg = [0u32; 'z' as usize - 'a' as usize + 1];
    group.split_whitespace().for_each(|x| {
        x.chars().for_each(|c| agg[c as usize - 'a' as usize] += 1);
        max += 1
    });
    agg.iter().filter(|x| **x == max).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!(count_answers(input), 6);
    }
}
