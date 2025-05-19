use core::poseidon::poseidon_hash_span;


#[executable]
fn main() -> felt252 {
    let a = fib(16);
    let mut felt: felt252 = a.into();
    let mut n: u32 = 100;
    while n != 0 {
        felt = poseidon_hash_span([felt].span());
        n = n - 1;
    }
    felt
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
