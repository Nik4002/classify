use crate::utilities::Classification;
use crate::utilities::{breaks_to_classification, to_vec_f64};
use num_traits::ToPrimitive;

/// Returns a Classification object following the Standard Deviation Breaks algorithm given the desired bin size as a proportion of a standard deviation and one-dimensional data
/// Note: This algorithm calculates Standard Deviation with Bessel's correction
///
/// # Arguments
///
/// * `bin_size` - A float (f64) representing the proportion of a standard deviation each bin should encompass
/// * `data` - A reference to a vector of unsorted data points (f64) to generate a Classification for
///
/// # Edge cases
///
/// * Inputting large u64/i64 data (near their max values) will result in loss of precision because data is being cast to f64
///
/// # Examples
///
/// ```
/// use classify::get_st_dev_classification;
/// use classify::{Classification, Bin};
///
/// let data: Vec<f32> = vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
/// let bin_size = 1.0; // Bins will be the size of one standard deviation
///
/// let result: Classification = get_st_dev_classification(bin_size, &data);
/// let expected: Classification = vec![
///     Bin{bin_start: 0.0, bin_end: 0.41987655026535653, count: 1},
///     Bin{bin_start: 0.41987655026535653, bin_end: 1.5, count: 2},
///     Bin{bin_start: 1.5, bin_end: 2.5801234497346437, count: 3},
///     Bin{bin_start: 2.5801234497346437, bin_end: 3.0, count: 1}
/// ];
///
/// assert!(result == expected);
/// ```
pub fn get_st_dev_classification<T: ToPrimitive>(bin_size: f64, data: &[T]) -> Classification {
    let breaks: Vec<f64> = get_st_dev_breaks(bin_size, data);
    breaks_to_classification(&breaks, data)
}

/// Returns a vector of breaks generated through the Standard Deviation Breaks algorithm given the desired bin size as a proportion of a standard deviation and a dataset
/// Note: This algorithm calculates Standard Deviation with Bessel's correction
///
/// # Arguments
///
/// * `bin_size` - A float (f64) representing the proportion of a standard deviation each bin should encompass
/// * `data` - A reference to a collection of unsorted data points (f64) to generate breaks for
///
/// # Edge cases
///
/// * Inputting large u64/i64 data (near their max values) will result in loss of precision because data is being cast to f64
///
/// # Examples
///
/// ```
/// use classify::get_st_dev_breaks;
///
/// let data: Vec<f32> = vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
/// let bin_size = 1.0; // Bins will be the size of one standard deviation
///
/// let result: Vec<f64> = get_st_dev_breaks(bin_size, &data);
///
/// assert_eq!(result, vec![0.41987655026535653, 1.5, 2.5801234497346437]);
/// ```
pub fn get_st_dev_breaks<T: ToPrimitive>(bin_size: f64, data: &[T]) -> Vec<f64> {
    let data = to_vec_f64(data);

    let mut min_value = data[0];
    let mut max_value = data[0];
    let mut mean = 0.0;
    for item in &data {
        mean += item;
        if *item < min_value {
            min_value = *item;
        } else if *item > max_value {
            max_value = *item
        }
    }
    mean /= data.len() as f64;

    let st_dev = calc_st_dev(&data);
    let new_dev = st_dev * bin_size;

    let devs_below_mean = ((mean - min_value) / new_dev) as isize;
    let devs_above_mean = ((max_value - mean) / new_dev) as isize;

    let mut breaks: Vec<f64> = vec![];
    for i in -devs_below_mean..(devs_above_mean + 1) {
        breaks.push(mean + (i as f64) * new_dev);
    }

    breaks
}

/// Calculates the standard deviation of a dataset using Bessel's correction
///
/// # Arguments
///
/// * `data` - A reference to a collection containing data to calculate standard deviation for
pub fn calc_st_dev(data: &Vec<f64>) -> f64 {
    let mut mean: f64 = 0.0;
    for val in data {
        mean += *val
    }
    mean /= data.len() as f64;

    let mut sum_squared_dev = 0.0;
    for val in data {
        sum_squared_dev += (mean - *val) * (mean - *val);
    }

    let variance = sum_squared_dev / ((data.len() - 1) as f64);

    variance.sqrt()
}
