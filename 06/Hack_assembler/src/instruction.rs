use std::str::FromStr;
use std::fmt;

pub trait Binary {
    fn value(&self) -> u16;

    fn add_component<T: Binary>(&self, component: Option<T>) -> u16 {
        self.value() | match component {
            Some(dest) => dest.value(),
            _ => 0x0
        }
    }
}

impl Binary for u16 {
    fn value(&self) -> u16 {
        *self
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Computation {
    Zero,
    One,
    NegativeOne,
    D,
    A,
    NotD,
    NotA,
    NegativeD,
    NegativeA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
}

impl Binary for Computation {
    fn value(&self) -> u16 {
        match *self {
            Computation::Zero => 0b1000101010000000,
            Computation::One => 0b1000111111000000,
            Computation::NegativeOne => 0b1000111010000000,
            Computation::D => 0b1000001100000000,
            Computation::A => 0b1000110000000000,
            Computation::NotD => 0b1000001101000000,
            Computation::NotA => 0b1000110001000000,
            Computation::NegativeD => 0b1000001111000000,
            Computation::NegativeA => 0b1000110011000000,
            Computation::DPlusOne => 0b1000011111000000,
            Computation::APlusOne => 0b1000110111000000,
            Computation::DMinusOne => 0b1000001110000000,
            Computation::AMinusOne => 0b1000110010000000,
            Computation::DPlusA => 0b1000000010000000,
            Computation::DMinusA => 0b1000010011000000,
            Computation::AMinusD => 0b1000000111000000,
            Computation::DAndA => 0b1000000000000000,
            Computation::DOrA => 0b1000010101000000,
        }
    }
}

impl FromStr for Computation {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Computation, Self::Err> {
        match s {
            "0" => Ok(Computation::Zero),
            "1" => Ok(Computation::One),
            "-1" => Ok(Computation::NegativeOne),
            "D" => Ok(Computation::D),
            "A" => Ok(Computation::A),
            "!D" => Ok(Computation::NotD),
            "!A" => Ok(Computation::NotA),
            "-D" => Ok(Computation::NegativeD),
            "-A" => Ok(Computation::NegativeA),
            "D+1" => Ok(Computation::DPlusOne),
            "A+1" => Ok(Computation::APlusOne),
            "D-1" => Ok(Computation::DMinusOne),
            "A-1" => Ok(Computation::AMinusOne),
            "D+A" => Ok(Computation::DPlusA),
            "D-A" => Ok(Computation::DMinusA),
            "A-D" => Ok(Computation::AMinusD),
            "D&A" => Ok(Computation::DAndA),
            "D|A" => Ok(Computation::DOrA),
            _ => Err(InstructionParseError{})
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Destination {
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD
}

impl Binary for Destination {
    fn value(&self) -> u16 {
        match *self {
            Destination::M => 0b0000000000001000,
            Destination::D => 0b0000000000010000,
            Destination::MD => 0b0000000000011000,
            Destination::A => 0b0000000000100000,
            Destination::AM => 0b0000000000101000,
            Destination::AD => 0b0000000000110000,
            Destination::AMD => 0b0000000000111000
        }
    }
}

impl FromStr for Destination {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Destination, Self::Err> {
        match s {
            "M" => Ok(Destination::M),
            "D" => Ok(Destination::D),
            "MD" => Ok(Destination::MD),
            "A" => Ok(Destination::A),
            "AM" => Ok(Destination::AM),
            "AD" => Ok(Destination::AD),
            "AMD" => Ok(Destination::AMD),
            _ => Err(InstructionParseError{})
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Jump {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP
}

impl Binary for Jump {
    fn value(&self) -> u16 {
        match *self {
            Jump::JGT => 0b0000000000000001,
            Jump::JEQ => 0b0000000000000010,
            Jump::JGE => 0b0000000000000011,
            Jump::JLT => 0b0000000000000100,
            Jump::JNE => 0b0000000000000101,
            Jump::JLE => 0b0000000000000110,
            Jump::JMP => 0b0000000000000111
        }
    }
}

impl FromStr for Jump {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Jump, Self::Err> {
        match s {
            "JGT" => Ok(Jump::JGT),
            "JEQ" => Ok(Jump::JEQ),
            "JGE" => Ok(Jump::JGE),
            "JLT" => Ok(Jump::JLT),
            "JNE" => Ok(Jump::JNE),
            "JLE" => Ok(Jump::JLE),
            "JMP" => Ok(Jump::JMP),
            _ => Err(InstructionParseError{})
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    pub computation: Option<Computation>,
    pub value: Option<u16>,
    pub destination: Option<Destination>,
    pub jump: Option<Jump>
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            computation: None,
            value: None,
            destination: None,
            jump: None
        }
    }
}

#[derive(Debug)]
pub struct InstructionParseError  {}

impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse instruction component")
    }
}


