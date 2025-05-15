#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    if ciphered.is_empty() {
        return Err(CipherError { expected: String::from_utf8("") });
    }
    let mut new_vec = vec![];
    for (i, &b) in original.as_bytes().iter().enumerate() {
        let mut num = b;
        if b >= 65 && b <= 90 {
            num = 90 - (b - 65);
        } else if b >= 97 && b <= 122 {
            num = 122 - (b - 97);
        }
        new_vec.push(num);
    }

    for (i, &a) in ciphered.as_bytes().iter().enumerate() {
        if a != new_vec[i] {
            return Err(CipherError { expected: String::from_utf8(new_vec).unwrap() });
        }
    }
    Ok(())
}
