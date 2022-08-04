# classify
This Rust crate aims to provide easy-to-use functions and structs that can classify data through a variety of algorithms, including Jenks Natural Breaks, Quantile Breaks, and more. 

Implemented as of version 0.1.3:
 * Classification methods: Jenks, Quantile, Head-Tail
 * Structs: Bin and Classification
 * Other miscellaneous functionality

Expected future features:
 * Classification methods: Equal Interval, Standard Deviation, and more
 * Function to determine the bin in a Classification to which a particular data point belongs
 * Generalizing data input types to include all float or numeric types, not just f64