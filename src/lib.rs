use std::collections::VecDeque;

use modular::Modular;

mod modular;

const BIG_PRIME: u64 = 1_000_000_007;

type Numeric = Modular<BIG_PRIME>;

pub struct RollingHash {
    current_bytes: VecDeque<u8>,
    current_hash: Numeric,
    base_powers: Vec<Numeric>,
}

impl Default for RollingHash {
    fn default() -> Self {
        Self::new()
    }
}

impl RollingHash {
    const BASE: u64 = 257;

    pub fn new() -> Self {
        Self {
            current_bytes: VecDeque::new(),
            current_hash: Modular::from_u64(0),
            base_powers: vec![Modular::from_u64(1)],
        }
    }

    pub fn from_initial_bytes(input: &[u8]) -> Self {
        let mut rh = Self::new();
        input.iter().for_each(|&c| rh.push_back(c));
        rh
    }

    // For debug purposes
    pub fn get_current_bytes(&self) -> Vec<u8> {
        self.current_bytes.clone().into()
    }

    pub fn get_current_hash(&self) -> u64 {
        self.current_hash.value
    }

    pub fn push_back(&mut self, b: u8) {
        self.current_bytes.push_back(b);

        self.current_hash = self.current_hash * Self::BASE;
        self.current_hash = self.current_hash + (b as u64);

        // After we have added a byte, we may need to update our
        // precomputed base powers, for use when removing
        self.update_base_powers();
    }

    fn update_base_powers(&mut self) {
        // At most, we will need to use BASE^len, where len is the length of the string
        let current_content_len = self.current_bytes.len();
        let current_base_powers_len = self.base_powers.len();
        if current_content_len >= current_base_powers_len {
            let needed = current_content_len - current_base_powers_len + 1;
            for _ in 0..needed {
                // We have constructed it with one value, and we never remove values
                let &last_power = self.base_powers.last().unwrap();
                let next_power = last_power * Self::BASE;
                self.base_powers.push(next_power);
            }
        }
    }

    pub fn pop_front(&mut self) {
        // If we do not have a front byte, we do not need to do anything
        if let Some(&front_byte) = self.current_bytes.front() {
            let len = self.current_bytes.len();
            // We maintain base_powers always updated, so we should
            // always have this value here
            let factor = self.base_powers[len - 1];
            let contribution = factor * front_byte as u64;
            self.current_hash = self.current_hash - contribution;
            self.current_bytes.pop_front();
        }
    }

    pub fn pop_back(&mut self) {
        // If we do not have a back byte, we do not need to do anything
        if let Some(&back_byte) = self.current_bytes.back() {
            // Its contribution is just the value itself
            let contribution = back_byte as u64;
            self.current_hash = self.current_hash - contribution;

            // And now we need to "shift" the previous bytes, regarding the exponents
            self.current_hash = self.current_hash / Self::BASE;
            self.current_bytes.pop_back();
        }
    }

