use js_sys::{JsString, Number};

use crate::{sys, HighsPtr};

pub trait HighsOptionValue {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number;
}

impl HighsOptionValue for bool {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number {
        let n = Number::from(if self { 1 } else { 0 });
        sys::Highs_setBoolOptionValue(highs.ptr(), *option, n)
    }
}

impl HighsOptionValue for i32 {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number {
        let n = Number::from(self);
        sys::Highs_setIntOptionValue(highs.ptr(), *option, n)
    }
}

impl HighsOptionValue for f64 {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number {
        let n = Number::from(self);
        sys::Highs_setDoubleOptionValue(highs.ptr(), *option, n)
    }
}

impl<'a> HighsOptionValue for &'a str {
    fn apply_to_highs(self, highs: HighsPtr, option: &JsString) -> Number {
        let s = JsString::from(self);
        sys::Highs_setStringOptionValue(highs.ptr(), *option, s)
    }
}
