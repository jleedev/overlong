// Cribbed from libcore/char.rs

use std::error::Error;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum Mode {
    Normal,
    AddOne,
    AddTwo,
    MinTwo,
    MinThree,
    Four,
}

impl FromStr for Mode {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Normal" => Ok(Mode::Normal),
            "AddOne" => Ok(Mode::AddOne),
            "AddTwo" => Ok(Mode::AddTwo),
            "MinTwo" => Ok(Mode::MinTwo),
            "MinThree" => Ok(Mode::MinThree),
            "Four" => Ok(Mode::Four),
            _ => {
                let mut out = String::new();
                write!(out, "invalid mode {:?}", s).unwrap();
                Err(From::from(out))
            }
        }
    }
}

// UTF-8 ranges and tags for encoding characters
const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;
const MAX_ONE_B: u32 = 0x7f;
const MIN_TWO_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x7ff;
const MIN_THREE_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0xffff;
const MIN_FOUR_B: u32 = 0x10000;
const MAX: u32 = 0x10ffff;

pub fn encode_char(c: char, m: Mode, dst: &mut [u8]) -> &[u8] {
    assert!(dst.len() >= 4);
    let code = c as u32;
    let code_for_len = match (m, c as u32) {
        (Mode::Normal, _) => code,
        (Mode::AddOne, 0...MAX_ONE_B) => MAX_TWO_B,
        (Mode::AddOne, MIN_TWO_B...MAX_TWO_B) => MAX_THREE_B,
        (Mode::AddOne, _) => MAX,
        (Mode::AddTwo, 0...MAX_ONE_B) => MAX_THREE_B,
        (Mode::AddTwo, _) => MAX,
        (Mode::MinTwo, 0...MAX_ONE_B) => MAX_TWO_B,
        (Mode::MinTwo, _) => code,
        (Mode::MinThree, MIN_FOUR_B...MAX) => code,
        (Mode::MinThree, _) => MIN_THREE_B,
        (Mode::Four, _) => MAX,
    };

    let len = if code_for_len <= MAX_ONE_B {
        dst[0] = code as u8;
        1
    } else if code_for_len <= MAX_TWO_B {
        dst[0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
        dst[1] = (code & 0x3F) as u8 | TAG_CONT;
        2
    } else if code_for_len <= MAX_THREE_B {
        dst[0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
        dst[1] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        dst[2] = (code & 0x3F) as u8 | TAG_CONT;
        3
    } else {
        dst[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
        dst[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
        dst[2] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        dst[3] = (code & 0x3F) as u8 | TAG_CONT;
        4
    };
    &dst[..len]
}

pub fn encode_str<I: Iterator<Item = char>>(s: I, m: Mode) -> Vec<u8> {
    let mut result = Vec::new();
    let mut buf = [0u8; 4];
    for c in s {
        result.extend(encode_char(c, m, &mut buf));
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::encode_char;
    use super::encode_str;
    use super::Mode::*;

    #[test]
    fn test_char() {
        let mut buf = [0u8; 4];
        assert_eq!(encode_char('A', Normal, &mut buf), b"A");
        assert_eq!(encode_char('A', AddOne, &mut buf), b"\xc1\x81");
    }

    #[test]
    fn test_str() {
        let input = "$¢€𐍈";
        let do_test = |mode, actual: &[u8]| assert_eq!(&encode_str(input.chars(), mode)[..], actual);
        do_test(Normal, &input.as_bytes());
        do_test(AddOne,
                b"\xc0\xa4\xe0\x82\xa2\xf0\x82\x82\xac\xf0\x90\x8d\x88");
        do_test(AddTwo,
                b"\xe0\x80\xa4\xf0\x80\x82\xa2\xf0\x82\x82\xac\xf0\x90\x8d\x88");
        do_test(MinTwo, b"\xc0\xa4\xc2\xa2\xe2\x82\xac\xf0\x90\x8d\x88");
        do_test(MinThree,
                b"\xe0\x80\xa4\xe0\x82\xa2\xe2\x82\xac\xf0\x90\x8d\x88");
        do_test(Four,
                b"\xf0\x80\x80\xa4\xf0\x80\x82\xa2\xf0\x82\x82\xac\xf0\x90\x8d\x88");
    }
}
