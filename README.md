# primes

A library for generating primes with a segmented sieve of Eratosthenes.

Iterators are used so that memory usage is minimal, and the iterators are lazy.

## Examples

```rs
let first_10: Vec<_> = primes::first(10).collect();
assert_eq!(&first_10, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

let below_30: Vec<_> = primes::below(30).collect();
assert_eq!(&below_30, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);

let nth_100 = primes::nth(100);
assert_eq!(nth_100, Some(541));
```