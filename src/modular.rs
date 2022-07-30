#[derive(Debug)]
struct Modular<const MOD: u64> {
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
