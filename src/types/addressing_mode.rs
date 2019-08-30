use std::fmt::{
    self,
    Display
};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::{BytePattern, Width, Offset};

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum AddressingMode
{
    Immediate           = 0b_000,
    Direct              = 0b_001,
    Indirect            = 0b_010,
    InstructionPointer  = 0b_011,
}

impl Display for AddressingMode
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let out = match self
        {
            Self::Immediate => "$",
            Self::Direct => "@",
            Self::Indirect => "!",
            Self::InstructionPointer => "%ip"
        };

        write!(f, "{}", out)
    }
}

impl From<u8> for AddressingMode
{
    fn from(num: u8) -> Self
    {
        AddressingMode::from_u8(num).expect("AddressingMode can't be converted from a supplied number!")
    }
}

impl From<AddressingMode> for u8
{
    fn from(mode: AddressingMode) -> Self
    {
        mode as u8
    }
}

impl BytePattern for AddressingMode
{
    const WIDTH: Width = Width::Three;
    const OFFSET: Offset = Offset::Zero;
}