#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem;

use js_sys::{JsString, Number, Uint8Array};
use wasm_bindgen::prelude::*;

// Lifted from highs-sys
pub type HighsInt = i32;

pub const MODEL_STATUS_NOTSET: HighsInt = 0;
pub const MODEL_STATUS_LOAD_ERROR: HighsInt = 1;
pub const MODEL_STATUS_MODEL_ERROR: HighsInt = 2;
pub const MODEL_STATUS_PRESOLVE_ERROR: HighsInt = 3;
pub const MODEL_STATUS_SOLVE_ERROR: HighsInt = 4;
pub const MODEL_STATUS_POSTSOLVE_ERROR: HighsInt = 5;
pub const MODEL_STATUS_MODEL_EMPTY: HighsInt = 6;
pub const MODEL_STATUS_OPTIMAL: HighsInt = 7;
pub const MODEL_STATUS_INFEASIBLE: HighsInt = 8;
pub const MODEL_STATUS_UNBOUNDED_OR_INFEASIBLE: HighsInt = 9;
pub const MODEL_STATUS_UNBOUNDED: HighsInt = 10;
pub const MODEL_STATUS_OBJECTIVE_BOUND: HighsInt = 11;
pub const MODEL_STATUS_OBJECTIVE_TARGET: HighsInt = 12;
pub const MODEL_STATUS_REACHED_TIME_LIMIT: HighsInt = 13;
pub const MODEL_STATUS_REACHED_ITERATION_LIMIT: HighsInt = 14;
pub const MODEL_STATUS_UNKNOWN: HighsInt = 15;
pub const MODEL_STATUS_MIN: HighsInt = MODEL_STATUS_NOTSET;
pub const MODEL_STATUS_MAX: HighsInt = MODEL_STATUS_UNKNOWN;

pub const STATUS_OK: HighsInt = 0;
pub const STATUS_WARNING: HighsInt = 1;
pub const STATUS_ERROR: HighsInt = -1;

pub const MATRIX_FORMAT_NONE: HighsInt = 0;
pub const MATRIX_FORMAT_COLUMN_WISE: HighsInt = 1;
pub const MATRIX_FORMAT_ROW_WISE: HighsInt = 2;

pub const OBJECTIVE_SENSE_MINIMIZE: HighsInt = 1;
pub const OBJECTIVE_SENSE_MAXIMIZE: HighsInt = -1;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub fn Highs_create() -> Number;
    #[wasm_bindgen]
    pub fn Highs_changeObjectiveSense(h: Number, s: Number) -> Number;
    #[wasm_bindgen]
    pub fn Highs_destroy(h: Number);
    #[wasm_bindgen]
    pub fn Highs_getNumCols(h: Number) -> Number;
    #[wasm_bindgen]
    pub fn Highs_getNumRows(h: Number) -> Number;
    #[wasm_bindgen]
    pub fn Highs_getModelStatus(h: Number) -> Number;
    #[wasm_bindgen]
    pub fn Highs_getSolution(
        h: Number,
        cv: &Uint8Array,
        cd: &Uint8Array,
        rv: &Uint8Array,
        rd: &Uint8Array,
    ) -> Number;
    #[wasm_bindgen]
    pub fn Highs_setBoolOptionValue(h: Number, o: JsString, v: Number) -> Number;
    #[wasm_bindgen]
    pub fn Highs_setDoubleOptionValue(h: Number, o: JsString, v: Number) -> Number;
    #[wasm_bindgen]
    pub fn Highs_setIntOptionValue(h: Number, o: JsString, v: Number) -> Number;
    #[wasm_bindgen]
    pub fn Highs_setStringOptionValue(h: Number, o: JsString, v: JsString) -> Number;
    #[wasm_bindgen]
    pub fn Highs_run(h: Number) -> Number;
}

pub fn Highs_getSolution_wrap(
    h: Number,
    cols: usize,
    rows: usize,
) -> (Number, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut cv = Uint8Array::new_with_length((cols * mem::size_of::<f64>()) as u32);
    let mut cd = Uint8Array::new_with_length((cols * mem::size_of::<f64>()) as u32);
    let mut rv = Uint8Array::new_with_length((rows * mem::size_of::<f64>()) as u32);
    let mut rd = Uint8Array::new_with_length((rows * mem::size_of::<f64>()) as u32);
    let ret = Highs_getSolution(h, &mut cv, &mut cd, &mut rv, &mut rd);
    let mut colvalue: Vec<f64> = Vec::with_capacity(cols);
    let mut coldual: Vec<f64> = Vec::with_capacity(cols);
    let mut rowvalue: Vec<f64> = Vec::with_capacity(rows);
    let mut rowdual: Vec<f64> = Vec::with_capacity(rows);
    if !ret.is_truthy() {
        let mut buff: [u8; mem::size_of::<f64>()] = Default::default();
        for c in cv.to_vec().chunks_exact(mem::size_of::<f64>()) {
            buff.copy_from_slice(c);
            colvalue.push(f64::from_ne_bytes(buff));
        }
        for c in cd.to_vec().chunks_exact(mem::size_of::<f64>()) {
            buff.copy_from_slice(c);
            coldual.push(f64::from_ne_bytes(buff));
        }
        for c in rv.to_vec().chunks_exact(mem::size_of::<f64>()) {
            buff.copy_from_slice(c);
            rowvalue.push(f64::from_ne_bytes(buff));
        }
        for c in rd.to_vec().chunks_exact(mem::size_of::<f64>()) {
            buff.copy_from_slice(c);
            rowdual.push(f64::from_ne_bytes(buff));
        }
    }
    (ret, colvalue, coldual, rowvalue, rowdual)
}
