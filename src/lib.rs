use std::collections::VecDeque;

pub type Hash = u64;

pub struct RollingHash {
    current_string: VecDeque<char>,
    current_hash: Hash,
    base_powers: Vec<u64>,
}

impl RollingHash {
    const BASE: u64 = 257;
    const MODULO: u64 = 1_000_000_000 + 7;

    pub fn new() -> Self {
        Self {
            current_string: VecDeque::new(),
            current_hash: 0,
            base_powers: vec![1],
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

        // After we have added a character, we may need to update our
        // precomputed base powers, for use when removing
        // At most, we will need to use BASE^len, where len is the length of the string
        let current_string_len = self.current_string.len();
        let current_base_powers_len = self.base_powers.len();
        if current_string_len > current_base_powers_len {
            let needed = current_string_len - current_base_powers_len;
            for _ in 0..needed {
                // We have constructed it with one value, and we never remove values
                let last_power = self.base_powers.last().unwrap();
                let next_power = (last_power * Self::BASE) % Self::MODULO;
                self.base_powers.push(next_power);
            }
        }
    }

    pub fn remove_front(&mut self) {
        // If we do not have a front char, we do not need to do anything
        if let Some(&front_char) = self.current_string.front() {
            let len = self.current_string.len();
            // We maintain base_powers always updated, so we should
            // always have this value here
            let factor = self.base_powers[len - 1];
            let contribution = (front_char as u64 * factor) % Self::MODULO;
            if contribution > self.current_hash {
                // This operation would underflow, as we are using unsigned integers
                // As we are working with MODULO, we can simply add a MODULO parcel here
                self.current_hash += Self::MODULO;
                // Note that at this point, current_hash could be outside the range of MODULO
                // but this will be fixed with the subtraction below
            }
            self.current_hash -= contribution;
        }
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

    #[test]
    fn hash_changes_with_remove_front() {
        let mut rh = RollingHash::from_initial_string("Eiger");
        let initial_hash = rh.get_current_hash();
        rh.remove_front();
        let new_hash = rh.get_current_hash();
        assert_ne!(initial_hash, new_hash);
    }

    #[test]
    fn hash_collision_example() {
        // `find_hash_collision` found a collision after 684247772 iterations:
        let s1 = "ryIqVm6i3M25uvTttp2Qo8mlkWmKap5PkuWHtS3AZZkRBWCAE9jGCWpkgYHaQobJDJrhdwdoNRGjqQmaTAi5ZGo6hbslnzIL2HaP";
        let s2 = "eVCblKi7jexBFHudJsTfj8ibzxgXGlol8EthCd8OBniEXI6tVR9LFkNzPtNeqR3EIVERZwtG1uxFimT3cPQAHwTTiuRnj6gHh406";
        let rh1 = RollingHash::from_initial_string(s1);
        let rh2 = RollingHash::from_initial_string(s2);
        assert_eq!(rh1.get_current_hash(), rh2.get_current_hash());
    }

    #[test]
    #[ignore] // It takes a while to find a hash collision
    fn find_hash_collision() {
        let mut counter = 0;
        loop {
            // Reference for random string generation: https://stackoverflow.com/a/54277357
            use rand::{distributions::Alphanumeric, Rng};
            let generate_random_string = |len: usize| {
                rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(len)
                    .map(char::from)
                    .collect::<String>()
            };
            let s1 = generate_random_string(100);
            let s2 = generate_random_string(100);
            let rh1 = RollingHash::from_initial_string(&s1);
            let rh2 = RollingHash::from_initial_string(&s2);

            if rh1.current_hash == rh2.current_hash && s1 != s2 {
                println!("Hash collision found after {} iterations", counter);
                println!("s1: {}", s1);
                println!("s2: {}", s2);
                println!("Both hash to: {}", rh1.current_hash);
                break;
            }
            counter += 1;
            // Printing slows down the program
            if counter % 1_000_000 == 0 {
                println!("Iterations: {}", counter);
            }
        }
    }
}
