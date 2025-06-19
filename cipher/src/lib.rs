// #[derive(Debug, PartialEq)]
// pub struct CipherError {
//     pub expected: String,
// }

// pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
//     if original.is_empty() && !ciphered.is_empty() {
//         return Err(CipherError { expected: String::new() });
//     }

//     let mut new_vec = vec![];
//     for (i, &b) in original.as_bytes().iter().enumerate() {
//         let mut num = b;
//         if b >= 65 && b <= 90 {
//             num = 90 - (b - 65);
//         } else if b >= 97 && b <= 122 {
//             num = 122 - (b - 97);
//         }
//         new_vec.push(num);
//     }

//     if original.len() != ciphered.len() {
//         return Err(CipherError { expected: String::from_utf8(new_vec).unwrap() });
//     }

//     for (i, &a) in ciphered.as_bytes().iter().enumerate() {
//         if a != new_vec[i] {
//             return Err(CipherError { expected: String::from_utf8(new_vec).unwrap() });
//         }
//     }
//     Ok(())
// }

#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut res = String::new();
    for ch in original.chars() {
        let mut alpha = ch as u8;
        if ch.is_alphabetic() {
            alpha = b'z' - ch as u8 + b'a';
            if ch.is_uppercase() {
                alpha = b'Z' - ch as u8 + b'A';
            }
        }
        res.push(alpha as char);
    }
    if res == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected: res })
    }
}
