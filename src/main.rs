mod encode;
mod decode;

use encode::*;
use std::io::Write;

fn p2<'a, I: Iterator<Item = &'a u8>>(it: I) {
    for b in it {
        std::io::stdout().write(&[*b]).unwrap();
    }
}

fn all(s: &str) {
    p2(encode_str(s.chars(), Mode::Normal).iter());
    p2(encode_str(s.chars(), Mode::AddOne).iter());
    p2(encode_str(s.chars(), Mode::AddTwo).iter());
    p2(encode_str(s.chars(), Mode::MinTwo).iter());
    p2(encode_str(s.chars(), Mode::MinThree).iter());
    p2(encode_str(s.chars(), Mode::Four).iter());
}

fn main() {
    all("$¢€𐍈");
    all("nerd sniped");
    all("ｎｅｒｄ　ｓｎｉｐｅ");
}
