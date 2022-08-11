use crate::utilities::Classification;
use crate::utilities::{breaks_to_classification, to_vec_f64};
use num_traits::ToPrimitive;

/// Returns a Classification object following the Quantile Breaks algorithm given the desired number of bins and one-dimensional f64 data
///
/// # Arguments
///
/// * `num_bins` - An integer (usize) representing the desired number of bins
/// * `data` - A reference to a vector of unsorted data points to generate a Classification for
///
/// # Edge Cases
///
/// * Inputting large u64/i64 data (near their max values) will result in loss of precision because data is being cast to f64
/// * The maximum number of bins generated by this algorithm is the number of unique values in the dataset
/// * If your dataset contains many duplicates, there is a chance that the number of bins produced by the algorithm differs from num_bins because duplicate breaks are removed
///
/// # Examples
///
/// ```
/// use classify::get_quantile_classification;
/// use classify::{Classification, Bin};
///
/// let data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
/// let num_bins = 3;
///
/// let result: Classification = get_quantile_classification(num_bins, &data);
/// let expected: Classification = vec![
///     Bin{bin_start: 1.0, bin_end: 3.5, count: 3},
///     Bin{bin_start: 3.5, bin_end: 6.5, count: 3},
///     Bin{bin_start: 6.5, bin_end: 9.0, count: 3}
/// ];
///
/// assert!(result == expected);
/// ```
pub fn get_quantile_classification<T: ToPrimitive>(num_bins: usize, data: &[T]) -> Classification {
    let breaks: Vec<f64> = get_quantile_breaks(num_bins, data);
    breaks_to_classification(&breaks, data)
}

/// Returns a vector of breaks generated through the Quantile Breaks algorithm given the desired number of bins and a dataset
///
/// # Arguments
///
/// * `num_bins` - The desired number of bins
/// * `data` - A reference to a vector of unsorted data points to generate breaks for
///
/// # Edge Cases
///
/// * Inputting large u64/i64 data (near their max values) will result in loss of precision because data is being cast to f64
/// * The maximum number of bins generated by this algorithm is the number of unique values in the dataset
/// * If your dataset contains many duplicates, there is a chance that the number of bins produced by the algorithm differs from num_bins because duplicate breaks are removed
///
/// # Examples
///
/// ```
/// use classify::get_quantile_breaks;
///
/// let data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let num_bins = 3;
///
/// let result: Vec<f64> = get_quantile_breaks(num_bins, &data);
///
/// assert_eq!(result, vec![3.5, 6.5]);
/// ```
pub fn get_quantile_breaks<T: ToPrimitive>(num_bins: usize, data: &[T]) -> Vec<f64> {
    let data = to_vec_f64(data);

    if num_bins == 0 || num_bins == 1 {
        let result: Vec<f64> = vec![];
        return result;
    }

    let num_vals = data.len();

    let mut sorted_data: Vec<f64> = vec![];
    for item in data.iter().take(num_vals) {
        sorted_data.push(*item);
    }
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:#?}", sorted_data);

    let true_num_bins = std::cmp::min(num_vals, num_bins);

    let mut breaks: Vec<f64> = vec![];

    for i in 1..true_num_bins {
        let new_break = (sorted_data
            [((i * num_vals) as f64 / (true_num_bins) as f64) as usize - 1]
            + sorted_data[((i * num_vals) as f64 / (true_num_bins) as f64) as usize])
            / 2.0;
        breaks.push(new_break);
    }

    breaks.dedup();

    breaks
}