    pub fn push_front(&mut self, b: u8) {
        let len = self.current_bytes.len();
        // We should always have base_powers[len], because we update it on both operations
        // that increase the length: push_back() and push_front()
        let factor = self.base_powers[len];
        let contribution = factor * (b as u64);
        self.current_hash = self.current_hash + contribution;
        self.current_bytes.push_front(b);

        // After we have added a byte, we may need to update our
        // precomputed base powers, for use when removing
        self.update_base_powers();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use proptest::proptest;

    use crate::RollingHash;

    fn same_content(deque: VecDeque<u8>, bytes: &[u8]) -> bool {
        let as_vec: Vec<u8> = deque.into();
        as_vec == bytes.to_vec()
    }

    fn hash_from_string(string: &str) -> u64 {
        let rh = RollingHash::from_initial_bytes(string.as_bytes());
        rh.get_current_hash()
    }

    proptest! {
        #[test]
        fn doesnt_crash(a in 0..255u8, b in 0..255u8, c in 0..255u8) {
            let input = vec![a, b, c];
            let rh = RollingHash::from_initial_bytes(input.as_slice());
            rh.get_current_hash();
        }
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn create_rolling_hash() {
        let _rh = RollingHash::new();
        let _rh_from_string = RollingHash::from_initial_bytes("Eiger".as_bytes());
    }

    #[test]
    fn push_bytes_to_rolling_hash() {
        let mut rh = RollingHash::new();
        rh.push_back(b'E');
        rh.push_back(b'i');
        rh.push_back(b'g');
        rh.push_back(b'e');
        rh.push_back(b'r');
        assert!(same_content(rh.current_bytes, b"Eiger"));
    }

    #[test]
    fn hash_changes_with_push() {
        let mut rh = RollingHash::new();
        let initial_hash = rh.get_current_hash();
        rh.push_back(b'E');
        let new_hash = rh.get_current_hash();
        assert_ne!(initial_hash, new_hash);
    }

    #[test]
    fn content_changes_with_push() {
        let mut rh = RollingHash::new();
        rh.push_back(b'E');
        let as_vec: Vec<u8> = rh.current_bytes.into();
        assert_eq!(as_vec, b"E");
    }

    #[test]
    fn hash_for_equal_strings_are_equal() {
        let rh1 = RollingHash::from_initial_bytes(b"Eiger");
        let rh2 = RollingHash::from_initial_bytes(b"Eiger");
        assert_eq!(rh1.get_current_hash(), rh2.get_current_hash());
    }

    #[test]
    fn hash_for_different_strings_are_different() {
        // Complete different strings
        let rh1 = RollingHash::from_initial_bytes(b"Eiger");
        let rh2 = RollingHash::from_initial_bytes(b"Matheus");
        assert_ne!(rh1.get_current_hash(), rh2.get_current_hash());

        // Different strings with same length
        let rh1 = RollingHash::from_initial_bytes(b"Eiger");
        let rh2 = RollingHash::from_initial_bytes(b"Great");
        assert_ne!(rh1.get_current_hash(), rh2.get_current_hash());

        // Single characters
        let rh1 = RollingHash::from_initial_bytes(b"A");
        let rh2 = RollingHash::from_initial_bytes(b"B");
        assert_ne!(rh1.get_current_hash(), rh2.get_current_hash());

        // Same starting character
        let rh1 = RollingHash::from_initial_bytes(b"Amazon");
        let rh2 = RollingHash::from_initial_bytes(b"Amazing");
        assert_ne!(rh1.get_current_hash(), rh2.get_current_hash());
    }

    #[test]
    fn hash_changes_with_pop_front() {
        let mut rh = RollingHash::from_initial_bytes(b"Eiger");
        let initial_hash = rh.get_current_hash();
        rh.pop_front();
        let new_hash = rh.get_current_hash();
        assert_ne!(initial_hash, new_hash);
    }

    #[test]
    fn content_changes_with_pop_front() {
        let mut rh = RollingHash::from_initial_bytes(b"Eiger");
        rh.pop_front();
        let as_vec: Vec<u8> = rh.current_bytes.into();
        assert_eq!(as_vec, b"iger");
    }

    #[test]
    fn hash_collision_example() {
        // `find_hash_collision` found a collision after 1201640840 iterations:
        let s1 = "ryIqVm6i3M25uvTttp2Qo8mlkWmKap5PkuWHtS3AZZkRBWCAE9jGCWpkgYHaQobJDJrhdwdoNRGjqQmaTAi5ZGo6hbslnzIL2HaP";
        let s2 = "eVCblKi7jexBFHudJsTfj8ibzxgXGlol8EthCd8OBniEXI6tVR9LFkNzPtNeqR3EIVERZwtG1uxFimT3cPQAHwTTiuRnj6gHh406";
        let rh1 = RollingHash::from_initial_bytes(s1.as_bytes());
        let rh2 = RollingHash::from_initial_bytes(s2.as_bytes());
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
            let rh1 = RollingHash::from_initial_bytes(s1.as_bytes());
            let rh2 = RollingHash::from_initial_bytes(s2.as_bytes());

            if rh1.current_hash == rh2.current_hash && s1 != s2 {
                println!("Hash collision found after {counter} iterations");
                println!("s1: {s1}");
                println!("s2: {s2}");
                println!("Both hash to: {:?}", rh1.current_hash);
                break;
            }
            counter += 1;
            // Printing slows down the program
            if counter % 1_000_000 == 0 {
                println!("Iterations: {counter}");
            }
        }
    }

    #[test]
    #[ignore] // It takes a while to find a hash collision
    fn find_hash_collision_unicode() {
        let mut counter = 0;
        loop {
            // Reference for random string generation: https://stackoverflow.com/a/54277357
            use rand::{distributions::Standard, Rng};
            let generate_random_string = |len: usize| {
                rand::thread_rng()
                    .sample_iter::<char, _>(Standard)
                    .take(len)
                    .collect::<String>()
            };
            let s1 = generate_random_string(100);
            let s2 = generate_random_string(100);
            let rh1 = RollingHash::from_initial_bytes(s1.as_bytes());
            let rh2 = RollingHash::from_initial_bytes(s2.as_bytes());

            if rh1.current_hash == rh2.current_hash && s1 != s2 {
                println!("Hash collision found after {counter} iterations");
                println!("s1: {s1}");
                println!("s2: {s2}");
                println!("Both hash to: {:?}", rh1.current_hash);
                break;
            }
            counter += 1;
            // Printing slows down the program
            if counter % 1_000_000 == 0 {
                println!("Iterations: {counter}");
            }
        }
    }

    #[test]
    fn pop_back_computes_the_correct_hash() {
        let mut rh = RollingHash::from_initial_bytes(b"Eiger");
        rh.pop_back();
        let hash_from_popped = rh.get_current_hash();
        let hash_from_string = RollingHash::from_initial_bytes(b"Eige").get_current_hash();
        assert_eq!(hash_from_popped, hash_from_string);
    }

    #[test]
    fn pop_back_on_empty_does_nothing() {
        let mut rh = RollingHash::new();
        let initial_hash = rh.get_current_hash();
        rh.pop_back();
        let new_hash = rh.get_current_hash();
        assert_eq!(initial_hash, new_hash);
    }

    #[test]
    fn multiple_pop_backs_compute_the_correct_hash() {
        let mut rh = RollingHash::from_initial_bytes(b"Eiger");
        rh.pop_back();
        assert_eq!(rh.get_current_hash(), hash_from_string("Eige"));
        rh.pop_back();
        assert_eq!(rh.get_current_hash(), hash_from_string("Eig"));
        rh.pop_back();
        assert_eq!(rh.get_current_hash(), hash_from_string("Ei"));
        rh.pop_back();
        assert_eq!(rh.get_current_hash(), hash_from_string("E"));
        rh.pop_back();
        assert_eq!(rh.get_current_hash(), hash_from_string(""));
    }

    #[test]
    fn push_front_computes_the_correct_hash() {
        let mut rh = RollingHash::from_initial_bytes(b"iger");
        rh.push_front(b'E');
        let hash_from_pushed = rh.get_current_hash();
        assert_eq!(hash_from_pushed, hash_from_string("Eiger"));
    }

    #[test]
    fn multiple_push_fronts_compute_the_correct_hash() {
        let mut rh = RollingHash::from_initial_bytes(b"");
        rh.push_front(b'r');
        assert_eq!(rh.get_current_hash(), hash_from_string("r"));
        rh.push_front(b'e');
        assert_eq!(rh.get_current_hash(), hash_from_string("er"));
        rh.push_front(b'g');
        assert_eq!(rh.get_current_hash(), hash_from_string("ger"));
        rh.push_front(b'i');
        assert_eq!(rh.get_current_hash(), hash_from_string("iger"));
        rh.push_front(b'E');
        assert_eq!(rh.get_current_hash(), hash_from_string("Eiger"));
    }

    #[test]
    fn big_string_also_works() {
        // The powers here will surely be bigger than MODULO, so if this works MODULO is ok
        let rh =
            RollingHash::from_initial_bytes(b"a b c d e f g h i j k l m n o p q r s t u v w x y z");
        assert_eq!(rh.current_bytes.len(), 51);
        rh.get_current_hash();
    }
}
