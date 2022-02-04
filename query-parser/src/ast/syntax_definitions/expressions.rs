use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum Expression {
    PrimitiveValue(PrimitiveValue),
}


#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Hash, Eq)]
pub enum PrimitiveValue {
    String(Box<str>),
    Float(F64),
    Int(i64),
    UInt(u64),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Hash, Eq)]
pub struct F64 {
    pub integer_part: i64,
    pub decimal_part: i64,
}

impl F64 {
    pub fn new(f: f64) -> Self {
        let integer_part = f.floor() as i64;
        let decimal_part = ((f - integer_part as f64) * 10_f64.powi(10)) as i64;

        return Self {
            integer_part,
            decimal_part,
        }
    }

    pub fn create_float(self) -> f64 {
        let integer_part = self.integer_part as f64;
        let decimal_part = self.decimal_part as f64;

        return integer_part + decimal_part / 10_f64.powi(10);
    }
}

#[test]
fn test_create_float(){
    let float_struct = F64::new(1.2345);
    let float = float_struct.create_float();
    assert_eq!(float, 1.2345);
}
