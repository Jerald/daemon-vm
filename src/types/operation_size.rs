use std::fmt::{
    self,
    Display
};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::{
    BytePattern,
    Width,
    Offset
};

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum OperationSize
{
    Zero    = 0b_00001,
    One     = 0b_00010,
    Two     = 0b_00100,
    Three   = 0b_01000,
    Four    = 0b_10000,
}

impl Display for OperationSize
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let out = match self
        {
            Self::Zero => "0",
            Self::One => "1",
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
        };

        write!(f, "{}", out)
    }
}

impl From<u8> for OperationSize
{
    fn from(num: u8) -> Self
    {
        OperationSize::from_u8(num).expect("OperationSize can't be converted from a supplied number!")
    }
}

impl From<OperationSize> for u8
{
    fn from(size: OperationSize) -> u8
    {
        size as u8
    }
}

impl BytePattern for OperationSize
{
    const WIDTH: Width = Width::Five;
    const OFFSET: Offset = Offset::Three;
}