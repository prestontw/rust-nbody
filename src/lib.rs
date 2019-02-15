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
    .iter()
    .zip(bs.masses.iter())
    .map(|(ref p, &m)| {
      let mut fx: f64 = 0.0;
      let mut fy: f64 = 0.0;
      let mut fz: f64 = 0.0;

      let reference = bs.poss.iter().zip(bs.masses.iter());
      // calculate forces over all other things
      for (ref otherpos, &othermass) in reference {
        let dx = p.x - otherpos.x;
        let dy = p.y - otherpos.y;
        let dz = p.z - otherpos.z;

        let d = dist(dx, dy, dz);
        let f = force(m, othermass, d);

        fx += (f * dx) / d;
        fy += (f * dy) / d;
        fz += (f * dz) / d;
      }
      Acceleration {
        ax: fx / m,
        ay: fy / m,
        az: fz / m,
      }
    })
    .collect()
}

// maybe make this update the position?
fn move_position(p: &Position, v: &Velocity) -> Position {
  Position {
    x: p.x + v.dx * TIMESTEP,
    y: p.y + v.dy * TIMESTEP,
    z: p.z + v.dz * TIMESTEP,
  }
}

// maybe make this statefully update vel?
fn update_velocity(v: &Velocity, a: &Acceleration) -> Velocity {
  Velocity {
    dx: v.dx + a.ax * TIMESTEP,
    dy: v.dy + a.ay * TIMESTEP,
    dz: v.dz + a.az * TIMESTEP,
  }
}

pub fn compute_forces(bs: BodyStates) -> BodyStates {
  let accs = accelerations(&bs);
  BodyStates {
    poss: bs
      .poss
      .iter()
      .zip(bs.vels.iter())
      .map(|(p, v)| move_position(p, v))
      .collect(),
    vels: bs
      .vels
      .iter()
      .zip(accs.iter())
      .map(|(v, a)| update_velocity(v, a))
      .collect(),
    masses: bs.masses,
  }
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
