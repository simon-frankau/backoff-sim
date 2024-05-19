use rand;
use rand::Rng;

const RETRIES: usize = 5;

// Binary exponential back-off, as used in Ethernet
// (https://en.wikipedia.org/wiki/Exponential_backoff#Example).)
fn generate_beb() -> Vec<Vec<f64>> {
    const LEN: usize = 1 << (RETRIES + 1);

    // Start off with 0th retry all in a single timeslot.
    let mut density = vec![0.0; LEN];
    density[0] = 1.0;
    let mut v = Vec::new();
    v.push(density);

    // Generate densities over the retry iterations.
    for retry in 1..=RETRIES {
        let prev = v.last().unwrap();
        let mut density = vec![0.0; LEN];

        // Number of slots over which the retries will be spread out.
        let smear = 1 << retry;
        let fract = (smear as f64).recip();

        // For each previous timestep, evenly smear out the
        // distribution over potential retry slots.
        for (idx, d) in prev[..prev.len() - (smear + 1)].iter().enumerate() {
            for tgt in density[idx + 1..][..smear].iter_mut() {
                *tgt += d * fract;
            }
        }

        v.push(density);
    }

    v
}

// Modified binary exponential back-off: The next timer starts from
// the end of the current window, not immediately after a failed
// retry. Means a client should always make n retry attempts before
// the 2^n-1 th time slot.
fn generate_mbeb() -> Vec<Vec<f64>> {
    const LEN: usize = 1 << (RETRIES + 1);

    // Start off with 0th retry all in a single timeslot.
    let mut density = vec![0.0; LEN];
    density[0] = 1.0;
    let mut v = Vec::new();
    v.push(density);

    // Generate densities over the retry iterations.
    for retry in 1..=RETRIES {
        let mut density = vec![0.0; LEN];

        // Number of slots over which the retries will be spread out.
        let smear = 1 << retry;
        let fract = (smear as f64).recip();

        // All retries smeared from the end of the time window.
        let idx = (1 << retry) - 1;
        for tgt in density[idx..][..smear].iter_mut() {
            *tgt += fract;
        }

        v.push(density);
    }

    v
}

fn print_beb(v: &[Vec<f64>]) {
    // Print out in a nice CSV format.
    print!("Timeslot");
    for i in 1..=RETRIES {
        print!(",Retry {}", i);
    }
    println!(",Total");

    for t in 0..v[0].len() {
        print!("{}", t);
        let mut sum = 0.0;
        for retry in 1..=RETRIES {
            print!(",{:.5}", v[retry][t]);
            sum += v[retry][t];
        }
        println!(",{:.5}", sum);
    }
}

fn print_beb_comparison(beb: &[Vec<f64>], mbeb: &[Vec<f64>]) {
    fn sum_transpose(v: &[Vec<f64>]) -> Vec<f64> {
        let mut res = Vec::new();
        for row in 0..v[0].len() {
            res.push(v.iter().map(|col| col[row]).sum());
        }
        res
    }

    let beb_sums = sum_transpose(beb);
    let mbeb_sums = sum_transpose(mbeb);

    // Print out in a nice CSV format.
    println!("Timeslot,BEB,MBEB");

    for (idx, (beb_sum, mbeb_sum)) in beb_sums.iter().zip(mbeb_sums.iter()).enumerate() {
        println!("{},{:.5},{:.5}", idx, beb_sum, mbeb_sum);
    }
}

// Implement exponential back-off with jitter, as seen in
// com.google.api.client.util.ExponentialBackOff.java.
fn generate_ebj() {
    // As the buckets don't align nicely with the
    // pretty-much-continuous nature of the retry timing in this
    // algorithm, the easiest good-enough solution is to just run a
    // simulation and collect the histogram.
    const RETRIES: usize = 5;
    const PATHS: usize = 500000000;
    const BUCKETS: usize = 200;

    const INITIAL_INTERVAL: f64 = 0.5;
    const MULTIPLIER: f64 = 1.5;
    const RAND_FACTOR: f64 = 0.5;

    let max_time = INITIAL_INTERVAL
        * MULTIPLIER.powi((RETRIES - 1) as i32)
        * (1.0 + RAND_FACTOR)
        * (1.0 - MULTIPLIER.recip()).recip();
    let bucket_size = max_time / BUCKETS as f64;

    let mut histograms = (0..RETRIES).map(|_| vec![0.0; BUCKETS]).collect::<Vec<_>>();

    let mut rng = rand::thread_rng();
    for path in 0..PATHS {
        if path % 1000000 == 0 {
            eprintln!("Path {}", path);
        }
        let mut t = 0.0;
        let mut interval = INITIAL_INTERVAL;
        for retry in 0..RETRIES {
            let actual_interval = interval * rng.gen_range(1.0 - RAND_FACTOR..=1.0 + RAND_FACTOR);
            interval *= MULTIPLIER;
            t += actual_interval;
            histograms[retry][(t / bucket_size) as usize] += 1.0;
        }
    }

    // To make comparison with BEB a little easier, we normalise the
    // peak of the first retry to magnitude 0.5 (the same as in BEB).
    let factor = 0.5
        * histograms[0]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
            .recip();
    for retry in histograms.iter_mut() {
        for entry in retry.iter_mut() {
            *entry *= factor;
        }
    }

    // Print out in a nice CSV format.
    print!("Time");
    for i in 1..=RETRIES {
        print!(",Retry {}", i);
    }
    println!(",Total");

    for t in 0..BUCKETS {
        print!("{}", t as f64 * bucket_size);
        let mut sum = 0.0;
        for retry in 0..RETRIES {
            print!(",{:.5}", histograms[retry][t]);
            sum += histograms[retry][t];
        }
        println!(",{:.5}", sum);
    }
}

fn main() {
    let beb_histogram = generate_beb();
    // print_beb(&beb_histogram);
    let mbeb_histogram = generate_mbeb();
    // print_beb(&mbeb_histogram);
    print_beb_comparison(&beb_histogram, &mbeb_histogram);
    // generate_ebj();
}
