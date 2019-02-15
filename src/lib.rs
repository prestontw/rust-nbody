use rayon::prelude::*;

const N: usize = 10000;
const G: f64 = 6.67e-11;
const TIMESTEP: f64 = 0.25;
pub const NSTEPS: usize = 10;

pub struct BodyStates {
  xs: Vec<f64>,
  ys: Vec<f64>,
  zs: Vec<f64>,
  dxs: Vec<f64>,
  dys: Vec<f64>,
  dzs: Vec<f64>,
  masses: Vec<f64>,
  /* x: f64,
  x: f64,
  x: f64,*/
}

fn dist(dx: f64, dy: f64, dz: f64) -> f64 {
  ((dx * dx) + (dy * dy) + (dz * dz)).sqrt()
}

fn force(mass1: f64, mass2: f64, distance: f64) -> f64 {
  (G * mass1 * mass2) / (distance * distance)
}

type Map = Vec<f64>;

fn forces(bs: &BodyStates) -> (Map, Map, Map) {
  let dxs = bs.xs.
}

fn accelerations(bs: &BodyStates) -> (Map, Map, Map) 
{
  let (fx, fy, fz) = ...;
  (bs.masses.iter().zip(fx).map(
    |(&m, &fx)| fx / m
  ).collect(),
  bs.masses.iter().zip(fy).map(
    |(&m, &fx)| fx / m
  ).collect(),
  bs.masses.iter().zip(fz).map(
    |(&m, &fx)| fx / m
).collect(),
)
i.into_par_iter()
    .map(|b| {
      let mut fx: f64 = 0.0;
      let mut fy: f64 = 0.0;
      let mut fz: f64 = 0.0;

// calculate forces over all other things
      for other in reference {
        let dx = b.x - other.x;
        let dy = b.y - other.y;
        let dz = b.z - other.z;

        let d = dist(dx, dy, dz);
        let f = force(b.mass, other.mass, d);

        fx += (f * dx) / d;
        fy += (f * dy) / d;
        fz += (f * dz) / d;
      }
    }
}

pub fn compute_forces(bs: BodyStates) -> BodyStates
{
  let (axs, ays, azs) = accelerations(&bs);
    BodyStates {
    xs: bs.xs.iter().zip(bs.dxs.iter()).map(
      |(&x, &dx)| x + dx * TIMESTEP).collect(),
    ys: bs.ys.iter().zip(bs.dys.iter()).map(
      |(&x, &dx)| x + dx * TIMESTEP).collect(),
    zs: bs.zs.iter().zip(bs.dzs.iter()).map(
      |(&x, &dx)| x + dx * TIMESTEP).collect(),
    dxs: bs.dxs.iter().zip(axs).map(
      |(&dx, &ax)| dx + ax * TIMESTEP).collect(),
    dys: bs.dys.iter().zip(ays).map(
      |(&dx, &ax)| dx + ax * TIMESTEP).collect(),
    dzs: bs.dzs.iter().zip(azs).map(
      |(&dx, &ax)| dx + ax * TIMESTEP).collect(),
    masses: bs.masses,
  }
}

pub fn init<'a>() -> BodyStates {
  let range: Vec<f64> = (0..N).map(|i| i as f64).collect();
  let ret = BodyStates {
    xs: range.iter().map(|i| 100. * (*i * 0.1)).collect(),
    ys: range.iter().map(|i| 200. * (*i * 0.1)).collect(),
    zs: range.iter().map(|i| 300. * (*i * 0.1)).collect(),
    dxs: range.iter().map(|i| 400. + *i).collect(),
    dys: range.iter().map(|i| 500. + *i).collect(),
    dzs: range.iter().map(|i| 600. + *i).collect(),
    masses: range.iter().map(|i| 10e6 * (*i + 100.2)).collect(),
  };
  ret
}
