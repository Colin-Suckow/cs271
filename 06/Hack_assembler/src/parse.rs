use crate::instruction::*;
use regex::Regex;


fn parse_line(line: &str) -> Option<Instruction> {
    
   match line.chars().next() {
        Some('@') => parse_a_instruction(line),
        Some(_) => parse_c_instruction(line),
        None => None
    }
}

fn parse_a_instruction(line: &str) -> Option<Instruction> {
   let value_string = line.split('@').last()?;
   match value_string.parse() {
       Ok(value) => Some(create_a_instruction(value)),
       Err(_) => None
   }
}


fn parse_c_instruction(line: &str) -> Option<Instruction> {

    let re = Regex::new(r"=|;").unwrap();
    let _split_instruction = re.split(line);

    

    Some(Instruction{..Default::default()})
}


fn create_a_instruction(val: u16) -> Instruction {
    Instruction {
        value: Some(val),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_a() {
        let instruction_string = "@2";
       
        assert_eq!(parse_line(instruction_string), Some(Instruction {
            value: Some(2),
            ..Default::default()
        }));
    }

    #[test]
    fn test_parse_c_comp_only() {
        let instruction_string = "A+1";

        assert_eq!(parse_line(instruction_string), Some(Instruction {
            computation: Some(Computation::APlusOne),
            ..Default::default()
        }));
    }
}