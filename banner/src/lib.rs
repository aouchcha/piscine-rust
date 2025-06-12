use std::{collections::HashMap, num::ParseFloatError};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Flag {
    pub short_hand : String,
    pub long_hand : String,
    pub desc : String
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        let short_hand = format!("-{}",name.chars().nth(0).unwrap());
        let long_hand = format!("--{}",name);
        Flag {
            short_hand,
            long_hand,
            desc : d.to_string()
        }
    }
}
   
pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<Flag, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.entry(flag).or_insert(func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        // println!("{:?}", self.flags);
        for (key, value) in &self.flags {
            if input == key.short_hand || input == key.long_hand {
                match value(argv[0], argv[1]) {
                    Ok(r) => return Ok(r),
                    Err(_) => return Err("invalid float literal".to_string())
                }
            };
        }
        Ok(String::new())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1 : f64 = a.parse()?;
    let num2 : f64 = b.parse()?;
    // if num2 == 0.0 {
    //     return Err(DivisionError::DivisionByZero);
    // }
    let result = num1 as f64 / num2 as f64;
    Ok(format!("{}", result))
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    // println!("{}",a);
    // println!("{}",b);

    let num1 : f64 = a.parse()?;
    let num2 : f64 = b.parse()?;
    let result = num1 as f64 % num2 as f64;
    Ok(format!("{}", result))
}