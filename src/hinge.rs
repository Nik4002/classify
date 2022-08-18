use crate::utilities::Classification;
use crate::utilities::{breaks_to_classification, to_vec_f64};
use num_traits::ToPrimitive;

/// Returns a Classification object following the Hinge Breaks algorithm given the desired number of bins and one-dimensional data
///
/// # Arguments
///
/// * `hinge_coefficient` - A coefficient representing the size of the hinge as a multiple of the data's IQR (usually 1.5 or 3)
/// * `data` - A reference to a collection of unsorted data points to generate a Classification for
///
/// # Edge cases
///
/// * Inputting large u64/i64 data (near their max values) will result in loss of precision because data is being cast to f64
/// * If the data doesn't have outliers below/above the hinges, the algorithm may not produce all six bins
///
/// # Examples
///
/// ```
/// use classify::get_hinge_classification;
/// use classify::{Classification, Bin};
///
/// let data: Vec<f32> = vec![0.0, 1.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 20.0, 25.0];
/// let hinge_coefficient = 1.5;
///
/// let result: Classification = get_hinge_classification(hinge_coefficient, &data);
/// let expected: Classification = vec![
///     Bin{bin_start: 0.0, bin_end: 3.0, count: 2},
///     Bin{bin_start: 3.0, bin_end: 10.5, count: 1},
///     Bin{bin_start: 10.5, bin_end: 13.0, count: 2},
///     Bin{bin_start: 13.0, bin_end: 15.5, count: 3},
///     Bin{bin_start: 15.5, bin_end: 23.0, count: 2},
///     Bin{bin_start: 23.0, bin_end: 25.0, count: 1}
/// ];
///
/// assert!(result == expected);
/// ```
pub fn get_hinge_classification<T: ToPrimitive, S: ToPrimitive>(
    hinge_coefficient: S,
    data: &[T],
) -> Classification {
    let breaks: Vec<f64> = get_hinge_breaks(hinge_coefficient, data);
    breaks_to_classification(&breaks, data)
}

/// Returns a vector of breaks generated through the Hinge Breaks algorithm given the desired number of bins and a dataset
///
/// # Arguments
///
/// * `hinge_coefficient` - A coefficient representing the size of the hinge as a multiple of the data's IQR (usually 1.5 or 3)
/// * `data` - A reference to a collection of unsorted data points to generate breaks for
///
/// # Edge cases
///
/// * Inputting large u64/i64 data (near their max values) will result in loss of precision because data is being cast to f64
/// * If the data doesn't have outliers below/above the hinges, the algorithm may not produce all six bins
///
/// # Examples
///
/// ```
/// use classify::get_hinge_breaks;
///
/// let data: Vec<usize> = vec![0, 1, 10, 11, 12, 13, 14, 15, 16, 20, 25];
/// let hinge_coefficient = 1.5;
///
/// let result: Vec<f64> = get_hinge_breaks(hinge_coefficient, &data);
///
/// assert_eq!(result, vec![3.0, 10.5, 13.0, 15.5, 23.0]);
/// ```
pub fn get_hinge_breaks<T: ToPrimitive, S: ToPrimitive>(
    hinge_coefficient: S,
    data: &[T],
) -> Vec<f64> {
    let hinge_coefficient = hinge_coefficient.to_f64().unwrap();
    let data = to_vec_f64(data);

    let num_vals = data.len();

    let mut sorted_data: Vec<f64> = vec![];
    for item in data.iter().take(num_vals) {
        sorted_data.push(*item);
    }
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min_val = sorted_data[0];
    let max_val = sorted_data[num_vals - 1];

    let perc_25 = percentile(25, &sorted_data);
    let perc_50 = percentile(50, &sorted_data);
    let perc_75 = percentile(75, &sorted_data);
    let iqr = perc_75 - perc_25;
    let hinge = iqr * hinge_coefficient;

    let mut breaks: Vec<f64> = vec![];
    if perc_25 - hinge > min_val {
        breaks.push(perc_25 - hinge);
    }
    breaks.push(perc_25);
    breaks.push(perc_50);
    breaks.push(perc_75);
    if perc_75 + hinge < max_val {
        breaks.push(perc_75 + hinge);
    }

    breaks
}

/// Calculates percentiles of a given dataset
pub fn percentile(perc: u8, data: &Vec<f64>) -> f64 {
    let num_vals = data.len();

    let mut sorted_data: Vec<f64> = vec![];
    for item in data.iter().take(num_vals) {
        sorted_data.push(*item);
    }
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let rank = (perc as f64 / 100.0) * (num_vals as f64 - 1.0);

    if rank as usize == num_vals - 1 {
        sorted_data[num_vals - 1]
    } else {
        let rank_int = rank as usize;
        let rank_dec = rank - rank_int as f64;

        sorted_data[rank_int] + rank_dec * (sorted_data[rank_int + 1] - sorted_data[rank_int])
    }
}
