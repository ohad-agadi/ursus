use core::poseidon::poseidon_hash_span;

#[executable]
fn main() -> felt252 {
    let a = fib(16);
    let felt: felt252 = a.into();
    poseidon_hash_span([felt].span())
}

fn fib(mut n: u32) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 1;
    while n != 0 {
        n = n - 1;
        let temp = b;
        b = a + b;
        a = temp;
    }
    a
}
