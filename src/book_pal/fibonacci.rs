use num::BigUint;

struct Fibonacci(BigUint, BigUint);

impl Iterator for Fibonacci {

    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.0.clone();

        self.0 = self.1.clone();
        self.1 = cur.clone() + self.1.clone();

        Some(cur)
    }

}

pub fn all_fibonacci_numbers() -> impl Iterator<Item = BigUint> {
    let arg1 = BigUint::from(1u32);
    let arg2 = BigUint::from(1u32);
    Fibonacci(arg1, arg2)
}