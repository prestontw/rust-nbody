use rayon::prelude::*;

const N: usize = 10000;
const G: f64 = 6.67e-11;
const TIMESTEP: f64 = 0.25;
pub const NSTEPS: usize = 10;

pub struct Body {
  x: f64,
  y: f64,
  z: f64,
  dx: f64,
  dy: f64,
  dz: f64,
  mass: f64,
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

pub fn compute_forces<'a, I>(i: I, reference: &Vec<Body>) -> Vec<Body>
where
  I: rayon::iter::IntoParallelIterator<Item = &'a Body>,
{
  i.into_par_iter()
    .map(|b| {
      let mut fx: f64 = 0.0;
      let mut fy: f64 = 0.0;
      let mut fz: f64 = 0.0;

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
      let ax = fx / b.mass;
      let ay = fy / b.mass;
      let az = fz / b.mass;

      Body {
        x: b.x + (TIMESTEP * b.dx),
        y: b.y + (TIMESTEP * b.dy),
        z: b.z + (TIMESTEP * b.dz),
        dx: b.dx + (TIMESTEP * ax),
        dy: b.dy + (TIMESTEP * ay),
        dz: b.dz + (TIMESTEP * az),
        mass: b.mass,
      }
    })
    .collect()
}

pub fn init() -> Vec<Body> {
  (0..N)
    .map(|i| {
      let i = i as f64;
      Body {
        x: 100. * (i + 0.1),
        y: 200. * (i + 0.1),
        z: 300. * (i + 0.1),
        dx: i + 400.0,
        dy: i + 500.,
        dz: i + 600.,
        mass: 10e6 * (i + 100.2),
      }
    })
    .collect()
}
