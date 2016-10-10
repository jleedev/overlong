mod lib;
use lib::encode_str;
use lib::HowLong::*;

pub fn to_hex_string(bytes: &[u8]) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
  strs.connect(" ")
}

fn p(x: &[u8]) {
    println!("{}", to_hex_string(x));
}

fn main() {
    let s: Vec<char> = "nerd sniped".chars().collect();
    p(&encode_str(OneTooLong, &s)[..]);
    p(&encode_str(TwoTooLong, &s)[..]);
    p(&encode_str(MaximumLongness, &s));
    println!("");
    let s: Vec<char> = "ｎｅｒｄ　ｓｎｉｐｅ".chars().collect();
    p(&encode_str(OneTooLong, &s)[..]);
    p(&encode_str(TwoTooLong, &s)[..]);
    p(&encode_str(MaximumLongness, &s)[..]);
}
