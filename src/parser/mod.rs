mod states;
pub mod codes;
pub mod style;

use std::collections::VecDeque;
use std::str::Chars;
use std::cell::RefCell;
use std::rc::Rc;

pub use self::codes::Code;
use self::states::*;

pub struct VtParser<'a> {
    term_actions: Rc<RefCell<VecDeque<Code>>>,
    stream: Rc<RefCell<Chars<'a>>>,
    state: Box<State>
}

impl<'a> VtParser<'a> {
    pub fn new(stream: Chars<'a>) -> VtParser<'a> {
        VtParser {
            term_actions: Rc::new(RefCell::new(VecDeque::new())),
            stream: Rc::new(RefCell::new(stream)),
            state: Box::new(Ground)
        }
    }

    pub fn emit(&self, term:Code) {
        self.term_actions.borrow_mut().push_back(term);
    }

    pub fn error(&self, chars: &[char]) {
        self.emit(Code::Error(format!("Unhandled sequnce: {}", chars.iter().cloned().collect::<String>())));
    }

    pub fn error_msg(&self, msg: String) {
       self.emit(Code::Error(msg))
    }

    pub fn get(&self)->Option<Code> {
        self.term_actions.borrow_mut().pop_front().clone()
    }

}

impl<'a> Iterator for VtParser<'a> {
    type Item = Code;

    fn next(&mut self) -> Option<Code> {
        loop {
            match self.get() {
                Some(x) => {
                    return Some(x)
                },
                None => match self.state.next(self) {
                    Some(x) => self.state = x,
                    None => break
                }
            }

        }

        None
    }
}

pub trait State {
    fn next(&self, emu: &VtParser) -> Option<Box<State>>;
}
