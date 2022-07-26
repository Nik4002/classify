use rand::thread_rng;
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::prelude::*;

use std::collections::HashSet;
use std::time::{Instant};

// extern crate test;

/// Represents a unique value found within a sorted dataset along with the indices of its first and last occurrences in the dataset
pub struct UniqueVal {val: f64, first: usize, last: usize}

/// Represents a single bin in a classification, including the bin's lowest (inclusive) and highest (exclusive) values and the number of points within it
pub struct Bin {
    pub bin_start: f64,
    pub bin_end: f64,
    pub count: i64
}

/// Represents a full classification, which is a collection of Bin objects
pub struct Classification {
    pub bins: Vec<Bin>
}

fn main() {
    // let data: Vec<f64> = vec![1.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0];
    // let data: Vec<f64> = vec![4.0, 1.0, 4.0, 3.0, 4.0, 1.0, 2.0, 5.0, 6.0, 8.0];
    // let mut undefined: Vec<bool> = vec![false, false, true, false, false, true, false];
    
    // let data = vec![-0.773196217050617, 0.24842717545639237,
    // -0.6598113252414564, 0.6920640566349373,
    // -0.23518920803371415, -0.5616678850149022,
    // -0.2816950877136631, -0.9114944430563943,
    // -0.24893149862052785, 0.584049927279119,
    // 0.7188483142673544, 0.4163443332288843,
    // 0.28795174508987703, 1.0276695211320594,
    // -1.078385486977444, 0.8874191999016873,
    // 0.23384176150735006, -0.7151122736860034,
    // -0.3481593622218171, 2.845586320877743]; // geojson sample data (col: numbers)
    // let mut undefined: Vec<bool> = vec![false; data.len()];

    let data = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];

    // let mut rng = thread_rng();
    // let data: Vec<f64> = Standard.sample_iter(&mut rng).take(10000).collect();
    // let mut undefined: Vec<bool> = vec![false; data.len()];

    println!("Data points: {}", data.len());
    
    // Starting timer
    let start = Instant::now();

    // Jenks algorithm call
    let num_bins: usize = 3;

    // let breaks: Vec<f64> = get_jenks_breaks(&num_bins, &data, &mut undefined);
    // // println!("Breaks: {}", num_bins);

    // let output: Classification = breaks_to_classification(breaks, &mut data, &mut undefined);
    let output: Classification = get_jenks_classification(&num_bins, &data);

    for bin in output.bins {
        println!("Start: {}, End: {}, Count: {}", bin.bin_start, bin.bin_end, bin.count);
    }

    // End timer
    let end = Instant::now();
    let elapsed = end.checked_duration_since(start).unwrap();
    println!("{:#?} elapsed", elapsed);
}

