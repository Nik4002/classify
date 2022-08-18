# classify
This Rust crate aims to provide easy-to-use functions and structs that can classify data through a variety of algorithms, including Jenks Natural Breaks, Quantile Breaks, and more. 

Implemented as of version 0.2.2:
 * Classification methods: Jenks, Quantile, Head-Tail, Equal Interval, Standard Deviation, Hinge
 * Structs/types: `Bin` and `Classification` (type synonym for `Vec<Bin>`)
 * Function to determine the bin in a Classification to which a particular data point belongs
 * Compatibility with any numeric data type (previously only f64)
 * Compatibility with any collection data type (previously only vector)
 * Changelog
 * WebAssembly (WASM) package + basic HTML file to test WASM code
 * Other miscellaneous functionality

Planned future features:
 * None

# WebAssembly 

 To generate the WASM package, run 

```bash
wasm-pack build --release -- -features js
```

in the root directory of this crate