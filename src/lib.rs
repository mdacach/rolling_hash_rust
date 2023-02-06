use std::collections::VecDeque;

use modular::Modular;

mod modular;

const BIG_PRIME: u64 = 1_000_000_007;

type Numeric = Modular<BIG_PRIME>;
type Hash = Modular<BIG_PRIME>;

pub struct RollingHash {
    current_string: VecDeque<char>,
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
            current_string: VecDeque::new(),
            current_hash: Modular::from_u64(0),
            base_powers: vec![Modular::from_u64(1)],
        }
    }

    pub fn from_initial_string(input: &str) -> Self {
        let mut rh = Self::new();
        input.chars().for_each(|c| rh.push_back(c));
        rh
    }

    pub fn get_current_hash(&self) -> u64 {
        self.current_hash.value
    }

    pub fn push_back(&mut self, c: char) {
        self.current_string.push_back(c);

        self.current_hash = self.current_hash * Self::BASE;
        self.current_hash = self.current_hash + (c as u64);

        // After we have added a character, we may need to update our
        // precomputed base powers, for use when removing
        self.update_base_powers();
    }

    fn update_base_powers(&mut self) {
        // At most, we will need to use BASE^len, where len is the length of the string
        let current_string_len = self.current_string.len();
        let current_base_powers_len = self.base_powers.len();
        if current_string_len >= current_base_powers_len {
            let needed = current_string_len - current_base_powers_len + 1;
            for _ in 0..needed {
                // We have constructed it with one value, and we never remove values
                let &last_power = self.base_powers.last().unwrap();
                let next_power = last_power * Self::BASE;
                self.base_powers.push(next_power);
            }
        }
    }

    pub fn pop_front(&mut self) {
        // If we do not have a front char, we do not need to do anything
        if let Some(&front_char) = self.current_string.front() {
            let len = self.current_string.len();
            // We maintain base_powers always updated, so we should
            // always have this value here
            let factor = self.base_powers[len - 1];
            let contribution = factor * front_char as u64;
            self.current_hash = self.current_hash - contribution;
            self.current_string.pop_front();
        }
    }

    pub fn pop_back(&mut self) {
        // If we do not have a back char, we do not need to do anything
        if let Some(&back_char) = self.current_string.back() {
            // Its contribution is just the value itself
            let contribution = back_char as u64;
            self.current_hash = self.current_hash - contribution;

            // And now we need to "shift" the previous characters, regarding the exponents
            self.current_hash = self.current_hash / Self::BASE;
            self.current_string.pop_back();
        }
    }

    pub fn push_front(&mut self, c: char) {
        let len = self.current_string.len();
        // We should always have base_powers[len], because we update it on both operations
        // that increase the length: push_back() and push_front()
        let factor = self.base_powers[len];
        let contribution = factor * (c as u64);
        self.current_hash = self.current_hash + contribution;
        self.current_string.push_front(c);

        // After we have added a character, we may need to update our
        // precomputed base powers, for use when removing
        self.update_base_powers();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use proptest::proptest;

    use crate::RollingHash;

    fn deque_as_string(vec: VecDeque<char>) -> String {
        vec.iter().collect()
    }

    fn hash_from_string(string: &str) -> u64 {
        let rh = RollingHash::from_initial_string(string);
        rh.get_current_hash()
    }

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
    fn push_characters_to_rolling_hash() {
        let mut rh = RollingHash::new();
        rh.push_back('E');
        rh.push_back('i');
        rh.push_back('g');
        rh.push_back('e');
        rh.push_back('r');
        assert_eq!(deque_as_string(rh.current_string), "Eiger");
    }

    #[test]
    fn hash_changes_with_push() {
        let mut rh = RollingHash::new();
        let initial_hash = rh.get_current_hash();
        rh.push_back('E');
        let new_hash = rh.get_current_hash();
        assert_ne!(initial_hash, new_hash);
    }

    #[test]
    fn string_changes_with_push() {
        let mut rh = RollingHash::new();
        rh.push_back('E');
        let string = deque_as_string(rh.current_string);
        assert_eq!(string, "E");
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
    fn hash_changes_with_pop_front() {
        let mut rh = RollingHash::from_initial_string("Eiger");
        let initial_hash = rh.get_current_hash();
        rh.pop_front();
        let new_hash = rh.get_current_hash();
        assert_ne!(initial_hash, new_hash);
    }

    #[test]
    fn string_changes_with_pop_front() {
        let mut rh = RollingHash::from_initial_string("Eiger");
        rh.pop_front();
        let string = deque_as_string(rh.current_string);
        assert_eq!(string, "iger");
    }

    #[test]
    fn hash_collision_example() {
        // `find_hash_collision` found a collision after 1201640840 iterations:
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
            let rh1 = RollingHash::from_initial_string(&s1);
            let rh2 = RollingHash::from_initial_string(&s2);

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
        let mut rh = RollingHash::from_initial_string("Eiger");
        rh.pop_back();
        let hash_from_popped = rh.get_current_hash();
        let hash_from_string = RollingHash::from_initial_string("Eige").get_current_hash();
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
        let mut rh = RollingHash::from_initial_string("Eiger");
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
        let mut rh = RollingHash::from_initial_string("iger");
        rh.push_front('E');
        let hash_from_pushed = rh.get_current_hash();
        assert_eq!(hash_from_pushed, hash_from_string("Eiger"));
    }

    #[test]
    fn multiple_push_fronts_compute_the_correct_hash() {
        let mut rh = RollingHash::from_initial_string("");
        rh.push_front('r');
        assert_eq!(rh.get_current_hash(), hash_from_string("r"));
        rh.push_front('e');
        assert_eq!(rh.get_current_hash(), hash_from_string("er"));
        rh.push_front('g');
        assert_eq!(rh.get_current_hash(), hash_from_string("ger"));
        rh.push_front('i');
        assert_eq!(rh.get_current_hash(), hash_from_string("iger"));
        rh.push_front('E');
        assert_eq!(rh.get_current_hash(), hash_from_string("Eiger"));
    }

    #[test]
    fn big_string_also_works() {
        // The powers here will surely be bigger than MODULO, so if this works MODULO is ok
        let rh =
            RollingHash::from_initial_string("a b c d e f g h i j k l m n o p q r s t u v w x y z");
        assert_eq!(rh.current_string.len(), 51);
        rh.get_current_hash();
    }

    proptest! {
        #[test]
        fn doesnt_crash(s in "\\PC*") {
            let rh = RollingHash::from_initial_string(&s);
            rh.get_current_hash();
        }
    }
}
