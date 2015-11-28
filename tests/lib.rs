extern crate vt100;

use vt100::terminal::Term;
use vt100::emulator::VtEmulator;
use vt100::ascii;

use std::str::Chars;

fn do_test(stream: Chars)->Vec<Term> {
    VtEmulator::new(stream).collect()
}

#[test]
fn test_beep() {
    let mut s = "This is a beep".to_string();
    s.push(ascii::BEL);
    assert_eq!(do_test(s.chars()), [Term::Chars("This is a beep".to_string()), Term::Beep]);
}

#[test]
fn test_insert_blank_characters() {
    let mut s = String::new();
    s.push(ascii::ESC);
    s.push_str("[3;@");
    assert_eq!(do_test(s.chars()), [Term::InsertBlankCharacters(3)]);
}
