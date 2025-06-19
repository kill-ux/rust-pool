// use std::collections::HashMap;
// use std::num::ParseFloatError;

// pub struct Flag {
//     pub short_hand: String,
//     pub long_hand: String,
//     pub desc: String,
// }

// impl Flag {
//     pub fn opt_flag(name: &str, d: &str) -> Self {
//         Self {
//             short_hand: format!("-{}", name.chars().next().unwrap()),
//             long_hand: format!("--{}", name),
//             desc: d.to_string(),
//         }
//     }
// }

// pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

// pub struct FlagsHandler {
//     pub flags: HashMap<(String, String), Callback>,
// }

// impl FlagsHandler {
//     pub fn add_flag(
//         &mut self,
//         flag: Flag,
//         func: fn(&str, &str) -> Result<String, ParseFloatError>
//     ) {
//         // todo!()
//         self.flags.insert((flag.short_hand.clone(), flag.long_hand.clone()), func);
//     }

//     pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
//         for (key, value) in self.flags.iter() {
//             if key.0 == input || key.1 == input {
//                 return match value(argv[0], argv[1]) {
//                     Ok(res) => Ok(res),
//                     Err(err) => Err(err.to_string())
//                 };
//             }
//         }
//         Err("NotFound 404".to_string())
//     }
// }

// pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
//     let num1 = match a.parse::<f64>() {
//         Ok(a) => a,
//         Err(err) => {
//             return Err(err);
//         }
//     };

//     let num2 = match b.parse::<f64>() {
//         Ok(b) => b,
//         Err(err) => {
//             return Err(err);
//         }
//     };

//     Ok((num1 / num2).to_string())
// }

// pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
//     let num1 = match a.parse::<f64>() {
//         Ok(a) => a,
//         Err(err) => {
//             return Err(err);
//         }
//     };

//     let num2 = match b.parse::<f64>() {
//         Ok(b) => b,
//         Err(err) => {
//             return Err(err);
//         }
//     };

//     Ok((num1 % num2).to_string())
// }

use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self {
            short_hand: format!("-{}", &name[0..1]),
            long_hand: format!("-{name}"),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let mut key = input;
        if input.starts_with("--") {
            key = &input[1..3];
        }
        let callback = self.flags.get(key).unwrap();
        match callback(argv[0], argv[1]) {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = match a.parse::<f64>() {
        Ok(n) => n,
        Err(err) => {
            return Err(err);
        }
    };

    let b = match b.parse::<f64>() {
        Ok(n) => n,
        Err(err) => {
            return Err(err);
        }
    };

    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = match a.parse::<f64>() {
        Ok(n) => n,
        Err(err) => {
            return Err(err);
        }
    };

    let b = match b.parse::<f64>() {
        Ok(n) => n,
        Err(err) => {
            return Err(err);
        }
    };

    Ok((a % b).to_string())
}
