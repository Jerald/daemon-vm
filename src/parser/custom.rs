use crate::types::{
    Instruction,
    Opcode,
    AddressingMode,
    OperationSize,

    BytePattern,
};

const BYTE_MAX: u8 = 0xFF;
const BYTE_LEN: u8 = 8;

struct ByteMatcher
{
    inner: Vec<u8>,
    index: usize,
}

impl ByteMatcher
{
    pub fn new(inner: Vec<u8>) -> Self
    {
        ByteMatcher {
            inner,
            index: 0
        }
    }

    pub fn match_byte<T: BytePattern>(&self) -> T
    {
        let byte = self.inner[self.index] >> T::offset();
        let mask = BYTE_MAX >> (BYTE_LEN - T::width());

        let value = byte & mask;

        T::from(value)
    }

    pub fn get_byte(&self) -> u8
    {
        self.inner[self.index]
    }

    pub fn advance(&mut self)
    {
        self.index += 1;
    }
}

pub fn parse_instruction(input: &[u8]) -> Instruction
{
    let mut matcher = ByteMatcher::new(Vec::from(input));

    let opcode = matcher.match_byte::<Opcode>();
    let l_field_mode = matcher.match_byte::<AddressingMode>();

    matcher.advance();

    let l_field = matcher.get_byte() as i8;

    matcher.advance();

    let size = matcher.match_byte::<OperationSize>();
    let r_field_mode = matcher.match_byte::<AddressingMode>();

    matcher.advance();

    let r_field = matcher.get_byte() as i8;

    Instruction {
        opcode,

        l_field_mode,
        l_field,

        size,

        r_field_mode,
        r_field
    }
}