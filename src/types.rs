mod instruction;
mod opcode;
mod addressing_mode;
mod operation_size;

pub use instruction::Instruction;
pub use opcode::Opcode;
pub use addressing_mode::AddressingMode;
pub use operation_size::OperationSize;

/// 8 - width + offset >= 0
pub trait BytePattern: Copy + From<u8>
{
    /// How many bits to extract.
    const WIDTH: Width;
    /// How far of an offset from the right to mask against.
    const OFFSET: Offset;

    fn width() -> u8
    {
        Self::WIDTH as u8
    }

    fn offset() -> u8
    {
        Self::OFFSET as u8
    }
}

pub enum Width
{
    One     = 1,
    Two     = 2,
    Three   = 3,
    Four    = 4,
    Five    = 5,
    Six     = 6,
    Seven   = 7,
    Eight   = 8
}

pub enum Offset
{
    Zero    = 0,
    One     = 1,
    Two     = 2,
    Three   = 3,
    Four    = 4,
    Five    = 5,
    Six     = 6,
    Seven   = 7,
    Eight   = 8
}