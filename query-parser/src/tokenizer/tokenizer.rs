use std::{io::Read, collections::VecDeque};

use input_cursor::Cursor;

use crate::token::Token;


pub struct Tokenizer<R> {
    cursor: Cursor<R>,
    peeked: VecDeque<Token>
}

impl<R> Tokenizer<R> where R: Read {

    pub fn new(reader: R) -> Self {
        Self {
            cursor: Cursor::new(reader),
            peeked: VecDeque::new()
        }
    }

    pub fn next(&mut self) -> Result<Option<Token>, CastleError>

}