#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem;

use js_sys::{JsString, Number, Object, Reflect, Uint8Array};
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
    pub fn Highs_getSolution(h: Number, c: Number, r: Number) -> Object;
    #[wasm_bindgen]
    pub fn Highs_passLp(
        h: Number,
        nc: Number,
        nr: Number,
        nnz: Number,
        afmt: Number,
        sense: Number,
        offset: Number,
        cc: &Uint8Array,
        cl: &Uint8Array,
        cu: &Uint8Array,
        rl: &Uint8Array,
        ru: &Uint8Array,
        ast: &Uint8Array,
        aidx: &Uint8Array,
        av: &Uint8Array,
    ) -> Number;
    #[wasm_bindgen]
    pub fn Highs_passMip(
        h: Number,
        nc: Number,
        nr: Number,
        nnz: Number,
        afmt: Number,
        sense: Number,
        offset: Number,
        cc: &Uint8Array,
        cl: &Uint8Array,
        cu: &Uint8Array,
        rl: &Uint8Array,
        ru: &Uint8Array,
        ast: &Uint8Array,
        aidx: &Uint8Array,
        av: &Uint8Array,
        int: &Uint8Array,
    ) -> Number;
    #[wasm_bindgen]
    pub fn Highs_setBoolOptionValue(h: Number, o: &JsString, v: Number) -> Number;
    #[wasm_bindgen]
    pub fn Highs_setDoubleOptionValue(h: Number, o: &JsString, v: Number) -> Number;
    #[wasm_bindgen]
    pub fn Highs_setIntOptionValue(h: Number, o: &JsString, v: Number) -> Number;
    #[wasm_bindgen]
    pub fn Highs_setStringOptionValue(h: Number, o: &JsString, v: &JsString) -> Number;
    #[wasm_bindgen]
    pub fn Highs_run(h: Number) -> Number;
}

pub fn c(n: usize) -> Number {
    Number::from(n as u32)
}

pub fn c_array_f64(v: &Vec<f64>) -> Uint8Array {
    let array = Uint8Array::new_with_length((v.len() * mem::size_of::<f64>()) as u32);
    let bytes: Vec<u8> = v.iter().flat_map(|i| f64::to_ne_bytes(*i)).collect();
    array.copy_from(&bytes);
    array
}

pub fn c_array_i32(v: &Vec<i32>) -> Uint8Array {
    let array = Uint8Array::new_with_length((v.len() * mem::size_of::<i32>()) as u32);
    let bytes: Vec<u8> = v.iter().flat_map(|i| i32::to_ne_bytes(*i)).collect();
    array.copy_from(&bytes);
    array
}

pub fn Highs_getSolution_wrap(
    h: Number,
    cols: usize,
    rows: usize,
) -> (HighsInt, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let o = Highs_getSolution(
        h,
        Number::from((cols * mem::size_of::<f64>()) as u32),
        Number::from((rows * mem::size_of::<f64>()) as u32),
    );
    let ret = Reflect::get(&o, &JsValue::from("ret")).unwrap();
    let mut colvalue: Vec<f64> = Vec::with_capacity(cols);
    let mut coldual: Vec<f64> = Vec::with_capacity(cols);
    let mut rowvalue: Vec<f64> = Vec::with_capacity(rows);
    let mut rowdual: Vec<f64> = Vec::with_capacity(rows);
    if !ret.is_truthy() {
        let mut buff: [u8; mem::size_of::<f64>()] = Default::default();
        let cv = Uint8Array::from(Reflect::get(&o, &JsValue::from("cv")).unwrap());
        for c in cv.to_vec().chunks_exact(mem::size_of::<f64>()) {
            buff.copy_from_slice(c);
            colvalue.push(f64::from_ne_bytes(buff));
        }
        let cd = Uint8Array::from(Reflect::get(&o, &JsValue::from("cd")).unwrap());
        for c in cd.to_vec().chunks_exact(mem::size_of::<f64>()) {
            buff.copy_from_slice(c);
            coldual.push(f64::from_ne_bytes(buff));
        }
        let rv = Uint8Array::from(Reflect::get(&o, &JsValue::from("rv")).unwrap());
        for c in rv.to_vec().chunks_exact(mem::size_of::<f64>()) {
            buff.copy_from_slice(c);
            rowvalue.push(f64::from_ne_bytes(buff));
        }
        let rd = Uint8Array::from(Reflect::get(&o, &JsValue::from("rd")).unwrap());
        for c in rd.to_vec().chunks_exact(mem::size_of::<f64>()) {
            buff.copy_from_slice(c);
            rowdual.push(f64::from_ne_bytes(buff));
        }
    }
    (
        ret.as_f64().unwrap() as HighsInt,
        colvalue,
        coldual,
        rowvalue,
        rowdual,
    )
}
