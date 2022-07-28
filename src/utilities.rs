/// Represents a unique value found within a sorted dataset along with the indices of its first and last occurrences in the dataset
pub struct UniqueVal {
    pub val: f64,
    pub first: usize,
    pub last: usize,
}

/// Represents a single bin in a classification, including the bin's lowest (inclusive) and highest (exclusive) values and the number of points within it
pub struct Bin {
    pub bin_start: f64,
    pub bin_end: f64,
    pub count: i64,
}

impl PartialEq for Bin {
    fn eq(&self, other: &Self) -> bool {
        let starts_eq: bool = self.bin_start == other.bin_start;
        let ends_eq: bool = self.bin_end == other.bin_end;
        let counts_eq: bool = self.count == other.count;
        return starts_eq && ends_eq && counts_eq;
    }
}

/// Represents a full classification, which is a collection of Bin objects
pub struct Classification {
    pub bins: Vec<Bin>,
}

impl PartialEq for Classification {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.bins.len() {
            if self.bins[i] != other.bins[i] {
                return false;
            }
        }
        return true;
    }
}

/// Populates an empty vector of UniqueVal objects for each unique value in the dataset in the format (value, first occurrence index, last occurrence index)
///
/// # Arguments
///
/// * `unique_val_map` - A mutable reference to an empty vector of UniqueVals
/// * `vals` - A reference to the data (sorted, ascending) to use in populating unique_val_map
pub fn create_unique_val_mapping(unique_val_map: &mut Vec<UniqueVal>, vals: &Vec<f64>) {
    unique_val_map.clear();
    let mut idx: i64 = -1;

    for i in 0..vals.len() {
        if unique_val_map.is_empty() {
            idx += 1;
            unique_val_map.push(UniqueVal {
                val: vals[i],
                first: i,
                last: i,
            });
        } else {
            if unique_val_map[idx as usize].val != vals[i] {
                unique_val_map[idx as usize].last = i - 1;
                idx += 1;
                unique_val_map.push(UniqueVal {
                    val: vals[i],
                    first: i,
                    last: i,
                });
            }
        }
    }
}

/// Adjusts break indices from unique value breaks to normal breaks, accounting for repeated values, given unique value breaks, a unique value map, and an empty vector for normal breaks
///
/// # Arguments
///
/// * `u_val_breaks` - A reference to a vector of uniquely valued breaks (sorted, ascending)
/// * `u_val_map` - A reference to a map of unique values to their first and last occurrences in the dataset
/// * `normal_breaks` - A mutable reference to an empty vector to populate with adjusted break indices
pub fn unique_to_normal_breaks(
    u_val_breaks: &Vec<usize>,
    u_val_map: &Vec<UniqueVal>,
    normal_breaks: &mut Vec<usize>,
) {
    if normal_breaks.len() != u_val_breaks.len() {
        normal_breaks.resize(u_val_breaks.len(), 0);
    }
    for i in 0..u_val_breaks.len() {
        normal_breaks[i] = u_val_map[u_val_breaks[i]].first;
    }
}

/// Returns a Classification object given a set of breaks between bins and the original dataset
///
/// # Arguments
///
/// * `breaks` - A reference to a vector of breaks (f64) generated through any classification function or manually
/// * `data` - A reference to a vector of unsorted data points (f64) used to count the points in each bin
///
/// # Examples
///
/// ```
/// use classify::{breaks_to_classification};
/// use classify::{Classification, Bin};
///
/// let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];
/// let breaks: Vec<f64> = vec![2.0, 5.0];
///
/// let result: Classification = breaks_to_classification(&breaks, &data);
/// let expected: Classification = Classification {bins: vec![
///     Bin{bin_start: 1.0, bin_end: 2.0, count: 1},
///     Bin{bin_start: 2.0, bin_end: 5.0, count: 2},
///     Bin{bin_start: 5.0, bin_end: 8.0, count: 3}]
/// };
/// 
/// assert!(result == expected);
/// ```
pub fn breaks_to_classification(breaks: &Vec<f64>, data: &Vec<f64>) -> Classification {
    let mut min_value = data[0];
    let mut max_value = data[0];
    for item in data {
        if *item < min_value {
            min_value = *item;
        }
        if *item > max_value {
            max_value = *item;
        }
    }

    let mut bounds: Vec<f64> = vec![min_value];
    for item in breaks {
        bounds.push(*item);
    }
    bounds.push(max_value);

    let mut results: Vec<Bin> = vec![];
    for i in 0..(bounds.len() - 1) {
        results.push(Bin {
            bin_start: bounds[i],
            bin_end: bounds[i + 1],
            count: 0,
        });
    }

    let num_bins = results.len();
    for i in 0..num_bins {
        for j in 0..data.len() {
            let mut bin = &mut results[i];
            let n = data[j];
            if bin.bin_start <= n && n < bin.bin_end {
                bin.count += 1;
            }
        }
    }
    results[num_bins - 1].count += 1;

    return Classification { bins: results };
}
