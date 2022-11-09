use std::ffi::{CStr, CString};

use js_sys::{JsString, Number};

use crate::{sys, HighsPtr};

pub trait HighsOptionValue {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number;
}

impl HighsOptionValue for bool {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number {
        sys::Highs_setBoolOptionValue(highs, option, if self { 1 } else { 0 })
    }
}

impl HighsOptionValue for i32 {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number {
        sys::Highs_setIntOptionValue(highs, option, self)
    }
}

impl HighsOptionValue for f64 {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number {
        sys::Highs_setDoubleOptionValue(highs, option, self)
    }
}

impl<'a> HighsOptionValue for &'a CStr {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number {
        sys::Highs_setStringOptionValue(highs, option, self.as_ptr())
    }
}

impl<'a> HighsOptionValue for &'a [u8] {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number {
        CString::new(self)
            .expect("invalid highs option value")
            .apply_to_highs(highs, option)
    }
}

impl<'a> HighsOptionValue for &'a str {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number {
        self.as_bytes().apply_to_highs(highs, option)
    }
}
