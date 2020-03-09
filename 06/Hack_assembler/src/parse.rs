use crate::instruction::*;
use regex::Regex;
use std::str::FromStr;


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

    //Fragile, fix later

    let re = Regex::new(r"=|;").unwrap();
    let instruction_components = re.split(line).collect::<Vec<&str>>();
    
    let re = Regex::new(r"=").unwrap(); 
    let has_destination = re.is_match(line);
    
    let re = Regex::new(r";").unwrap();
    let has_jump = re.is_match(line);


    if has_destination && has_jump {
        return Some(Instruction{
            computation: Some(Computation::from_str(instruction_components[1]).unwrap()),
            destination: Some(Destination::from_str(instruction_components[0]).unwrap()),
            jump: Some(Jump::from_str(instruction_components[2]).unwrap()),
            ..Default::default()
        });
    } else if has_destination && !has_jump {
        return Some(Instruction{
            computation: Some(Computation::from_str(instruction_components[1]).unwrap()),
            destination: Some(Destination::from_str(instruction_components[0]).unwrap()),
            ..Default::default()
        });
    } else if has_jump && !has_destination {
        return Some(Instruction{
            computation: Some(Computation::from_str(instruction_components[0]).unwrap()),
            jump: Some(Jump::from_str(instruction_components[1]).unwrap()),
            ..Default::default()
        });
    } else {
        return Some(Instruction{
            computation: Some(Computation::from_str(instruction_components[0]).unwrap()),
            ..Default::default()
        });
    }
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

    #[test]
    fn test_parse_c_dest_no_jump() {
        let instruction_string = "A=A+1";

        assert_eq!(parse_line(instruction_string), Some(Instruction {
            computation: Some(Computation::APlusOne),
            destination: Some(Destination::A),
            ..Default::default()
        }));
    }

    #[test]
    fn test_parse_c_jump_no_dest() {
        let instruction_string = "A+1;JMP";

        assert_eq!(parse_line(instruction_string), Some(Instruction {
            computation: Some(Computation::APlusOne),
            jump: Some(Jump::JMP),
            ..Default::default()
        }));
    }

    #[test]
    fn test_parse_c_dest_and_jump() {
        let instruction_string = "AD=A+1;JGE";

        assert_eq!(parse_line(instruction_string), Some(Instruction {
            computation: Some(Computation::APlusOne),
            destination: Some(Destination::AD),
            jump: Some(Jump::JGE),
            ..Default::default()
        }));
    }
}