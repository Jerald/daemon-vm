use std::fmt::{
    self,
    Display
};

use super::{
    Opcode,
    AddressingMode,
    OperationSize
};

#[derive(Debug, Clone)]
pub struct Instruction
{
    pub opcode: Opcode,

    pub l_field_mode: AddressingMode,
    pub l_field: i8,

    pub size: OperationSize,

    pub r_field_mode: AddressingMode,
    pub r_field: i8,
}

impl Display for Instruction
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}.{} {}{}, {}{}", self.opcode, self.size, self.l_field_mode, self.l_field, self.r_field_mode, self.r_field)
    }
}

