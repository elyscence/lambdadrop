use rand::{rng, Rng};
use rand::distr::Alphanumeric;

pub fn generate_short_id(length: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id = generate_short_id(6);
        assert_eq!(id.len(), 6);

        assert!(id.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_ids_are_unique() {
        let id1 = generate_short_id(6);
        let id2 = generate_short_id(6);

        assert_ne!(id1, id2);
    }
}