/// Returns a Classification object following the Jenks Natural Breaks algorithm given the desired number of categories and one-dimensional f64 data
///
/// # Arguments
///
/// * `num_bins` - A reference to an integer (u64) representing the desired number of bins
/// * `data` - A reference to a vector of unsorted data points (f64) to generate breaks for
///
/// # Examples
///
/// ```
/// use classify::get_jenks_classification; // Note: make sure this is still the name of the crate later
/// use classify::Classification;
/// 
/// let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];
/// let num_bins = 3;
/// 
/// let result: Classification = jenks(&num_bins, &mut data);
/// 
/// for bin in result.bins {
///     println!("Start: {}, End: {}, Count: {}", bin.bin_start, bin.bin_end, bin.count); 
/// }
/// ```
pub fn get_jenks_classification(num_bins: &usize, data: &Vec<f64>) -> Classification {
    let breaks: Vec<f64> = get_jenks_breaks(&num_bins, data);
    return breaks_to_classification(&breaks, data);
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
/// use classify::{breaks_to_classification}; // Note: make sure this is still the name of the crate later
/// use classify::Classification;
/// 
/// let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];
/// let breaks: Vec<f64> = vec![2.0, 5.0];
///
/// let result: Classification = breaks_to_classification(&breaks, &mut data);
/// 
/// for bin in result.bins {
///     println!("Start: {}, End: {}, Count: {}", bin.bin_start, bin.bin_end, bin.count); 
/// }
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
    // println!("Bin bounds: {:#?}", bounds);

    let mut results: Vec<Bin> = vec![];
    for i in 0..(bounds.len() - 1) {
        results.push(Bin{
            bin_start: bounds[i], 
            bin_end: bounds[i + 1],
            count: 0});

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
    results[num_bins-1].count += 1;

    return Classification{bins: results};
}

/// Returns a vector of breaks generated through the Jenks Natural Breaks algorithm given the desired number of bins and a dataset
///
/// # Arguments
///
/// * `num_bins` - The desired number of bins
/// * `data` - A reference to a vector of unsorted data points (f64) to generate breaks for
///
/// # Examples
///
/// ```
/// use classify::{breaks_to_classification}; // Note: make sure this is still the name of the crate later
/// 
/// let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];
/// let num_bins = 3;
///
/// let result: Vec<f64> = get_jenks_breaks(&num_bins, &mut data);
/// 
/// println!("{:#}", result);
/// ```
pub fn get_jenks_breaks(num_bins: &usize, data: &Vec<f64>) -> Vec<f64> {
    let num_vals = data.len();

    let mut sorted_data: Vec<f64> = vec![];
    for i in 0..num_vals {
        sorted_data.push(data[i]);
    }
    sorted_data.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    // println!("{:#?}", sorted_data);

    let mut unique_val_map: Vec<UniqueVal> = vec![];
    create_unique_val_mapping(&mut unique_val_map, &sorted_data);
    // for elem in &unique_val_map {println!("Val: {}, first: {}, last: {}", elem.val, elem.first, elem.last)}
    // println!("");

    let num_unique_vals = unique_val_map.len();
    let true_num_bins = std::cmp::min(&num_unique_vals, &num_bins);
    // println!("Unique vals: {}, categories: {}\n", num_unique_vals, true_num_bins);

    let gssd = calc_gssd(&sorted_data);

    let mut rand_breaks: Vec<usize> = vec![0 as usize; true_num_bins - 1];
    let mut best_breaks: Vec<usize> = vec![0 as usize; true_num_bins - 1];
    let mut unique_rand_breaks: Vec<usize> = vec![0 as usize; true_num_bins - 1];

    let mut max_gvf: f64 = 0.0;
    
    let c = 5000*2200*4;
    let mut permutations = c/num_vals;
    if permutations < 10 {permutations = 10}
    if permutations > 10000 {permutations = 10000}
    println!("permutations: {}", permutations);

    let mut pseudo_rng = StdRng::seed_from_u64(123456789);
    
    // pick_rand_breaks(&mut unique_rand_breaks, &num_unique_vals, &mut pseudo_rng);
    // println!("Random unique value breaks: {:#?}\n", unique_rand_breaks);

    // unique_to_normal_breaks(&unique_rand_breaks, &unique_val_map, &mut rand_breaks);
    // println!("Random normal breaks: {:#?}\n", rand_breaks);

    // println!("GVF of these breaks: {}", calc_gvf(&rand_breaks, &sorted_data, &gssd));

    for _ in 0..permutations {
        pick_rand_breaks(&mut unique_rand_breaks, &num_unique_vals, &mut pseudo_rng);
        unique_to_normal_breaks(&unique_rand_breaks, &unique_val_map, &mut rand_breaks);
        let new_gvf: f64 = calc_gvf(&rand_breaks, &sorted_data, &gssd);
        if new_gvf > max_gvf {
            max_gvf = new_gvf;
            for i in 0..rand_breaks.len() {
                best_breaks[i] = rand_breaks[i];
            }
        }
    }

    let mut nat_breaks: Vec<f64> = vec![];
    nat_breaks.resize(best_breaks.len(), 0.0);
    for i in 0..best_breaks.len() {
        nat_breaks[i] = sorted_data[best_breaks[i]];
    }
    println!("Breaks: {:#?}", nat_breaks);

    return nat_breaks;
}

/// Populates an empty vector of UniqueVal objects for each unique value in the dataset in the format (value, first occurrence index, last occurrence index)
/// 
/// # Arguments
/// 
/// * `unique_val_map` - A mutable reference to an empty vector of UniqueVals
/// * `vals` - A reference to the data (sorted, ascending) to use in populating unique_val_map
/// 
/// # Examples
/// 
/// ```
/// use classify::create_unique_val_mapping;
/// use classify::UniqueVal;
/// 
/// let mut unique_val_map: Vec<UniqueVal> = vec![];
/// let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];
/// 
/// create_unique_val_mapping(&mut unique_val_map, &data);
/// 
/// for unique_val in unique_val_map {
///     println!("Val: {}, first: {}, last: {}", unique_val.val, unique_val.first, unique_val.last);
/// }
/// ```
pub fn create_unique_val_mapping(unique_val_map: &mut Vec<UniqueVal>, vals: &Vec<f64>) {
    unique_val_map.clear();
    let mut idx: i64 = -1;

    for i in 0..vals.len() {
        if unique_val_map.is_empty() {
            idx += 1;
            unique_val_map.push(UniqueVal {val: vals[i], first: i, last: i});
        } else {
            if unique_val_map[idx as usize].val != vals[i] {
                unique_val_map[idx as usize].last = i-1;
                idx += 1;
                unique_val_map.push(UniqueVal {val: vals[i], first: i, last: i});
            }
        }
    }
}

/// Populates a vector with a set of breaks as unique random integers that are valid indices within the dataset given the number of data points and an RNG
/// 
/// # Arguments
/// 
/// * `breaks` - A mutable reference to an empty vector of breaks whose length is taken to be the desired number of breaks
/// * `num_vals` - A reference to the number of data points
/// * `rng` - A mutable reference to a seedable random number generator (RNG) from the "rand" crate
/// 
/// # Examples
/// 
/// ```
/// use classify::pick_rand_breaks;
/// use rand::rngs::StdRng;
/// use rand::prelude::*;
/// 
/// let num_bins = 5;
/// let mut breaks: Vec<usize> = vec![0; num_bins - 1];
/// let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];
/// let num_vals = data.len();
/// let mut rng = StdRng::seed_from_u64(123456789);
/// 
/// pick_rand_breaks(&mut breaks, &num_vals, &mut rng);
/// 
/// println!("{:#}", breaks);
/// ```
pub fn pick_rand_breaks(breaks: &mut Vec<usize>, num_vals: &usize, rng: &mut StdRng) {
    let num_breaks = breaks.len();
    if num_breaks > num_vals-1 {return}

    let mut set = HashSet::new();
    while set.len() < num_breaks {set.insert(rng.gen_range(1..*num_vals));}
    let mut set_iter = set.iter();
    for i in 0..set_iter.len() {
        breaks[i] = *set_iter.next().unwrap();
    }
    breaks.sort();
}

/// Adjusts break indices from unique value breaks to normal breaks, accounting for repeated values, given unique value breaks, a unique value map, and an empty vector for normal breaks
/// 
/// # Arguments
/// 
/// * `u_val_breaks` - A reference to a vector of uniquely valued breaks (sorted, ascending)
/// * `u_val_map` - A reference to a map of unique values to their first and last occurrences in the dataset
/// * `normal_breaks` - A mutable reference to an empty vector to populate with adjusted break indices
/// 
/// # Examples
/// 
/// ```
/// use classify::{unique_to_normal_breaks, create_unique_val_mapping};
/// use classify::UniqueVal;
/// 
/// let mut unique_val_map: Vec<UniqueVal> = vec![];
/// let data: Vec<f64> = vec![1.0, 2.0, 2.0, 4.0, 5.0, 7.0, 7.0, 7.0, 8.0];
/// create_unique_val_mapping(&mut unique_val_map, &data);
/// let unique_val_breaks: Vec<usize> = vec![1, 3, 4];
/// let normal_breaks: Vec<usize> = vec![0; unique_val_breaks.len()];
/// 
/// unique_to_normal_breaks(&unique_val_breaks, &unique_val_map, &mut normal_breaks);
/// 
/// println!("{:#}", normal_breaks);
/// ```
pub fn unique_to_normal_breaks(u_val_breaks: &Vec<usize>, u_val_map: &Vec<UniqueVal>, normal_breaks: &mut Vec<usize>) {
    if normal_breaks.len() != u_val_breaks.len() {
        normal_breaks.resize(u_val_breaks.len(), 0);
    }
    for i in 0..u_val_breaks.len() {
        normal_breaks[i] = u_val_map[u_val_breaks[i]].first;
    }
}

/// Calculates goodness of variance fit (GVF) for a particular set of breaks on a dataset
/// 
/// # Arguments
/// 
/// * `breaks` - A reference to a vector (usize) of break indices (sorted, ascending)
/// * `vals` - A reference to a vector (f64) of data points (sorted, ascending)
/// * `gssd` - A reference to the global sum of squared deviations (GSSD)
/// 
/// # Examples
/// 
/// ```
/// use classify::{calc_gvf, create_unique_val_mapping, calc_gssd};
/// use classify::UniqueVal;
/// 
/// let mut unique_val_map: Vec<UniqueVal> = vec![];
/// let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];
/// create_unique_val_mapping(&mut unique_val_map, &data);
/// let breaks: Vec<usize> = vec![1, 3, 4];
/// let gssd: f64 = calc_gssd(&data);
/// 
/// println!("{}", calc_gvf(&breaks, &data, &gssd));
/// ```
pub fn calc_gvf(breaks: &Vec<usize>, vals: &Vec<f64>, gssd: &f64) -> f64 {
    let num_vals = vals.len();
    let num_bins = breaks.len() + 1;
    let mut tssd: f64 = 0.0;
    for i in 0..num_bins {
        let lower = if i == 0 {0} else {breaks[i-1]};
        let upper = if i == num_bins-1 {num_vals} else {breaks[i]};

        let mut mean: f64 = 0.0;
        let mut ssd: f64 = 0.0;
        for j in lower..upper {mean += vals[j]}
        mean /= (upper-lower) as f64;
        for j in lower..upper {ssd += (vals[j]-mean)*(vals[j]-mean)}
        tssd += ssd;
    }
    return 1.0-(tssd/gssd);
}

/// Calculates global sum of squared deviations (GSSD) for a particular dataset
/// 
/// # Arguments
/// 
/// * `data` - A reference to a vector (f64) of data points (sorted, ascending)
/// 
/// # Examples
/// 
/// ```
/// use classify::{calc_gssd};
/// 
/// let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];
/// 
/// println!("{}", calc_gssd(&data));
/// ```
pub fn calc_gssd(data: &Vec<f64>) -> f64 {
    let num_vals = data.len();
    let mut mean = 0.0;
    let mut max_val: f64 = data[0];
    for i in 0..num_vals {
        let val = data[i];
        if val > max_val {max_val = val}
        mean += val;
    }
    mean /= num_vals as f64;

    let mut gssd: f64 = 0.0;
    for i in 0..num_vals {
        let val = data[i];
        gssd += (val-mean)*(val-mean);
    }

    return gssd;
}