mod encode;
mod decode;

use encode::*;
use decode::*;

use std::error::Error;
use std::io;
use std::io::{stdin, stdout, stderr, Read, Write};

const USAGE: &'static str = "usage: overlong [MODE]

Converts between valid UTF-8 and UTF-8 with \
                             overlong characters.

standard input: Any valid or overlong UTF-8 \
                             string.
standard output: A valid or overlong UTF-8 string, depending \
                             on MODE.

MODE:
    Normal (default), AddOne, AddTwo, MinTwo, \
                             MinThree, Four
";

fn handle_args() -> Result<Mode, Box<Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        Ok(Mode::Normal)
    } else if args.len() == 1 {
        args[0].parse()
    } else {
        Err(From::from(""))
    }
}

fn communicate<R: Read, W: Write>(mode: Mode, reader: &mut R, writer: &mut W) -> io::Result<()> {
    let mut input = vec![];
    try!(reader.read_to_end(&mut input));
    let s = decode_str(input);
    let out = encode_str(s.chars(), mode);
    try!(writer.write_all(&out));
    Ok(())
}

fn main() {
    match handle_args() {
        Ok(mode) => {
            communicate(mode, &mut stdin(), &mut stdout()).unwrap();
        }
        Err(err) => {
            writeln!(stderr(), "{}", err).unwrap();
            writeln!(stderr(), "{}", USAGE).unwrap();
            std::process::exit(1);
        }
    }
}
