use std::convert::TryFrom;

#[derive(Clone, Copy)]
pub enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 6 {
            return Err("string is too short (less than 6)");
        }

        let parts = value.split_at(4);
        let val = parts
            .1
            .parse::<i32>()
            .map_err(|_| "invalid numeric value")?;

        match parts.0.trim() {
            "acc" => Ok(Instruction::ACC(val)),
            "jmp" => Ok(Instruction::JMP(val)),
            "nop" => Ok(Instruction::NOP(val)),
            _ => Err("unrecognized instruction"),
        }
    }
}

pub fn tokenize(input: &str) -> Option<Instruction> {
    Instruction::try_from(input).ok()
}

