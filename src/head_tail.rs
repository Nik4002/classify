use crate::utilities::breaks_to_classification;
use crate::utilities::Classification;

/// Returns a Classification object following the Head-Tail Breaks algorithm given one-dimensional f64 data
///
/// # Arguments
///
/// * `data` - A reference to a vector of unsorted data points (f64) to generate a Classification for
///
/// # Examples
///
/// ```
/// use classify::get_head_tail_classification;
/// use classify::{Classification, Bin};
///
/// let data: Vec<f64> = vec![1.0/1.0, 1.0/2.0, 1.0/3.0, 1.0/4.0, 1.0/5.0, 
///                           1.0/6.0, 1.0/7.0, 1.0/8.0, 1.0/9.0, 1.0/10.0];
///
/// let result: Classification = get_head_tail_classification(&data);
/// let expected: Classification = Classification {bins: vec![
///     Bin{bin_start: 0.1, bin_end: 0.2928968253968254, count: 7},
///     Bin{bin_start: 0.2928968253968254, bin_end: 0.611111111111111, count: 2},
///     Bin{bin_start: 0.611111111111111, bin_end: 1.0, count: 1}]
/// };
///
/// assert!(result == expected);
/// ```
pub fn get_head_tail_classification(data: &Vec<f64>) -> Classification {
    let breaks: Vec<f64> = get_head_tail_breaks(data);
    breaks_to_classification(&breaks, data)
}

/// Returns a vector of breaks generated through the Head-Tail Breaks algorithm given a dataset
///
/// # Arguments
///
/// * `data` - A reference to a vector of unsorted data points (f64) to generate breaks for
///
/// # Examples
///
/// ```
/// use classify::get_head_tail_breaks;
///
/// let data: Vec<f64> = vec![1.0/1.0, 1.0/2.0, 1.0/3.0, 1.0/4.0, 1.0/5.0, 
///                           1.0/6.0, 1.0/7.0, 1.0/8.0, 1.0/9.0, 1.0/10.0];
///
/// let result: Vec<f64> = get_head_tail_breaks(&data);
///
/// assert_eq!(result, vec![0.2928968253968254, 0.611111111111111]);
/// ```
pub fn get_head_tail_breaks(data: &Vec<f64>) -> Vec<f64> {
    let mut breaks: Vec<f64> = vec![];

    let mut sorted_data: Vec<f64> = vec![];
    for item in data.iter().take(data.len()) {
        sorted_data.push(*item);
    }
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    head_tail_recursion(&sorted_data, &mut breaks);
    
    breaks
}

/// Recursive function used by get_head_tail_breaks that populates a vector of breaks according to the head-tail breaks algorithm
pub fn head_tail_recursion(data: &Vec<f64>, breaks: &mut Vec<f64>) {
    let mut mean: f64 = 0.0;
    for val in data {mean += val};
    mean /= data.len() as f64;

    breaks.push(mean);

    let break_idx;
    let mut low = 0;
    let mut high = data.len();
    'outer: loop {
        let mid = (low + high)/2;
        if mean < data[mid] as f64 {
            high = mid;
        } else if mean == data[mid] {
            break_idx = mid;
            break 'outer;
        } else if high - low <= 1 {
            break_idx = high;
            break 'outer;
        } else {
            low = mid;
        }
    }

    let head: Vec<f64> = data[break_idx..].to_vec();

    if head.len() > 0 {
        if (head.len() as f64)/(data.len() as f64) <= 0.4 && head[0] != head[head.len()-1] {
            head_tail_recursion(&head, breaks);
        }
    }
}