mod jenks;
pub use jenks::{get_jenks_breaks, get_jenks_classification};

mod utilities;
pub use utilities::breaks_to_classification;
pub use utilities::{Bin, Classification, UniqueVal};

#[cfg(test)]
mod tests {
    use crate::*;
    use rand::rngs::StdRng;
    use rand::prelude::*;

    #[test]
    fn test_create_unique_val_mapping() {
        let mut unique_val_map: Vec<UniqueVal> = vec![];
        let data: Vec<f64> = vec![2.0, 2.0, 7.0, 7.0, 7.0, 8.0];

        utilities::create_unique_val_mapping(&mut unique_val_map, &data);

        let expected = vec![UniqueVal{val: 2.0, first: 0, last: 1}, UniqueVal{val: 7.0, first: 2, last: 4}, UniqueVal{val: 8.0, first: 5, last: 5}];

        for i in 0..unique_val_map.len() {
            let value = &unique_val_map[i];
            let check = &expected[i];
            assert_eq!(value.val, check.val, "create_unique_val_mapping not working!");
            assert_eq!(value.first, check.first, "create_unique_val_mapping not working!");
            assert_eq!(value.last, check.last, "create_unique_val_mapping not working!");
        }
    }

    #[test]
    fn test_unique_to_normal_breaks() {
        let mut unique_val_map: Vec<UniqueVal> = vec![];
        let data: Vec<f64> = vec![1.0, 2.0, 2.0, 4.0, 5.0, 7.0, 7.0, 7.0, 8.0];
        utilities::create_unique_val_mapping(&mut unique_val_map, &data);
        let unique_val_breaks: Vec<usize> = vec![1, 3, 4];
        let mut normal_breaks: Vec<usize> = vec![0; unique_val_breaks.len()];

        utilities::unique_to_normal_breaks(&unique_val_breaks, &unique_val_map, &mut normal_breaks);

        assert_eq!(normal_breaks, vec![1, 4, 5], "unique_to_normal_breaks not working!");
    }

    #[test]
    fn test_pick_rand_breaks() {
        let num_bins = 3;
        let mut breaks: Vec<usize> = vec![0; num_bins - 1];
        let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0, 10.0, 11.0];
        let num_vals = data.len();
        let mut rng = StdRng::seed_from_u64(123456789);
        
        jenks::pick_rand_breaks(&mut breaks, &num_vals, &mut rng);
        
        assert_eq!(breaks, vec![3, 7], "pick_rand_breaks not working!");
    }

    #[test]
    fn test_calc_gvf() {
        let mut unique_val_map: Vec<UniqueVal> = vec![];
        let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];
        utilities::create_unique_val_mapping(&mut unique_val_map, &data);
        let breaks: Vec<usize> = vec![1, 3, 4];
        let gssd: f64 = jenks::calc_gssd(&data);

        assert_eq!(jenks::calc_gvf(&breaks, &data, &gssd), 0.9333333333333333);
    }

    #[test]
    fn test_calc_gssd() {
        let data: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 7.0, 8.0];

        assert_eq!(jenks::calc_gssd(&data), 37.5);
    }
}
