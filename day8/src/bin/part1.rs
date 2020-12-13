use std::io::{self, BufRead};
use day8::{Instruction, tokenize};

fn main() {
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .collect();
    let result = run_till_cycle(input);
    println!("{}", result);
}

fn run_till_cycle(input: Vec<String>) -> i32 {
    let instructions: Vec<Instruction> = input
        .into_iter()
        .filter_map(|x| tokenize(x.as_ref()))
        .collect();
    let mut instruction_pointer: i32 = 0;
    let mut acc = 0;
    let mut already_used = vec![false; instructions.len()];
    
    while !already_used[instruction_pointer as usize] {
        already_used[instruction_pointer as usize] = true;

        match instructions[instruction_pointer as usize] {
            Instruction::NOP(_) => instruction_pointer += 1,
            Instruction::ACC(val) => { acc += val; instruction_pointer += 1 },
            Instruction::JMP(val) => instruction_pointer += val,
        };
    }

    acc
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
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
        assert_eq!(run_till_cycle(input), 5);
    }
}
