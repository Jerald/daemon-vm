use nom::{
    IResult,
    error::{
        ErrorKind,
    },
    bits::{
        complete::{
            take
        },
        bits,
    },
    sequence::{
        tuple,
    },
};

use crate::types::Instruction;

pub type ParsedTuple = (u8, u8, u8, u8, u8, u8);

pub fn parse_binary_to_tuple<'a>(input: &'a [u8]) -> IResult<&'a [u8], ParsedTuple>
{
    let take_opcode = take(5u8);

    let take_l_field_mode = take(3u8);
    let take_l_field = take(8u8);

    let take_operation_size = take(5u8);

    let take_r_field_mode = take(3u8);
    let take_r_field = take(8u8);

    // This is needed because Nom does bad things with error types...
    type TupleError<'a> = ((&'a [u8], usize), ErrorKind);
    let parse_bits = tuple::<_, _, TupleError<'a>, _>((
        take_opcode,

        take_l_field_mode,
        take_l_field,

        take_operation_size,

        take_r_field_mode,
        take_r_field));

    bits(parse_bits)(input)
}

pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction>
{
    let (input, (opcode, l_field_mode, l_field, size, r_field_mode, r_field)) = parse_binary_to_tuple(input)?;

    let inst = Instruction {
        opcode: opcode.into(),

        l_field_mode: l_field_mode.into(),
        l_field: l_field as i8,

        size: size.into(),

        r_field_mode: r_field_mode.into(),
        r_field: r_field as i8,
    };

    Ok((input, inst))
}