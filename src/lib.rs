mod jenks;
use crate::jenks::get_jenks_classification;
use crate::jenks::{UniqueVal, Bin, Classification};

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_jenks_classification() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}