use num::One;
use num::Zero;
use std::ops::Add;
use std::ops::Not;
use vm::cpu::nibble::Nibble;

#[derive(Copy, Clone)]
pub(crate) struct AdderResult<T: Copy + Clone> {
    pub value: T,
    pub half_carry: bool,
    pub carry: bool,
}

pub(crate) fn get_bytes(value: u16) -> (u8, u8) {
    let high = ((value & 0xF0) >> 8) as u8;
    let low = (value & 0x0F) as u8;
    (high, low)
}

pub(crate) fn get_word(high: u8, low: u8) -> u16 {
    (high << 8) as u16 | low as u16
}

pub(crate) fn get_bit<T: Zero + One>(value: bool) -> T {
    if value {
        num::one()
    } else {
        num::zero()
    }
}

pub(crate) fn negate<T: Add<Output = T> + Not<Output = T> + One>(value: T) -> T {
    !value + num::one()
}

pub(crate) fn add_bytes(a: u8, b: u8) -> AdderResult<u8> {
    let (low_nibble, half_carry) = Nibble::from_u8(a).overflowing_add(Nibble::from_u8(b));
    let (high_nibble_temp, part_carry_a) =
        Nibble::from_u8_high(a).overflowing_add(get_bit::<Nibble>(half_carry));
    let (high_nibble, part_carry_b) = high_nibble_temp.overflowing_add(Nibble::from_u8_high(b));
    let carry = part_carry_a | part_carry_b;
    AdderResult {
        value: Nibble::u8_from_nibbles(high_nibble, low_nibble),
        half_carry: half_carry,
        carry: carry,
    }
}

pub(crate) fn add_words(a: u16, b: u16) -> AdderResult<u16> {
    let low = {
        let (_, op1) = get_bytes(a);
        let (_, op2) = get_bytes(b);
        add_bytes(op1, op2)
    };
    let high = {
        let (op1, _) = get_bytes(a);
        let (op2, _) = get_bytes(b);
        add_bytes(op1 + get_bit::<u8>(low.carry), op2)
    };
    AdderResult {
        value: get_word(high.value, low.value),
        half_carry: high.half_carry,
        carry: high.carry,
    }
}
