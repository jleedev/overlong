// Cribbed from libcore/char.rs

use std::char;
use std::iter;

/// An iterator over an iterator of bytes of the characters the bytes represent
/// as UTF-8
#[derive(Clone, Debug)]
pub struct DecodeUtf8<I: Iterator<Item = u8>>(iter::Peekable<I>);

/// Decodes an `Iterator` of bytes as UTF-8.
#[inline]
pub fn decode_utf8<I: IntoIterator<Item = u8>>(i: I) -> DecodeUtf8<I::IntoIter> {
    DecodeUtf8(i.into_iter().peekable())
}

/// `<DecodeUtf8 as Iterator>::next` returns this for an invalid input sequence.
#[derive(PartialEq, Eq, Debug)]
pub struct InvalidSequence(());

impl<I: Iterator<Item = u8>> Iterator for DecodeUtf8<I> {
    type Item = Result<char, InvalidSequence>;
    #[inline]

    fn next(&mut self) -> Option<Result<char, InvalidSequence>> {
        self.0.next().map(|first_byte| {
            // Emit InvalidSequence according to
            // Unicode §5.22 Best Practice for U+FFFD Substitution
            // http://www.unicode.org/versions/Unicode9.0.0/ch05.pdf#G40630

            // Roughly: consume at least one byte,
            // then validate one byte at a time and stop before the first unexpected byte
            // (which might be the valid start of the next byte sequence).

            let mut code_point;
            macro_rules! first_byte {
                ($mask: expr) => {
                    code_point = u32::from(first_byte & $mask)
                }
            }
            macro_rules! continuation_byte {
                () => { continuation_byte!(0x80...0xBF) };
                ($range: pat) => {
                    match self.0.peek() {
                        Some(&byte @ $range) => {
                            code_point = (code_point << 6) | u32::from(byte & 0b0011_1111);
                            self.0.next();
                        }
                        _ => return Err(InvalidSequence(()))
                    }
                }
            }

            match first_byte {
                0x00...0x7F => {
                    first_byte!(0b1111_1111);
                }
                0xC0...0xDF => {  // 0xC0...0xC1 here are overlong
                    first_byte!(0b0001_1111);
                    continuation_byte!();
                }
                0xE0 => {
                    first_byte!(0b0000_1111);
                    continuation_byte!(0x80...0xBF);  // 0x80...0x9F here are overlong
                    continuation_byte!();
                }
                0xE1...0xEC | 0xEE...0xEF => {
                    first_byte!(0b0000_1111);
                    continuation_byte!();
                    continuation_byte!();
                }
                0xED => {
                    first_byte!(0b0000_1111);
                    continuation_byte!(0x80...0x9F);  // 0xA0..0xBF here are surrogates
                    continuation_byte!();
                }
                0xF0 => {
                    first_byte!(0b0000_0111);
                    continuation_byte!(0x80...0xBF);  // 0x80..0x8F here are overlong
                    continuation_byte!();
                    continuation_byte!();
                }
                0xF1...0xF3 => {
                    first_byte!(0b0000_0111);
                    continuation_byte!();
                    continuation_byte!();
                    continuation_byte!();
                }
                0xF4 => {
                    first_byte!(0b0000_0111);
                    continuation_byte!(0x80...0x8F);  // 0x90..0xBF here are beyond char::MAX
                    continuation_byte!();
                    continuation_byte!();
                }
                _ => return Err(InvalidSequence(()))  // Illegal first byte, overlong, or beyond MAX
            }
            unsafe {
                Ok(char::from_u32_unchecked(code_point))
            }
        })
    }
}

pub fn decode_str<I: IntoIterator<Item = u8>>(i: I) -> String {
    decode_utf8(i).map(|c| c.unwrap_or('�')).collect()
}

#[cfg(test)]
mod tests {
    use super::decode_str;

    #[test]
    fn test_decode() {
        let expect = "$¢€𐍈";
        let do_test = |b: &[u8]| assert_eq!(decode_str(b.iter().cloned()), expect);
        do_test(expect.as_bytes());
        do_test(b"\xc0\xa4\xe0\x82\xa2\xf0\x82\x82\xac\xf0\x90\x8d\x88");
        do_test(b"\xe0\x80\xa4\xf0\x80\x82\xa2\xf0\x82\x82\xac\xf0\x90\x8d\x88");
        do_test(b"\xc0\xa4\xc2\xa2\xe2\x82\xac\xf0\x90\x8d\x88");
        do_test(b"\xe0\x80\xa4\xe0\x82\xa2\xe2\x82\xac\xf0\x90\x8d\x88");
        do_test(b"\xf0\x80\x80\xa4\xf0\x80\x82\xa2\xf0\x82\x82\xac\xf0\x90\x8d\x88");
    }
}
