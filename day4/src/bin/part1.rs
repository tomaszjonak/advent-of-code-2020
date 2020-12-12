#![feature(bool_to_option)]
use std::io::{self, Read};

use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf)?;
    let count = count_valid_passwords(buf.as_ref());
    println!("{}", count);
    Ok(())
}

fn count_valid_passwords(i: &str) -> usize {
    i.split("\n\n").filter_map(valid_password).count()
}

fn valid_password(p: &str) -> Option<()> {
    let permitted_words: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|x| *x)
        .collect();
    let words: HashSet<&str> = p.split_whitespace().map(|x| x.split_at(3).0).collect();

    permitted_words.is_subset(&words).then_some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(count_valid_passwords(input), 2);
    }
}
