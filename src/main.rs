use std::time::Instant;

const TEN: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

fn main() {
    for i in 0..=10 {
        let primes: Vec<_> = primes::first(i as u64).collect();
        if &primes != &TEN[..i] {
            println!("First {i} failed");
            println!("Expected: {:?}", &TEN[..i]);
            println!("Got: {:?}", primes);
            return;
        }
    }
    println!("First passed");
    let mut j = 0;
    for i in 0..=29 {
        if TEN[j] == i {
            j += 1;
        }
        let primes: Vec<_> = primes::below(i).collect();
        if &primes != &TEN[..j] {
            println!("Below {i} failed");
            println!("Expected: {:?}", &TEN[..j]);
            println!("Got: {:?}", primes);
            return;
        }
    }
    println!("Below passed");
    for (i, x) in [
        (5, 11),
        (10, 29),
        (25, 97),
        (50, 229),
        (100, 541),
        (1_000, 7_919),
        (10_000, 104_729),
        (100_000, 1_299_709),
        (1_000_000, 15_485_863),
        (10_000_000, 179_424_673),
        (100_000_000, 2_038_074_743),
    ] {
        let now = Instant::now();
        let prime = primes::nth(i).unwrap();
        let time = now.elapsed().as_nanos() as f64 / 1_000_000.;
        if prime == x {
            println!("{i}th succeeded in {time}ms");
        } else {
            println!("{i}th failed");
            println!("Expected: {}", x);
            println!("Got: {}", prime);
            return;
        }
    }
}
