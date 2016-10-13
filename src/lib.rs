mod encode;
mod decode;

pub use encode::encode_str;
pub use encode::Mode;
pub use decode::decode_str;

#[cfg(test)]
mod tests {
    use super::*;
    use std::char;

    #[test]
    fn test_all_chars() {
        let test_char = |c| {
            let expect: String = [c].iter().cloned().collect();
            let b = encode_str(expect.chars(), Mode::AddOne);
            let s = decode_str(b);
            assert_eq!(s, expect);
        };

        for c in 0..0xd800 {
            test_char(char::from_u32(c).unwrap());
        }
        // slow
        // for c in 0xf000 .. 0x110000 {
        //     test_char(char::from_u32(c).unwrap());
        // }
    }
}
