#![feature(bool_to_option)]
use lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read};

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
    // let permitted_words: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    //     .iter()
    //     .map(|x| *x)
    //     .collect();
    let words: Vec<(&str, &str)> = p.split_whitespace().map(|x| x.split_at(3)).collect();
    let mut ctr = 0;


    for (id, value) in words {
        let value = value.split_at(1).1;
        ctr |= match id {
            "byr" => byr(value),
            "iyr" => iyr(value),
            "eyr" => eyr(value),
            "hgt" => hgt(value),
            "hcl" => hcl(value),
            "ecl" => ecl(value),
            "pid" => pid(value),
            _ => 0,
        };
    }

    (ctr == 0b1111111).then_some(())
}

fn byr(value: &str) -> u32 {
    has_n_letters(value, 4) * val_in_range(value, 1920, 2002) * (1 << 0)
}

fn iyr(value: &str) -> u32 {
    has_n_letters(value, 4) * val_in_range(value, 2010, 2020) * (1 << 1)
}

fn eyr(value: &str) -> u32 {
    has_n_letters(value, 4) * val_in_range(value, 2020, 2030) * (1 << 2)
}

fn hgt(value: &str) -> u32 {
    let (v, suffix) = value.split_at(value.len() - 2);
    let vv = match suffix.to_lowercase().as_ref() {
        "cm" => val_in_range(v, 150, 193),
        "in" => val_in_range(v, 59, 76),
        _ => 0,
    };
    vv * (1 << 3)
}

fn hcl(value: &str) -> u32 {
    lazy_static::lazy_static! {
        static ref re1: Regex = Regex::new("#[0-9a-f]{6}").unwrap();
    }
    if re1.is_match(value) {
        1 << 4
    } else {
        0
    }
}

fn ecl(value: &str) -> u32 {
    lazy_static::lazy_static! {
        static ref permitted_colors: HashSet<&'static str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .map(|x| *x)
            .collect();
    }
    if permitted_colors.contains(value) {
        1 << 5
    } else {
        0
    }
}

fn pid(value: &str) -> u32 {
    lazy_static::lazy_static! {
        static ref re2: Regex = Regex::new("[0-9]{9}").unwrap();
    }
    if value.len() != 9 || !re2.is_match(value) {
        return 0;
    }
    1 << 6
}

fn has_n_letters(v: &str, n: u32) -> u32 {
    if v.len() as u32 == n {
        return 1;
    }
    0
}

fn val_in_range(v: &str, lower: u32, upper: u32) -> u32 {
    let vv = v.parse::<u32>().unwrap_or(0);
    if vv < lower || vv > upper {
        return 0;
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let invalids = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(count_valid_passwords(invalids), 0);

        let valids = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(count_valid_passwords(valids), 4);
    }

    #[test]
    fn test_preds() {
        assert_eq!(byr("2003"), 0);
        assert_eq!(iyr("2009"), 0);
        assert_eq!(eyr("2019"), 0);
        assert_eq!(hgt("190in"), 0);
        assert_eq!(hgt("190"), 0);
        assert_eq!(hcl("123abc"), 0);
        assert_eq!(ecl("wat"), 0);
        assert_eq!(pid("0123456789"), 0);
    }
}
