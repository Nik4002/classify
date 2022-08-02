use crate::utilities::breaks_to_classification;
use crate::utilities::Classification;

// fn get_quantile_classification() {
    
// }

fn get_quantile_breaks(num_bins: &usize, data: &Vec<f64>) -> Vec<f64> {
    if num_bins == 0 || num_bins == 1 {
        let result: Vec<f64> = vec![];
        return result;
    }

    let num_vals = data.len();
    
    let mut unique_data: Vec<f64> = vec![];
    for item in data.iter().take(num_vals) {
        unique_data.push(*item);
    }
    unique_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    unique_data.dedup();

    let num_unique_vals = unique_data.len();
    let true_num_bins = *std::cmp::min(&num_unique_vals, num_bins);

    let mut breaks: Vec<f64> = vec![];

    for i in 1..true_num_bins-1 { // Check if there's an off-by-one error here (I think there is...)
        breaks.push(unique_data[((i*num_unique_vals) as f64/(true_num_bins-1) as f64) as usize]);
    }

    breaks
}