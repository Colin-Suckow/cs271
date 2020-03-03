use crate::instruction::{Computation, Jump, Destination, Instruction, Binary};

fn assemble_instruction(instruction: &Instruction) -> u16 {
    //TODO: Find a better way to organize this
    add_component(
        add_component(
            add_component(
                add_component(
                    0b0000000000000000   
                , instruction.value)
            , instruction.jump)    
        , instruction.destination)    
    , instruction.computation)

}

fn add_component<T: Binary>(instruction: u16, component: Option<T>) -> u16 {
    instruction | match component {
        Some(dest) => dest.value(),
        _ => 0x0
    }
}



pub fn do_nothing() {}

#[cfg(test)]
mod tests {

    use super::*;

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
