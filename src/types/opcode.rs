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
pub enum Opcode
{
    Add = 0b_00000,
    Sub = 0b_00001,
    Mul = 0b_00010,
    Div = 0b_00011,
    Jmp = 0b_00100,
    Jez = 0b_00101,
    Jnz = 0b_00110,
    Jgz = 0b_00111,
    Jlz = 0b_01000,
    Hcf = 0b_01001,
    Mov = 0b_01010,
}

impl Display for Opcode
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // I just want the name of the enum variant, but lowercase.
        // Surprisingly, that's a bit annoying to do...
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl From<u8> for Opcode
{
    fn from(num: u8) -> Self
    {
        Opcode::from_u8(num).expect("Opcode can't be converted from a supplied number!")
    }
}

impl From<Opcode> for u8
{
    fn from(opcode: Opcode) -> u8
    {
        opcode as u8
    }
}

impl BytePattern for Opcode
{
    const WIDTH: Width = Width::Five;
    const OFFSET: Offset = Offset::Three;
}