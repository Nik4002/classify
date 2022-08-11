use crate::{Bin,Classification};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JSBin{
   pub bin_start:f64,
   pub bin_end:f64,
   pub count:i64
}

#[wasm_bindgen]
pub struct JSClassification (Vec<JSBin>);

impl From<Vec<Bin>> for JSClassification{
    fn from(classification: Vec<Bin>)->Self{
        JSClassification(classification.iter().map(|bin| JSBin::from(bin)).collect())
    }
}

impl From<&Bin> for JSBin{
    fn from(bin: &Bin)->Self{
        JSBin{
            bin_start: bin.bin_start,
            bin_end: bin.bin_end,
            count: bin.count
        }
    }
}   

#[wasm_bindgen]
pub fn get_jenks_breaks(no_bins: usize,data: &[f64])->JsValue{
   let breaks: JSClassification = crate::jenks::get_jenks_classification(no_bins,data).into();
   breaks.into()
}
