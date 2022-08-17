use crate::Bin;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct JSBin {
    pub bin_start: f64,
    pub bin_end: f64,
    pub count: u64,
}

impl PartialEq for JSBin {
    fn eq(&self, other: &Self) -> bool {
        let starts_eq: bool = self.bin_start == other.bin_start;
        let ends_eq: bool = self.bin_end == other.bin_end;
        let counts_eq: bool = self.count == other.count;
        starts_eq && ends_eq && counts_eq
    }
}

#[derive(Serialize, Deserialize)]
pub struct JSClassification(Vec<JSBin>);

impl PartialEq for JSClassification {
    fn eq(&self, other: &Self) -> bool {
        let JSClassification(self_bins) = self;
        let JSClassification(other_bins) = other;

        self_bins == other_bins
    }
}

impl From<Vec<Bin>> for JSClassification {
    fn from(classification: Vec<Bin>) -> Self {
        JSClassification(classification.iter().map(|bin| JSBin::from(bin)).collect())
    }
}

impl From<&JSClassification> for Vec<Bin> {
    fn from(jsclassification: &JSClassification) -> Self {
        let JSClassification(jsbins) = jsclassification;
        let mut result: Vec<Bin> = vec![];

        for bin in jsbins {
            result.push(Bin {bin_start: bin.bin_start, bin_end: bin.bin_end, count: bin.count});
        }

        result
    }
}

impl From<&Bin> for JSBin {
    fn from(bin: &Bin) -> Self {
        JSBin {
            bin_start: bin.bin_start,
            bin_end: bin.bin_end,
            count: bin.count,
        }
    }
}

#[wasm_bindgen]
pub fn breaks_to_classification(breaks: &[f64], data: &[f64]) -> JsValue { // Should this be JSClassification or JsValue?
    let class: JSClassification = crate::utilities::breaks_to_classification(&breaks.to_vec(), data).into();
    JsValue::into_serde(&class).unwrap()
}

#[wasm_bindgen]
pub fn classify_val(val: f64, class: &JSClassification) -> Option<usize> { 
    let bin: Option<usize> = crate::utilities::classify_val(val, &(class.into()));
    bin
}

#[wasm_bindgen]
pub fn get_jenks_breaks(no_bins: usize, data: &[f64]) -> Box<[f64]> { // Should these be Box<[f64]> or something else? (Maybe JsValue?)
    let breaks = crate::jenks::get_jenks_breaks(no_bins, data);
    breaks.into_boxed_slice()
}

#[wasm_bindgen]
pub fn get_jenks_classification(no_bins: usize, data: &[f64]) -> JsValue { // What's the benefit of using JsValue instead of JSClassification here?
    let class: JSClassification = crate::jenks::get_jenks_classification(no_bins, data).into();
    JsValue::into_serde(&class).unwrap()
}

#[wasm_bindgen]
pub fn get_quantile_breaks(no_bins: usize, data: &[f64]) -> Box<[f64]> {
    let breaks = crate::quantile::get_quantile_breaks(no_bins, data);
    breaks.into_boxed_slice()
}

#[wasm_bindgen]
pub fn get_quantile_classification(no_bins: usize, data: &[f64]) -> JsValue {
    let class: JSClassification = crate::quantile::get_quantile_classification(no_bins, data).into();
    JsValue::into_serde(&class).unwrap()
}

#[wasm_bindgen]
pub fn get_head_tail_breaks(data: &[f64]) -> Box<[f64]> {
    let breaks = crate::head_tail::get_head_tail_breaks(data);
    breaks.into_boxed_slice()
}

#[wasm_bindgen]
pub fn get_head_tail_classification(data: &[f64]) -> JsValue {
    let class: JSClassification = crate::head_tail::get_head_tail_classification(data).into();
    JsValue::into_serde(&class).unwrap()
}

#[wasm_bindgen]
pub fn get_equal_interval_breaks(no_bins: usize, data: &[f64]) -> Box<[f64]> {
    let breaks = crate::equal_interval::get_equal_interval_breaks(no_bins, data);
    breaks.into_boxed_slice()
}

#[wasm_bindgen]
pub fn get_equal_interval_classification(no_bins: usize, data: &[f64]) -> JsValue {
    let class: JSClassification = crate::equal_interval::get_equal_interval_classification(no_bins, data).into();
    JsValue::into_serde(&class).unwrap()
}

#[wasm_bindgen]
pub fn get_st_dev_breaks(bin_size: f64, data: &[f64]) -> Box<[f64]> {
    let breaks = crate::standard_deviation::get_st_dev_breaks(bin_size, data);
    breaks.into_boxed_slice()
}

#[wasm_bindgen]
pub fn get_st_dev_classification(bin_size: f64, data: &[f64]) -> JsValue {
    let class: JSClassification = crate::standard_deviation::get_st_dev_classification(bin_size, data).into();
    JsValue::into_serde(&class).unwrap()
}