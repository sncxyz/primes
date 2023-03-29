# primes

A library for generating primes with a segmented sieve of Eratosthenes.

Iterators are used so that memory usage is minimal, and the iterators are lazy.

If all primes below `n` are generated, heap memory is only used to store primes below `sqrt(n)`.
For example, if `n = 100_000_000`, only primes below `10_000` are stored on the heap, of which there are 1229.
This puts the total heap usage by the iterator at 1229 * 8 = 9832 bytes.

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