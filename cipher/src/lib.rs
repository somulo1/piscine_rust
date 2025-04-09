#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected = original
        .chars()
        .map(|c| match c {
            'A'..='Z' => ((b'Z' + b'A') - (c as u8)) as char,
            'a'..='z' => ((b'z' + b'a') - (c as u8)) as char,
            _ => c,
        })
        .collect::<String>();

    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}
