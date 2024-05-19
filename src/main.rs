// Binary exponential back-off, as used in Ethernet
// (https://en.wikipedia.org/wiki/Exponential_backoff#Example).)
fn generate_beb() {
    const RETRIES: usize = 5;
    const LEN: usize = 1 << RETRIES;

    // Start off with 0th retry all in a single timeslot.
    let mut density = vec![0.0; LEN];
    density[0] = 1.0;
    let mut v = Vec::new();
    v.push(density);

    // Generate densities over the retry iterations.
    for retry in 1..RETRIES {
	let prev = v.last().unwrap();
	let mut density = vec![0.0; LEN];
	
	// Number of slots over which the retries will be spread out.
	let smear = 1 << retry;
	let fract = (smear as f64).recip();

	// For each previous timestep, evenly smear out the
	// distribution over potential retry slots.
	for (idx, d) in prev[..prev.len() - (smear + 1)].iter().enumerate() {
	    for tgt in density[idx+1..][..smear].iter_mut() {
		*tgt += d * fract;
	    }
	}

	v.push(density);
    }

    // Print out in a nice CSV format.
    print!("Timeslot");
    for i in 0..RETRIES {
	print!(",Retry {}", i);
    }
    println!(",Total");

    for t in 0..LEN {
	print!("{}", t);
	let mut sum = 0.0;
	for retry in 0..RETRIES {
	    print!(",{:.5}", v[retry][t]);
	    sum += v[retry][t];
	}
	println!(",{:.5}", sum);
    }
}

fn main() {
    generate_beb();
}
