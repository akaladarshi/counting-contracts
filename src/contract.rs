pub mod query {
    use crate::msg::{IncrementResp, ValueResp};

    pub fn value() -> ValueResp {
        ValueResp { value: 0 }
    }

    pub fn increment(mut i: u8) -> IncrementResp {
        i += 1;
       IncrementResp { value: i }
    }
}