pub trait Binary {
    fn value(&self) -> u16;
}

impl Binary for u16 {
    fn value(&self) -> u16 {
        *self
    }
}

#[derive(Copy, Clone)]
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
    DoOrA,
}

impl Binary for Computation {
    fn value(&self) -> u16 {
        match *self {
            /*
            Computation::Zero => 0b0000000101010001,
            Computation::One => 0b0000001111110001,
            Computation::NegativeOne => 0b0000000101110001,
            Computation::D => 0b0000000011000001,
            Computation::A => 0b0000000000110001,
            Computation::NotD => 0b0000001011000001,
            Computation::NotA => 0b0000001000110001,
            Computation::NegativeD => 0b0000001111000001,
            Computation::NegativeA => 0b0000001100110001,
            Computation::DPlusOne => 0b0000001111100001,
            Computation::APlusOne => 0b0000001110110001,
            Computation::DMinusOne => 0b0000000111000001,
            Computation::AMinusOne => 0b0000000100110001,
            Computation::DPlusA => 0b0000000100000001,
            Computation::DMinusA => 0b0000001100100001,
            Computation::AMinusD => 0b0000001110000001,
            Computation::DAndA => 0b0000000000000001,
            Computation::DoOrA => 0b0000001010100001,
            */

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
            Computation::DoOrA => 0b1000010101000000,
        }
    }
}
#[derive(Copy, Clone)]
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
            /*
            Destination::M => 0b0001000000000000,
            Destination::D => 0b0000100000000000,
            Destination::MD => 0b0001100000000000,
            Destination::A => 0b0000010000000000,
            Destination::AM => 0b0001010000000000,
            Destination::AD => 0b0000110000000000,
            Destination::AMD => 0b0001110000000000
            */

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

#[derive(Copy, Clone)]
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
            /*
            Jump::JGT => 0b1000000000000000,
            Jump::JEQ => 0b0100000000000000,
            Jump::JGE => 0b1100000000000000,
            Jump::JLT => 0b0010000000000000,
            Jump::JNE => 0b1010000000000000,
            Jump::JLE => 0b0110000000000000,
            Jump::JMP => 0b1110000000000000
            */

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
