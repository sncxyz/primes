# primes

A library for generating primes with a segmented sieve of Eratosthenes.

Iterators are used so that memory usage is minimal, and the iterators are lazy.

# Examples

```rust
let first_10: Vec<_> = primes::first(10).collect();
assert_eq!(&first_10, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

let below_30: Vec<_> = primes::below(30).collect();
assert_eq!(&below_30, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

let nth_100 = primes::nth(100);
assert_eq!(nth_100, Some(541));

let divisors_504: Vec<_> = primes::divisors(504).collect();
assert_eq!(&divisors_504, &[(2, 3), (3, 2), (7, 1)]);

let is_prime_53 = primes::is_prime(53);
assert_eq!(is_prime_53, true);
```