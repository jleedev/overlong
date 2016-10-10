#[derive(Clone,Copy)]
pub enum HowLong {
    /// Legal utf-8, mainly for testing
    JustRight,
    /// Increase the length by (at most) one byte
    OneTooLong,
    /// Increase the length by (at most) two bytes
    TwoTooLong,
    /// Increase the length of one-byte chars only
    AtLeastTwoBytes,
    /// Increase the length of one- and two-byte chars only
    AtLeastThreeBytes,
    /// Four byte overlong to the max
    MaximumLongness,
}
use self::HowLong::*;

#[derive(Clone,Copy)]
enum NumBytes {
    One,
    Two,
    Three,
    Four,
}
use self::NumBytes::*;

fn succ(n: NumBytes) -> NumBytes {
    match n {
        One => NumBytes::Two,
        Two => NumBytes::Three,
        _ => NumBytes::Four,
    }
}

fn overlong_length(mode: HowLong, n: NumBytes) -> NumBytes {
    match (mode, n) {
        (JustRight, _) => n,
        (OneTooLong, _) => succ(n),
        (TwoTooLong, _) => succ(succ(n)),
        (AtLeastTwoBytes, One) => Two,
        (AtLeastTwoBytes, _) => n,
        (AtLeastThreeBytes, Four) => Four,
        (AtLeastThreeBytes, _) => Three,
        (MaximumLongness, _) => Four,
    }
}

fn regular_utf8_bytes(c: char) -> NumBytes {
    match c {
        '\u{0000}'...'\u{007f}' => One,
        '\u{0080}'...'\u{07ff}' => Two,
        '\u{0800}'...'\u{ffff}' => Three,
        '\u{10000}'...'\u{10ffff}' => Four,
        _ => unreachable!("issue #12483 in rust"),
    }
}

fn pattern(n: NumBytes) -> [u8; 4] {
    match n {
        One => *b"\0\0\0\0",
        Two => *b"\0\0\xc0\x80",
        Three => *b"\0\xe0\x80\x80",
        Four => *b"\xf0\x80\x80\x80",
    }
}

fn install_bits(c: char, ret: &mut [u8; 4], n: NumBytes) {
    let c = c as u32;
    match n {
        One => {
            ret[3] |= (c & 0x7f) as u8;
        },
        Two => {
            ret[3] |= (c & 0x3f) as u8;
            ret[2] |= ((c >> 6) & 0x1f) as u8;
        },
        Three => {
            ret[3] |= (c & 0x3f) as u8;
            ret[2] |= ((c >> 6) & 0x3f) as u8;
            ret[1] |= ((c >> 12) & 0x0f) as u8;
        },
        Four => {
            ret[3] |= (c & 0x3f) as u8;
            ret[2] |= ((c >> 6) & 0x3) as u8;
            ret[1] |= ((c >> 12) & 0x3f) as u8;
            ret[0] |= ((c >> 18) & 0x07) as u8;
        },
    }
}

/// Returns up to 4 bytes, front-padded with nulls.
fn encode_char(mode: HowLong, c: char) -> Vec<u8> {
    let n = overlong_length(mode, regular_utf8_bytes(c));
    let mut ret = pattern(n);
    install_bits(c, &mut ret, n);
    match n {
        One => vec![ret[3]],
        Two => vec![ret[2], ret[3]],
        Three => vec![ret[1], ret[2], ret[3]],
        Four => vec![ret[0], ret[1], ret[2], ret[3]],
    }
}

pub fn encode_str(mode: HowLong, s: &[char]) -> Vec<u8> {
    s.iter().cloned().flat_map(|c| encode_char(mode, c).into_iter()).collect()
}

#[cfg(test)]
mod tests {
    use super::encode_str;
    use super::HowLong::*;
    #[test]
    fn it_works() {
        let s: Vec<char> = "I prefer VTF-8overbyte".chars().collect();
        let b = b"\xc1\x89\xc0\xa0\xc1\xb0\xc1\xb2\xc1\xa5\xc1\xa6\xc1\xa5\xc1\xb2\xc0\xa0\xc1\x96\xc1\x94\xc1\x86\xc0\xad\xc0\xb8\xc1\xaf\xc1\xb6\xc1\xa5\xc1\xb2\xc1\xa2\xc1\xb9\xc1\xb4\xc1\xa5";
        println!("{:?}", encode_str(OneTooLong, &s));
        assert_eq!(encode_str(OneTooLong, &s), Vec::from(&b[..]));
    }
}
