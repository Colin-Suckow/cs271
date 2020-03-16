use std::fmt;
use std::str::FromStr;

pub trait Binary {
    fn value(&self) -> u16;

    fn add_component<T: Binary>(&self, component: Option<T>) -> u16 {
        self.value()
            | match component {
                Some(dest) => dest.value(),
                _ => 0x0,
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

    M,
    NotM,
    NegativeM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM
}

impl Binary for Computation {
    fn value(&self) -> u16 {
        match *self {
            Computation::Zero => 0b1110101010000000,
            Computation::One => 0b1110111111000000,
            Computation::NegativeOne => 0b1110111010000000,
            Computation::D => 0b1110001100000000,
            Computation::A => 0b1110110000000000,
            Computation::NotD => 0b1110001101000000,
            Computation::NotA => 0b1110110001000000,
            Computation::NegativeD => 0b1110001111000000,
            Computation::NegativeA => 0b1110110011000000,
            Computation::DPlusOne => 0b1110011111000000,
            Computation::APlusOne => 0b1110110111000000,
            Computation::DMinusOne => 0b1110001110000000,
            Computation::AMinusOne => 0b1110110010000000,
            Computation::DPlusA => 0b1110000010000000,
            Computation::DMinusA => 0b1110010011000000,
            Computation::AMinusD => 0b1110000111000000,
            Computation::DAndA => 0b1110000000000000,
            Computation::DOrA => 0b1110010101000000,

            Computation::M => 0b1111110000000000,
            Computation::NotM => 0b1111110001000000,
            Computation::NegativeM => 0b1111110011000000,
            Computation::MPlusOne => 0b1111110111000000,
            Computation::MMinusOne => 0b1111110010000000,
            Computation::DPlusM => 0b1111000010000000,
            Computation::DMinusM => 0b1111010011000000,
            Computation::MMinusD => 0b1111000111000000,
            Computation::DAndM => 0b1111000000000000,
            Computation::DOrM => 0b1111010101000000,

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

            "M" => Ok(Computation::M),
            "!M" => Ok(Computation::NotM),
            "-M" => Ok(Computation::NegativeM),
            "M+1" => Ok(Computation::MPlusOne),
            "M-1" => Ok(Computation::MMinusOne),
            "D+M" => Ok(Computation::DPlusM),
            "D-M" => Ok(Computation::DMinusM),
            "M-D" => Ok(Computation::MMinusD),
            "D&M" => Ok(Computation::DAndM),
            "D|M" => Ok(Computation::DOrM),
            _ => Err(InstructionParseError {}),
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
    AMD,
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
            Destination::AMD => 0b0000000000111000,
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
            _ => Err(InstructionParseError {}),
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
    JMP,
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
            Jump::JMP => 0b0000000000000111,
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
            _ => Err(InstructionParseError {}),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    pub computation: Option<Computation>,
    pub value: Option<u16>,
    pub destination: Option<Destination>,
    pub jump: Option<Jump>,
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            computation: None,
            value: None,
            destination: None,
            jump: None,
        }
    }
}

impl Instruction {
    pub fn is_empty(self: &Self) -> bool {
        self.computation == None && self.destination == None && self.jump == None && self.value == None //Also ew. Should make a line object trait or something
    }
}

#[derive(Debug)]
pub struct InstructionParseError {}

impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse instruction component")
    }
}
