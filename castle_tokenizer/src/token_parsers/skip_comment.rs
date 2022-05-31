use std::io::Read;
use castle_input_cursor::Cursor;
use castle_types::CastleError;

pub(crate) fn skip_comment(cursor: &mut Cursor<impl Read>) -> Result<(), CastleError> {
    // while the next byte is not a newline or EOF
    loop {
        match cursor.next_byte()? {
            Some(b'\n') | Some(b'\r') | None => break Ok(()),
            _ => continue
        }
    }
}