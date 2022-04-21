use castle_error::CastleError;
use tokenizer::{Tokenizable, extensions::ExpectPunctuator, Punctuator, TokenKind};


pub fn parse_comments(tokenizer: &mut impl Tokenizable) -> Result<(), CastleError>{
    tokenizer.expect_punctuator(Punctuator::DoubleSlash, true);
    loop {
        let peek = tokenizer.peek_expect(false)?;
        if peek.kind == TokenKind::LineTerminator {
            tokenizer.next(false)?;
            break;
        } else {
            tokenizer.next(false)?;
        }
    }
    Ok(())
}