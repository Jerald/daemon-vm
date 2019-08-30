use daemon_vm::parser::{custom, nom_based};

fn main()
{
    let test_input = vec![0x00, 0x05, 0x82, 0x02, 0xFF];

    let inst = nom_based::parse_instruction(&test_input).expect("Nom did bad things...").1;
    println!("Nom parsed instruction: {:?}", inst);
    println!("Nom parsed instruction as asm: {}", inst);

    let inst = custom::parse_instruction(&test_input);
    println!("Custom parsed instruction: {:?}", inst);
    println!("Custom parsed instruction as asm: {}", inst);
}
