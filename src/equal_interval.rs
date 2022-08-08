use crate::utilities::breaks_to_classification;
use crate::utilities::Classification;

/// Returns a Classification object following the Equal Interval Breaks algorithm given the desired number of bins and one-dimensional f64 data
///
/// # Arguments
///
/// * `num_bins` - An integer (usize) representing the desired number of bins
/// * `data` - A reference to a vector of unsorted data points (f64) to generate a Classification for
///
/// # Edge cases
///
/// * If there is a wide enoguh gap in the data, this algorithm may produce one or more empty bins
///
/// # Examples
///
/// ```
/// use classify::get_equal_interval_classification;
/// use classify::{Classification, Bin};
///
/// let data: Vec<f64> = vec![0.0, 0.5, 1.0, 1.5, 2.5, 3.0];
/// let num_bins = 3;
///
/// let result: Classification = get_equal_interval_classification(num_bins, &data);
/// let expected: Classification = vec![
///     Bin{bin_start: 0.0, bin_end: 1.0, count: 2},
///     Bin{bin_start: 1.0, bin_end: 2.0, count: 2},
///     Bin{bin_start: 2.0, bin_end: 3.0, count: 2}
/// ];
///
/// assert!(result == expected);
/// ```
pub fn get_equal_interval_classification(num_bins: usize, data: &Vec<f64>) -> Classification {
    let breaks: Vec<f64> = get_equal_interval_breaks(num_bins, data);
    breaks_to_classification(&breaks, data)
}

/// Returns a vector of breaks generated through the Equal Interval Breaks algorithm given the desired number of bins and a dataset
///
/// # Arguments
///
/// * `num_bins` - The desired number of bins
/// * `data` - A reference to a vector of unsorted data points (f64) to generate breaks for
///
/// # Edge cases
///
/// * If there is a wide enoguh gap in the data, this algorithm may produce one or more empty bins
///
/// # Examples
///
/// ```
/// use classify::get_equal_interval_breaks;
///
/// let data: Vec<f64> = vec![0.0, 0.5, 1.0, 1.5, 2.5, 3.0];
/// let num_bins = 3;
///
/// let result: Vec<f64> = get_equal_interval_breaks(num_bins, &data);
///
/// assert_eq!(result, vec![1.0, 2.0]);
/// ```
pub fn get_equal_interval_breaks(num_bins: usize, data: &Vec<f64>) -> Vec<f64> {
    let mut min_value = data[0];
    let mut max_value = data[0];
    for item in data {
        if *item < min_value {
            min_value = *item;
        } else if *item > max_value {
            max_value = *item
        }
    }

    let mut result: Vec<f64> = vec![];
    for i in 1..num_bins {
        result.push(min_value + (max_value - min_value) * (i as f64 / num_bins as f64));
    }

    result
}
