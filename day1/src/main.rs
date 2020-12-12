use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // let res = contains_pair(
    //     io::stdin()
    //         .lock()
    //         .lines()
    //         .filter_map(|l| l.ok())
    //         .filter_map(|l| l.parse::<u32>().ok()),
    // );

    let data: Vec<_> =  io::stdin()
            .lock()
            .lines()
            .filter_map(|l| l.ok())
            .filter_map(|l| l.parse::<u32>().ok()).collect();
    
    let res = contains_triplet(data.as_slice(), 2020);

    if let Some(res) = res {
        println!("{}", res)
    }

    Ok(())
}

#[allow(dead_code)]
fn contains_pair<IterType>(i: IterType) -> Option<u64>
where
    IterType: Iterator<Item = u32>,
{
    let mut mem: HashMap<u32, ()> = HashMap::new();

    for v in i {
        let compl = 2020 - v;
        if mem.contains_key(&v) {
            return Some(v as u64 * compl as u64);
        }
        mem.insert(compl, ());
    }

    None
}

fn contains_triplet(elems: &[u32], sum: u32) -> Option<u64> {
    if elems.len() < 3 {
        return None;
    }

    for i in 0..elems.len() - 1 {
        let mut s = HashSet::new();
        let curr_sum = sum - elems[i];
        for j in i + 1..elems.len() {
            let rem = curr_sum as i64 - elems[j] as i64;
            if s.contains(&rem) {
                return Some(elems[i] as u64 * elems[j] as u64 * rem as u64);
            }
            s.insert(elems[j] as i64);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pair() {
        assert_eq!(contains_pair(vec![2019, 1].into_iter()), Some(2019));
        assert_eq!(contains_pair(vec![2019, 2].into_iter()), None);
    }

    #[test]
    fn test_triplet() {
        assert_eq!(contains_triplet(vec![1, 2, 3].as_slice(), 6), Some(6));
        assert_eq!(contains_triplet(vec![1, 2, 3].as_slice(), 5), None);
    }
}
