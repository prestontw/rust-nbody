use rust_nbody;
use std::time::{Duration, Instant};

fn time_as_seconds(d: Duration) -> f64 {
    d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9
}

fn main() {
    let start_time = Instant::now();
    let mut current = rust_nbody::init();
    println!(
        "done initializing: {}",
        time_as_seconds(start_time.elapsed())
    );
    let start_time = Instant::now();
    for ts in 0..rust_nbody::NSTEPS {
        current = rust_nbody::compute_forces(current);
        println!(
            "timestep {} complete: {}",
            ts,
            time_as_seconds(start_time.elapsed())
        );
    }
    println!(
        "simulation complete in: {}",
        time_as_seconds(start_time.elapsed())
    );
}
