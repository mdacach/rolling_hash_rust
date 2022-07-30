#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
pub struct Modular<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> std::ops::Add for Modular<MOD> {
    type Output = Modular<MOD>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: (self.value + rhs.value) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::Add<u64> for Modular<MOD> {
    type Output = Modular<MOD>;

    fn add(self, rhs: u64) -> Self::Output {
        Self::Output {
            value: (self.value + rhs) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::Mul for Modular<MOD> {
    type Output = Modular<MOD>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: (self.value * rhs.value) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::Mul<u64> for Modular<MOD> {
    type Output = Modular<MOD>;

    fn mul(self, rhs: u64) -> Self::Output {
        Self::Output {
            value: (self.value * rhs) % MOD,
        }
    }
}

impl<const MOD: u64> std::ops::Sub for Modular<MOD> {
    type Output = Modular<MOD>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut value = self.value;
        if rhs.value > self.value {
            value += MOD;
        }
        value -= rhs.value;

        Self::Output { value }
    }
}

impl<const MOD: u64> std::ops::Sub<u64> for Modular<MOD> {
    type Output = Modular<MOD>;

    fn sub(self, rhs: u64) -> Self::Output {
        let mut value = self.value;
        if rhs > self.value {
            value += MOD;
        }
        value -= rhs;

        Self::Output { value }
    }
}

impl<const MOD: u64> std::ops::Div for Modular<MOD> {
    type Output = Modular<MOD>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        let inverse = Self::Output::find_modular_inverse(rhs.value);

        Self::Output {
            value: (self * inverse).value,
        }
    }
}

impl<const MOD: u64> std::ops::Div<u64> for Modular<MOD> {
    type Output = Modular<MOD>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: u64) -> Self::Output {
        let inverse = Self::Output::find_modular_inverse(rhs);

        Self::Output {
            value: (self * inverse).value,
        }
    }
}

impl<const MOD: u64> Modular<MOD> {
    pub fn from_u64(number: u64) -> Self {
        Self {
            value: number % MOD,
        }
    }

    // Division is tricky under modulo, we need to actually multiply by the modular multiplicative inverse
    // See: https://cp-algorithms.com/algebra/module-inverse.html
    fn find_modular_inverse(number: u64) -> u64 {
        // TODO: I have never really understood this
        // Reference: https://cp-algorithms.com/algebra/module-inverse.html#finding-the-modular-inverse-using-binary-exponentiation
        Self::fast_exponentiation(number, MOD - 2)
    }

    // Uses Modulo
    fn fast_exponentiation(mut base: u64, mut exponent: u64) -> u64 {
        let is_last_bit_on = |x| (x & 1) == 1;

        let mut result = 1;
        while exponent != 0 {
            if is_last_bit_on(exponent) {
                result *= base;
                result %= MOD;
            }
            base *= base;
            base %= MOD;
            exponent >>= 1; // Shift the bits
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::modular::Modular;

    #[test]
    fn add_modular() {
        let lhs = Modular::<25> { value: 10 };
        let rhs = Modular::<25> { value: 20 };
        assert_eq!((lhs + rhs).value, 5);
    }

    #[test]
    fn add_u64() {
        let lhs = Modular::<25> { value: 10 };
        let rhs: u64 = 20;
        assert_eq!((lhs + rhs).value, 5);
    }

    #[test]
    fn multiply_modular() {
        let lhs = Modular::<25> { value: 5 };
        let rhs = Modular::<25> { value: 6 };
        assert_eq!((lhs * rhs).value, 5);
    }

    #[test]
    fn multiply_u64() {
        let lhs = Modular::<25> { value: 5 };
        let rhs: u64 = 6;
        assert_eq!((lhs * rhs).value, 5);
    }

    #[test]
    fn subtract_modular() {
        let lhs = Modular::<25> { value: 10 };
        let rhs = Modular::<25> { value: 15 };
        assert_eq!((lhs - rhs).value, 20);
    }

    #[test]
    fn subtract_u64() {
        let lhs = Modular::<25> { value: 10 };
        let rhs: u64 = 15;
        assert_eq!((lhs - rhs).value, 20);
    }

    #[test]
    fn fast_exponentiation_works() {
        const BIG_PRIME: u64 = 1_000_000_007;
        type M = Modular<BIG_PRIME>;
        assert_eq!(M::fast_exponentiation(2, 3), 8);
        assert_eq!(M::fast_exponentiation(2, 0), 1);
        assert_eq!(M::fast_exponentiation(10, 2), 100);
        // Big numbers also work
        assert!(M::fast_exponentiation(257, 143) < BIG_PRIME);
        assert_eq!(M::fast_exponentiation(257, 4), 362470373);
    }

    #[test]
    fn modular_multiplicative_inverse_works() {
        const BIG_PRIME: u64 = 1_000_000_007;
        assert_eq!(Modular::<BIG_PRIME>::find_modular_inverse(200), 285000002);
        assert_eq!((200 * 285000002) % BIG_PRIME, 1);
    }

    #[test]
    fn divide_modular() {
        let lhs = Modular::<23> { value: 8 };
        let rhs = Modular::<23> { value: 5 };
        let div = lhs / rhs;
        assert_eq!(div * rhs, lhs);
    }

    #[test]
    fn divide_u64() {
        let lhs = Modular::<23> { value: 8 };
        let rhs: u64 = 5;
        let div = lhs / rhs;
        assert_eq!(div * rhs, lhs);
    }
}
