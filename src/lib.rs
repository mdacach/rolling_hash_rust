use std::collections::VecDeque;

pub type Hash = u64;

pub struct RollingHash {
    current_string: VecDeque<char>,
    current_hash: Hash,
}

impl RollingHash {
    const BASE: u64 = 257;
    const MODULO: u64 = 1_000_000_000 + 7;

    pub fn new() -> Self {
        Self {
            current_string: VecDeque::new(),
            current_hash: 0,
        }
    }

    pub fn from_initial_string(input: &str) -> Self {
        let mut rh = Self::new();
        input.chars().for_each(|c| rh.append(c));
        rh
    }

    pub fn get_current_hash(&self) -> Hash {
        self.current_hash
    }

    pub fn append(&mut self, c: char) {
        self.current_string.push_back(c);

        self.current_hash *= Self::BASE;
        self.current_hash += c as u64;
        self.current_hash %= Self::MODULO;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::RollingHash;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn create_rolling_hash() {
        let _rh = RollingHash::new();
        let _rh_from_string = RollingHash::from_initial_string("Eiger");
    }

    #[test]
    fn append_characters_to_rolling_hash() {
        let mut rh = RollingHash::new();
        rh.append('E');
        rh.append('i');
        rh.append('g');
        rh.append('e');
        rh.append('r');
        let as_string = |vec: VecDeque<char>| -> String { vec.iter().collect() };
        assert_eq!(as_string(rh.current_string), "Eiger");
    }

    #[test]
    fn hash_changes_with_append() {
        let mut rh = RollingHash::new();
        let initial_hash = rh.get_current_hash();
        rh.append('E');
        let new_hash = rh.get_current_hash();
        assert_ne!(initial_hash, new_hash);
    }

    #[test]
    fn hash_for_equal_strings_are_equal() {
        let rh1 = RollingHash::from_initial_string("Eiger");
        let rh2 = RollingHash::from_initial_string("Eiger");
        assert_eq!(rh1.get_current_hash(), rh2.get_current_hash());
    }

    #[test]
    fn hash_for_different_strings_are_different() {
        // Complete different strings
        let rh1 = RollingHash::from_initial_string("Eiger");
        let rh2 = RollingHash::from_initial_string("Matheus");
        assert_ne!(rh1.get_current_hash(), rh2.get_current_hash());

        // Different strings with same length
        let rh1 = RollingHash::from_initial_string("Eiger");
        let rh2 = RollingHash::from_initial_string("Great");
        assert_ne!(rh1.get_current_hash(), rh2.get_current_hash());

        // Single characters
        let rh1 = RollingHash::from_initial_string("A");
        let rh2 = RollingHash::from_initial_string("B");
        assert_ne!(rh1.get_current_hash(), rh2.get_current_hash());

        // Same starting character
        let rh1 = RollingHash::from_initial_string("Amazon");
        let rh2 = RollingHash::from_initial_string("Amazing");
        assert_ne!(rh1.get_current_hash(), rh2.get_current_hash());
    }
}
