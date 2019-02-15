use rust_nbody;

fn main() {
    let mut current = rust_nbody::init();
    for ts in 0..rust_nbody::NSTEPS {
        current = rust_nbody::compute_forces(&current, &current);
        println!("timestep {} complete", ts);
    }
    println!("simulation complete");
}
