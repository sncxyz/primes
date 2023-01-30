//! A library for generating primes with a segmented sieve of Eratosthenes.
//!
//! Iterators are used so that memory usage is minimal, and the iterators are lazy.
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
//! let nth_100 = primes::nth(100);
//! assert_eq!(nth_100, Some(541));
//! ```

const SIZE: usize = 64_000;

/// Returns an iterator that yields the first `n` primes.
///
/// # Examples
///
/// ```
/// let first_10: Vec<_> = primes::first(10).collect();
/// assert_eq!(&first_10, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
/// ```
#[inline(always)]
pub fn first(n: u64) -> Primes {
    Primes::new(n)
}

/// Returns an iterator that yields the primes less than or equal to `n`.
///
/// # Examples
///
/// ```
/// let below_30: Vec<_> = primes::below(30).collect();
/// assert_eq!(&below_30, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
/// ```
#[inline(always)]
pub fn below(n: u64) -> PrimesBelow {
    PrimesBelow::new(n)
}

// Returns the `n`th prime, with `primes::nth(1) = Some(2)`.
///
/// # Examples
///
/// ```
/// let nth_100 = primes::nth(100);
/// assert_eq!(nth_100, Some(541));
/// ```
#[inline(always)]
pub fn nth(n: u64) -> Option<u64> {
    first(n).last()
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
        if self.count == 0 {
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
                if next * next <= self.limit {
                    self.primes.push(next);
                }
                return Some(next);
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
    fn new(n: u64) -> Self {
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

pub struct PrimesBelow {
    primes: Vec<u64>,
    sieve: Sieve,
    p: u64,
    limit: u64,
}

impl Iterator for PrimesBelow {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p >= self.limit {
            return None;
        }
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

impl PrimesBelow {
    fn new(n: u64) -> Self {
        Self {
            primes: vec![3],
            sieve: Sieve::new(3),
            p: 1,
            limit: n,
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
