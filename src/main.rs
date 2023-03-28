use std::time::Instant;

fn main() {
    for (n, prime) in [
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
        let result = primes::nth(n).unwrap();
        let time = now.elapsed().as_nanos() as f64 / 1_000_000.;
        if result == prime {
            println!("{n}th succeeded in {time}ms");
        } else {
            println!("{n}th failed");
            println!("Expected: {}", prime);
            println!("Got: {}", result);
            return;
        }
    }
}
