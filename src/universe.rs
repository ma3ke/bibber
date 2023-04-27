use crate::time::Time;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Particle {
    /// Position in meters.
    pos: Vec3,
    /// Velocity in meters / second.
    vel: Vec3,
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
    iteration: usize,
    dt: Time,
    boundary: Vec3,
    /// Temperature in Kelvin.
    temperature: f64,
    particles: Vec<Particle>,
}

impl Universe {
    pub fn new(dt: Time) -> Self {
        Self {
            time: Time::zero(),
            iteration: 0,
            dt,
            boundary: Vec3::new(1e-8, 1e-8, 1e-8),
            temperature: 200.0,
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
    // ε is the depth of the potential well.
    let epsilon = 1.0;
    // σ is the distance at which the potential crosses zero.
    let sigma = 1.0;

    // sigma is first turned into a vector to enable division by another vector.
    let sigma_over_r = Vec3::one() * sigma / r;

    (sigma_over_r.powi(12) - sigma_over_r.powi(6)) * 4.0 * epsilon
}

impl Universe {
    /// Apply one time step.
    pub fn step(&mut self) {
        // Predictor stage.
        for particle in &mut self.particles {
            // Move the particles. pos = pos + vel * Δt + 1/2 * acc * Δt^2
            particle.pos += particle.vel * self.dt + particle.acc * self.dt * self.dt * 0.5;
            // Update velocities. vel = vel + acc * Δt
            particle.vel += particle.acc * self.dt;
        }

        // Get forces and adjust accelerations.
        let other_positions: Vec<_> = self.particles.iter().map(|p| p.pos).collect();
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
                let r = particle.pos - *other_pos;
                force += -lennard_jones(r);
            }

            // Update acceleration. a = F / m
            particle.acc = force / particle.mass;
            println!("force: {force:?}, acc: {:?}", particle.acc);
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
            if pos.x < -bound.x / 2.0 || pos.x > bound.x / 2.0 {
                pos.x = pos.x % bound.x
            }
            if pos.y < -bound.y / 2.0 || pos.y > bound.y / 2.0 {
                pos.y = pos.y % bound.y
            }
            if pos.z < -bound.z / 2.0 || pos.z > bound.z / 2.0 {
                pos.z = pos.z % bound.z
            }
        }

        const BOLTZMANN: f64 = 1.380649e-23f64; // J⋅K−1
        const INV_BOLTZMANN: f64 = 1.0 / BOLTZMANN; // K⋅J−1

        // Apply temperature control.
        let temperature = self.particles.iter().fold(0.0, |accum, p| {
            // E_kin = 1/2 * m * v^2
            let kinetic_energy = 0.5 * p.mass * p.vel.powi(2).norm();
            // T = 2/3 * 1/k_B * E_kin
            let temp = (2.0 / 3.0) * INV_BOLTZMANN * kinetic_energy;
            accum + temp
        });
        println!(
            "temperature at {:.06} ns is {temperature} K",
            self.time.nanoseconds()
        );

        // // Calculate the difference in temperature per particle.
        // let d_temperature = (self.temperature - temperature) / self.particles.len() as f64;
        // // This difference we spread out over all of the particles by subtracting it. This will
        // // push the system back into the set temperature.
        // for particle in &mut self.particles {
        //     let vel = particle.vel;
        //     // Adjust the velocity such that it satisfies |v_new| = |v| - d_temperature
        //     particle.vel -= vel * (1.0 - d_temperature / vel.norm())
        // }

        // Apply pressure control.
        // TODO: Implement pressure control.

        // Increase time and iteration count.
        self.time += self.dt;
        self.iteration += 1;
    }

    /// Apply `n` time steps in succession.
    pub fn steps(&mut self, n: usize) {
        for _ in 0..n {
            self.step()
        }
    }
}
