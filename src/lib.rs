//! Implementing the N-body program in Rust (with a little bit of EC(S) thrown in).
//! 
//! This is almost a direct translation from an assignment in a Parallel Systems
//! course I took algorithm-wise.
//! The interesting part of that assignment is that we had to use UPC—the
//! interesting parts of this implementation are:
//! * it's in Rust,
//! * it's a little optimized, and
//! * it has the data layout of an EC(S) implementation.

use rayon::prelude::*;

const N: usize = 10000;
const G: f64 = 6.67e-11;
const TIMESTEP: f64 = 0.25;
/// The number of steps to simulate.
pub const NSTEPS: usize = 10;

struct Position {
  x: f64,
  y: f64,
  z: f64,
}
struct Velocity {
  dx: f64,
  dy: f64,
  dz: f64,
}
struct Force {
  fx: f64,
  fy: f64,
  fz: f64,
}
struct Acceleration {
  ax: f64,
  ay: f64,
  az: f64,
}

/// The main structure of this program.
/// The state for a body is split across
/// a position vector,
/// a velocities vector,
/// and a mass vector.
/// 
/// Originally, this was implemented as an array of structures—now
/// it's a structure of arrays.
/// This was both for testing optimizations and for minute practice with EC(S).
pub struct BodyStates {
  poss: Vec<Position>,
  vels: Vec<Velocity>,
  masses: Vec<f64>,
}

/// Given a point from the origin as `f64`'s,
/// calculate its distance from the origin.
fn dist(dx: f64, dy: f64, dz: f64) -> f64 {
  dist_squared(dx, dy, dz).sqrt()
}

fn dist_squared(dx: f64, dy: f64, dz: f64) -> f64 {
  ((dx * dx) + (dy * dy) + (dz * dz))
}

/// Given the masses of two bodies and the distance between them,
/// calculate the force between them.
fn force(mass1: f64, mass2: f64, distance: f64) -> f64 {
  force_d(mass1, mass2, distance * distance)
}

fn force_d(mass1: f64, mass2: f64, distance_squared: f64) -> f64 {
  (G * mass1 * mass2) / distance_squared
}

/// Given the position and mass of one body,
/// calculate the force acting on it from all of the other bodies.
fn forces_for_body<'a, I>(p: &Position, m: f64, reference: I) -> Force
where
  I: rayon::iter::IntoParallelIterator<Item = (&'a Position, &'a f64)>,
{
  reference
    .into_par_iter()
    .map(|(ref otherpos, &othermass)| {
      let dx = p.x - otherpos.x;
      let dy = p.y - otherpos.y;
      let dz = p.z - otherpos.z;

      let d = dist_squared(dx, dy, dz);
      let f = force_d(m, othermass, d);

      Force {
        fx: (f * dx) / d,
        fy: (f * dy) / d,
        fz: (f * dz) / d,
      }
    })
    .reduce(
      || Force {
        fx: 0.,
        fy: 0.,
        fz: 0.,
      },
      |acc: Force, f: Force| Force {
        fx: acc.fx + f.fx,
        fy: acc.fy + f.fy,
        fz: acc.fz + f.fz,
      },
    )
}

/// Calculate the accelerations for these bodies—used to update the bodies' velocities.
fn accelerations(bs: &BodyStates) -> Vec<Acceleration> {
  bs.poss
    .par_iter()
    .zip(bs.masses.par_iter())
    .map(|(ref p, &m)| {
      let reference = bs.poss.par_iter().zip(bs.masses.par_iter());

      // calculate forces over all other things
      let Force { fx, fy, fz } = forces_for_body(p, m, reference);
      Acceleration {
        ax: fx / m,
        ay: fy / m,
        az: fz / m,
      }
    })
    .collect()
}

/// Returns a new `Position` from a `Position` moving at a certain `Velocity`.
fn move_position(p: &Position, v: &Velocity) -> Position {
  Position {
    x: p.x + v.dx * TIMESTEP,
    y: p.y + v.dy * TIMESTEP,
    z: p.z + v.dz * TIMESTEP,
  }
}

/// Returns a new `Velocity` from a `Velocity` accelerating at a certain `Acceleration`.
fn update_velocity(v: &Velocity, a: &Acceleration) -> Velocity {
  Velocity {
    dx: v.dx + a.ax * TIMESTEP,
    dy: v.dy + a.ay * TIMESTEP,
    dz: v.dz + a.az * TIMESTEP,
  }
}

/// Computes the next `BodyStates`.
pub fn compute_forces(bs: BodyStates) -> BodyStates {
  let accs = accelerations(&bs);
  BodyStates {
    poss: bs
      .poss
      .par_iter()
      .zip(bs.vels.par_iter())
      .map(|(p, v)| move_position(p, v))
      .collect(),
    vels: bs
      .vels
      .par_iter()
      .zip(accs.par_iter())
      .map(|(v, a)| update_velocity(v, a))
      .collect(),
    masses: bs.masses,
  }
}

/// Simple function to create a lot of bodies.
/// Thank you, Larkins, for letting me use these umbers.
pub fn init<'a>() -> BodyStates {
  let range: Vec<f64> = (0..N).map(|i| i as f64).collect();
  let ret = BodyStates {
    poss: range
      .iter()
      .map(|i| Position {
        x: 100. * (*i * 0.1),
        y: 200. * (*i * 0.1),
        z: 300. * (*i * 0.1),
      })
      .collect(),
    vels: range
      .iter()
      .map(|i| Velocity {
        dx: 400. + *i,
        dy: 500. + *i,
        dz: 600. + *i,
      })
      .collect(),
    masses: range.iter().map(|i| 10e6 * (*i + 100.2)).collect(),
  };
  ret
}
