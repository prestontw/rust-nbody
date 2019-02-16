use rayon::prelude::*;

const N: usize = 10000;
const G: f64 = 6.67e-11;
const TIMESTEP: f64 = 0.25;
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

pub struct BodyStates {
  poss: Vec<Position>,
  vels: Vec<Velocity>,
  masses: Vec<f64>,
}

fn dist(dx: f64, dy: f64, dz: f64) -> f64 {
  ((dx * dx) + (dy * dy) + (dz * dz)).sqrt()
}

fn force(mass1: f64, mass2: f64, distance: f64) -> f64 {
  (G * mass1 * mass2) / (distance * distance)
}

fn accelerations(bs: &BodyStates) -> Vec<Acceleration> {
  bs.poss
    .par_iter()
    .zip(bs.masses.par_iter())
    .map(|(ref p, &m)| {
      let reference = bs.poss.par_iter().zip(bs.masses.par_iter());
      // calculate forces over all other things
      let forces = reference.map(|(ref otherpos, &othermass)| {
        let dx = p.x - otherpos.x;
        let dy = p.y - otherpos.y;
        let dz = p.z - otherpos.z;

        let d = dist(dx, dy, dz);
        let f = force(m, othermass, d);

        Force {
          fx: (f * dx) / d,
          fy: (f * dy) / d,
          fz: (f * dz) / d,
        }
      });
      let Force { fx, fy, fz } = forces.reduce(
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
      );
      Acceleration {
        ax: fx / m,
        ay: fy / m,
        az: fz / m,
      }
    })
    .collect()
}

// maybe make this update the position?
fn move_position(p: &mut Position, v: &Velocity) {
  p.x += v.dx * TIMESTEP;
  p.y += v.dy * TIMESTEP;
  p.z += v.dz * TIMESTEP;
}

// maybe make this statefully update vel?
fn update_velocity(v: &mut Velocity, a: &Acceleration) {
  v.dx += a.ax * TIMESTEP;
  v.dy += a.ay * TIMESTEP;
  v.dz += a.az * TIMESTEP;
}

// slight slowdown from making not parallel
pub fn compute_forces(mut bs: BodyStates) -> BodyStates {
  let accs = accelerations(&bs);
  bs.poss
    .par_iter_mut()
    .zip(bs.vels.par_iter())
    .for_each(|(p, v)| move_position(p, v));
  bs.vels
    .par_iter_mut()
    .zip(accs.par_iter())
    .for_each(|(v, a)| update_velocity(v, a));

  bs
}

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
