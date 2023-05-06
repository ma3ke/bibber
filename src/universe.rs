use crate::time::Time;
use crate::vec3::Vec3;

const BOLTZMANN: f64 = 1.380649e-23; // J⋅K−1
const INV_BOLTZMANN: f64 = 1.0 / BOLTZMANN; // K⋅J−1
const V_LIGHT: f64 = 299_792_458.0; // m / s
#[rustfmt::skip]
const NEIGHBOURS: [(isize, isize, isize); 9 * 3] = [
    (-1, -1, -1), (-1, -1,  0), (-1, -1,  1), 
    (-1,  0, -1), (-1,  0,  0), (-1,  0,  1), 
    (-1,  1, -1), (-1,  1,  0), (-1,  1,  1), 
    ( 0, -1, -1), ( 0, -1,  0), ( 0, -1,  1), 
    ( 0,  0, -1), ( 0,  0,  0), ( 0,  0,  1), 
    ( 0,  1, -1), ( 0,  1,  0), ( 0,  1,  1), 
    ( 1, -1, -1), ( 1, -1,  0), ( 1, -1,  1), 
    ( 1,  0, -1), ( 1,  0,  0), ( 1,  0,  1), 
    ( 1,  1, -1), ( 1,  1,  0), ( 1,  1,  1), 
];

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Particle {
    /// Position in meters.
    pub(crate) pos: Vec3,
    /// Velocity in meters / second.
    pub(crate) vel: Vec3,
    /// Acceleration in meters / second^2.
    acc: Vec3,
    /// Mass in kg.
    mass: f64,
}

impl Particle {
    pub const fn new(pos: Vec3, vel: Vec3, acc: Vec3, mass: f64) -> Self {
        Self {
            pos,
            vel,
            acc,
            mass,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Universe {
    pub time: Time,
    pub(crate) iteration: usize,
    pub(crate) dt: Time,
    pub(crate) boundary: Vec3,
    /// Temperature in Kelvin.
    pub(crate) temperature: f64,
    pub(crate) particles: Vec<Particle>,
}

impl Universe {
    pub fn new(dt: Time) -> Self {
        Self {
            time: Time::zero(),
            iteration: 0,
            dt,
            boundary: Vec3::new(1e-8, 1e-8, 1e-8),
            temperature: 1000.0,
            particles: Vec::new(),
        }
    }

    pub fn boundary(mut self, boundary: Vec3) -> Self {
        self.boundary = boundary;
        self
    }

    pub fn add_particle(mut self, particle: Particle) -> Self {
        self.particles.push(particle);
        self
    }

    pub fn add_particles(mut self, particles: &[Particle]) -> Self {
        self.particles.extend_from_slice(particles);
        self
    }
}

/// Interatomic potential according to
/// [Lennard-Jones potential](https://en.wikipedia.org/wiki/Lennard-Jones_potential).
///
/// ```
/// V_LJ(r) = 4 * ε * [ ( σ / r ) ^ 12 − ( σ / r ) ^ 6 ]
/// ```
pub fn lennard_jones(r: Vec3) -> Vec3 {
    // ε is the depth of the potential well. (J/mol)
    let epsilon = 1.8e3;
    // σ is the distance at which the potential crosses zero. (meter)
    let sigma = 4.0e-10;

    let sigma_over_r = sigma / r.norm();

    // FIXME: x^12 = (x^6)^2 optimization
    r * ((sigma_over_r.powi(12) - sigma_over_r.powi(6)) * 4.0 * epsilon)
}

impl Universe {
    /// Apply one time step.
    pub fn step(&mut self) -> bool {
        let mut okay = true;
        // Predictor stage.
        for particle in &mut self.particles {
            // Move the particles. pos = pos + vel * Δt + 1/2 * acc * Δt^2
            particle.pos += particle.vel * self.dt + particle.acc * self.dt * self.dt * 0.5;
            // Update velocities. vel = vel + acc * Δt
            particle.vel += particle.acc * self.dt;
        }

        // Get forces and adjust accelerations.
        let other_positions: Vec<_> = self.particles.iter().map(|p| p.pos).collect();
        for (x, y, z) in NEIGHBOURS {
            for (index, particle) in self.particles.iter_mut().enumerate() {
                // Get forces.
                // F = - ∇V(pos)
                //
                // We can obtain this force by simply negating the Lennard-Jones potential. With the
                // small timestep (dt) we integrate this so we can treat it as a force in our model.
                let mut force = Vec3::zero();
                for (other_index, other_pos) in other_positions.iter().enumerate() {
                    if index == other_index {
                        continue;
                    }
                    let other_pos_adjusted = Vec3::new(x as f64, y as f64, z as f64) * *other_pos;
                    let r = particle.pos - other_pos_adjusted;
                    if r.norm() < 2.5e-8 {
                        force -= lennard_jones(r);
                    }
                }

                // Update acceleration. a = F / m
                particle.acc = force / particle.mass;
            }
        }

        // // Corrector stage.
        // for particle in &mut self.particles {
        //     // Adjust predicted particle positions and velocities based on new acceleration.
        //     particle.pos += adjust(particle.acc, self.dt);
        //     particle.vel += adjust(particle.acc, self.dt);
        // }

        // Apply boundary conditions.
        for particle in &mut self.particles {
            let pos = &mut particle.pos;
            let bound = self.boundary;

            // TODO: If this even works, might as well do the remainder tactic anyways. The check
            // is more expensive than the actual calculation, probably. Is also branchles, and
            // therefore much less chance of a misprediction.
            if pos.x < -bound.x || pos.x > bound.x {
                pos.x = pos.x % bound.x
            }
            if pos.y < -bound.y || pos.y > bound.y {
                pos.y = pos.y % bound.y
            }
            if pos.z < -bound.z || pos.z > bound.z {
                pos.z = pos.z % bound.z
            }
        }

        // let total_kinetic_energy: f64 = self
        //     .particles
        //     .iter()
        //     .map(|p| {
        //         // E_kin = 1/2 * m * v^2
        //         0.5 * p.mass * p.vel.norm().powi(2)
        //     })
        //     .sum();
        // // T = 2/3 * 1/k_B * E_kin
        // let temperature = (2.0 / 3.0) * INV_BOLTZMANN * total_kinetic_energy;
        // eprintln!(
        //     "temperature at {:06} ns is {temperature} K",
        //     self.time.nanoseconds()
        // );

        // Apply temperature control.
        let n_particles = self.particles.len();
        let t_per_particle = self.temperature / n_particles as f64;
        for particle in &mut self.particles {
            // E_kin = T / (2/3 * 1/k_B)
            //       = 3/2 * k_B * T
            //
            // E_kin = 1/2 * m * v^2
            //   v^2 = E_kin / (1/2 * m)
            //       = 2 * E_kin / m
            //
            // |v| = sqrt(2 * E_kin / m)
            //     = sqrt(2 * 3/2 * k_B * T / m)
            //     = sqrt(3 * k_B * T / m)
            // FIXME: Optimization possible here.
            let new_norm = f64::sqrt(3.0 * BOLTZMANN * t_per_particle / particle.mass);
            let scaling_factor = new_norm / particle.vel.norm();
            particle.vel = particle.vel * scaling_factor;
        }

        // Apply pressure control.
        // TODO: Implement pressure control.

        // Increase time and iteration count.
        self.time += self.dt;
        self.iteration += 1;

        okay
    }

    /// Apply `n` time steps in succession.
    pub fn steps(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }
}