use crate::instruction::{Binary, Instruction};

pub fn assemble_instruction(instruction: &Instruction) -> u16 {
    0x00
        .add_component(instruction.value)
        .add_component(instruction.jump)
        .add_component(instruction.destination)
        .add_component(instruction.computation)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::instruction::{Computation, Destination, Jump};

    //0
    #[test]
    fn test_assemble_instruction_zero() {
        let instruction = Instruction {
            computation: Some(Computation::Zero),
            ..Default::default()
        };
        assert_eq!(assemble_instruction(&instruction), 0b1000101010000000);
    }

    //1;JMP
    #[test]
    fn test_assemble_instruction_one() {
        let instruction = Instruction {
            computation: Some(Computation::One),
            jump: Some(Jump::JMP),
            ..Default::default()
        };
        assert_eq!(assemble_instruction(&instruction), 0b1000111111000111);
    }

    //A=D+1
    #[test]
    fn test_assemble_instruction_d_plus_one() {
        let instruction = Instruction {
            computation: Some(Computation::DPlusOne),
            destination: Some(Destination::A),
            ..Default::default()
        };
        assert_eq!(assemble_instruction(&instruction), 0b1000011111100000);
    }

    //@1
    #[test]
    fn test_assemble_instruction_a_one() {
        let instruction = Instruction {
            value: Some(1),
            ..Default::default()
        };
        assert_eq!(assemble_instruction(&instruction), 0b0000000000000001);
    }
}
