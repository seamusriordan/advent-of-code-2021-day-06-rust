mod tests;

fn advance_state(hist: &mut Vec<u64>) {
    let spawned = hist.remove(0);

    hist[6] += spawned;
    hist.push(spawned);
}

pub fn advance_n_days(state: &Vec<usize>, n: u64) -> u64 {
    let mut hist = vec![0; 9];

    for s in state {
        hist[*s] += 1;
    }

    for _ in 0..n {
        advance_state(&mut hist);
    }

    let mut sum = 0;
    for v in hist {
        sum += v;
    }
    sum
}