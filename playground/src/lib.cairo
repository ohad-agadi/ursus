use core::poseidon::poseidon_hash_span;


#[executable]
fn main(mut n: felt252) -> felt252 {
    let a = fib(n);
    let mut felt: felt252 = a.into();
    while n != 0 {
        felt = poseidon_hash_span([felt].span());
        n = n - 1;
    }
    felt
}

fn fib(mut n: felt252) -> felt252 {
    let mut a: felt252 = 1;
    let mut b: felt252 = 1;
    while n != 0 {
        n = n - 1;
        let temp = b;
        b = a + b;
        a = temp;
    }
    a
}
