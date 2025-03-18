use super::mem::Memory;

#[derive(Default)]
pub struct CPU {
    a: u8,
    x: u8,
    y: u8,

    pc: u16,
    stack_ptr: u8,
    s: u8, // only 6 bits are used,

    mem: Memory
}