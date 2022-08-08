# classify
This Rust crate aims to provide easy-to-use functions and structs that can classify data through a variety of algorithms, including Jenks Natural Breaks, Quantile Breaks, and more. 

Implemented as of version 0.2.0:
 * Classification methods: Jenks, Quantile, Head-Tail, Equal Interval, Standard Deviation
 * Structs/types: `Bin` and `Classification` (type synonym for `Vec<Bin>`)
 * Function to determine the bin in a Classification to which a particular data point belongs
 * Changelog
 * Other miscellaneous functionality

Planned future features:
 * Classification methods: Hinge
 * Generalizing data input types to include all float or numeric types, not just f64