use day8::{tokenize, Instruction};
use std::io::{self, BufRead};

fn main() {
    let input: Vec<_> = io::stdin().lock().lines().filter_map(|x| x.ok()).collect();
    let result = find_convergent_assembly(input);
    println!("{}", result);
}

fn find_convergent_assembly(input: Vec<String>) -> i32 {
    let base_assembly: Vec<Instruction> = input
        .into_iter()
        .filter_map(|x| tokenize(x.as_ref()))
        .collect();

    prepare_possible_fixes(base_assembly)
        .into_iter()
        .filter_map(|proposed| test_fix(proposed))
        .take(1)
        .next()
        .unwrap_or(0)
}

fn test_fix(instructions: Vec<Instruction>) -> Option<i32> {
    let mut iptr: i32 = 0;
    let mut acc = 0;
    let mut already_used = vec![false; instructions.len()];
    
    while iptr < instructions.len() as i32 && iptr >= 0 && !already_used[iptr as usize] {
        already_used[iptr as usize] = true;

        match instructions[iptr as usize] {
            Instruction::NOP(_) => iptr += 1,
            Instruction::ACC(val) => { acc += val; iptr += 1 },
            Instruction::JMP(val) => iptr += val,
        };
    }

    match iptr < instructions.len() as i32 {
        true => None,
        false => Some(acc),
    }
}

fn prepare_possible_fixes(base_assembly: Vec<Instruction>) -> Vec<Vec<Instruction>> {
    let mut result: Vec<Vec<Instruction>> = Vec::new();

    for (idx, instruction) in base_assembly.iter().enumerate() {
        match instruction {
            Instruction::NOP(val) => {
                let mut new = base_assembly.to_vec();
                new[idx] = Instruction::JMP(*val);
                result.push(new);
            },
            Instruction::JMP(val) => {
                let mut new = base_assembly.to_vec();
                new[idx] = Instruction::NOP(*val);
                result.push(new);
            }, 
            Instruction::ACC(_) => {},
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let input = input.lines().map(|x| x.to_owned()).collect();
        assert_eq!(run_till_completion(input), 8);
    }
}
