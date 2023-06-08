//! A small library for generating primes with a segmented sieve of Eratosthenes.
//!
//! Iterators are used so that memory usage is minimal, and the iterators are lazy.
//!
//! If all primes below `n` are generated, heap memory is only used to store primes below `sqrt(n)`.
//! For example, if `n = 100_000_000`, only primes below `10_000` are stored on the heap, of which there are 1229.
//! This puts the total heap usage by the iterator at 1229 * 8 = 9832 bytes (although `Vec` could allocate up to double that).
//!
//! Takes roughly 3 seconds to iterate the first 100,000,000 primes on my machine.
//!
//! # Examples
//!
//! ```
//! let first_10: Vec<_> = primes::first(10).collect();
//! assert_eq!(&first_10, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
//!
//! let below_30: Vec<_> = primes::below(30).collect();
//! assert_eq!(&below_30, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
//!
//! assert_eq!(primes::nth(100), Some(541));
//!
//! let divisors_504: Vec<_> = primes::divisors(504).collect();
//! assert_eq!(&divisors_504, &[(2, 3), (3, 2), (7, 1)]);
//!
//! assert!(primes::is_prime(53));
//! assert!(!primes::is_prime(51));
//! ```

const SIZE: usize = 64_000;

/// Returns an iterator over the first `n` primes.
///
/// # Examples
///
/// ```
/// let first_10: Vec<_> = primes::first(10).collect();
/// assert_eq!(&first_10, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
/// ```
#[inline(always)]
pub fn first(n: u64) -> Primes {
    Primes::first(n)
}

/// Returns an iterator over the primes less than or equal to `n`.
///
/// # Examples
///
/// ```
/// let below_30: Vec<_> = primes::below(30).collect();
/// assert_eq!(&below_30, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
/// ```
#[inline(always)]
pub fn below(n: u64) -> Primes {
    Primes::below(n)
}

/// Returns the `n`th prime, with `primes::nth(1) = Some(2)`.
///
/// # Examples
///
/// ```
/// assert_eq!(primes::nth(100), Some(541));
/// ```
#[inline(always)]
pub fn nth(n: u64) -> Option<u64> {
    first(n).last()
}

/// Returns an iterator over the prime divisors of `n`, and their exponents.
///
/// e.g. `(2, 4)` means the prime `2` divides `n` with exponent `4`.
///
/// # Examples
///
/// ```
/// let divisors_504: Vec<_> = primes::divisors(504).collect();
/// assert_eq!(&divisors_504, &[(2, 3), (3, 2), (7, 1)]);
///
/// let divisors_25: Vec<_> = primes::divisors(25).collect();
/// assert_eq!(&divisors_25, &[(5, 2)]);
///
/// let divisors_53: Vec<_> = primes::divisors(53).collect();
/// assert_eq!(&divisors_53, &[(53, 1)]);
/// ```
#[inline(always)]
pub fn divisors(n: u64) -> Divisors {
    Divisors::new(n)
}

/// Returns `true` if `n` is prime, or `false` otherwise.
///
/// # Examples
///
/// ```
/// assert!(!primes::is_prime(504));
///
/// assert!(!primes::is_prime(25));
///
/// assert!(primes::is_prime(53));
///
/// assert!(!primes::is_prime(51));
///
/// assert!(primes::is_prime(541));
/// ```
#[inline(always)]
pub fn is_prime(n: u64) -> bool {
    divisors(n).next().map_or(false, |d| d.0 == n)
}

pub struct Primes {
    primes: Vec<u64>,
    sieve: Sieve,
    p: u64,
    count: u64,
    limit: u64,
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 || self.p >= self.limit {
            return None;
        }
        self.count -= 1;
        if self.p < 3 {
            self.p += 1;
            return Some(self.p);
        }
        if self.sieve.start == 3 && self.p * self.p <= self.sieve.end {
            self.sieve.sieve(self.p);
        }
        loop {
            if let Some(next) = self.sieve.next_prime() {
                self.p = next;
                if next > self.limit {
                    return None;
                }
                if next * next <= self.limit {
                    self.primes.push(next);
                }
                return Some(next);
            }
            if self.sieve.end + 1 >= self.limit {
                self.p = self.limit;
                return None;
            }
            self.sieve.slide();
            for &p in &self.primes {
                if p * p > self.sieve.end {
                    break;
                }
                self.sieve.sieve(p);
            }
        }
    }
}

impl Primes {
    fn below(n: u64) -> Self {
        Self {
            primes: vec![3],
            sieve: Sieve::new(3),
            p: 1,
            count: u64::MAX,
            limit: n,
        }
    }

    fn first(n: u64) -> Self {
        let limit = if n > 5 {
            let f = n as f64;
            let log = f.ln();
            (f * (log + log.ln())) as u64
        } else {
            11
        };
        Self {
            primes: vec![3],
            sieve: Sieve::new(3),
            p: 1,
            count: n,
            limit,
        }
    }
}

struct Sieve {
    sieve: [State; SIZE],
    start: u64,
    end: u64,
    current: u64,
}

impl Sieve {
    fn new(start: u64) -> Self {
        Self {
            sieve: [State::Prime; SIZE],
            start,
            end: start + SIZE as u64 * 2 - 2,
            current: 0,
        }
    }

    fn slide(&mut self) {
        self.sieve.fill(State::Prime);
        self.start += SIZE as u64 * 2;
        self.end += SIZE as u64 * 2;
        self.current = u64::MAX;
    }

    fn next_prime(&mut self) -> Option<u64> {
        self.current = self.current.wrapping_add(1);
        while self.current < SIZE as u64 {
            if self.sieve[self.current as usize] == State::Prime {
                return Some(self.start + self.current * 2);
            }
            self.current += 1;
        }
        None
    }

    fn sieve(&mut self, p: u64) {
        let sq = p * p;
        let mut c = if sq >= self.start {
            sq
        } else {
            let q = self.start / p;
            let m = q * p;
            if q % 2 == 0 {
                m + p
            } else if m == self.start {
                m
            } else {
                m + p * 2
            }
        };
        c = (c - self.start) / 2;
        while c < SIZE as u64 {
            self.sieve[c as usize] = State::Composite;
            c += p;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Prime,
    Composite,
}

pub struct Divisors {
    n: u64,
    primes: Primes,
}

impl Divisors {
    fn new(n: u64) -> Self {
        let sqrt = (n as f64).sqrt() as u64;
        Self {
            n,
            primes: below(sqrt),
        }
    }
}

impl Iterator for Divisors {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.n <= 1 {
                return None;
            }
            if let Some(prime) = self.primes.next() {
                if prime * prime > self.n {
                    let prime = self.n;
                    self.n = 1;
                    return Some((prime, 1));
                }
                let mut exponent = 0;
                while self.n % prime == 0 {
                    self.n /= prime;
                    exponent += 1;
                }
                if exponent > 0 {
                    return Some((prime, exponent));
                }
            } else {
                let prime = self.n;
                self.n = 1;
                return Some((prime, 1));
            }
        }
    }
}
