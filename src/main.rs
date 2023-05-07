use std::fs::read_to_string;

use rand::{thread_rng, Rng};

use recipe::Recipe;
use trajectory::Trajectory;
use universe::{Particle, Universe};
use vec3::Vec3;

pub mod recipe;
pub mod time;
pub mod trajectory;
pub mod universe;
pub mod vec3;

fn main() {
    // Read our recipe file. This is the configuration of the system.
    let recipe = Recipe::from_string(read_to_string("recipe.bibber").unwrap()).unwrap();

    // Prepare some particles is a totally not hacky way.
    let boundary = recipe.boundary;
    let mut rng = thread_rng();
    let mut gen_in_range = |bound: f64| rng.gen_range(-0.5 * bound..0.5 * bound);
    let mut gen_particle = || {
        Particle::new(
            Vec3::new(
                gen_in_range(boundary.x),
                gen_in_range(boundary.y),
                gen_in_range(boundary.z),
            ),
            Vec3::new(
                gen_in_range(boundary.x * 100.0),
                gen_in_range(boundary.y * 100.0),
                gen_in_range(boundary.z * 100.0),
            ),
            Vec3::zero(),
            1e-24,
        )
    };
    let mut particles: Vec<Particle> = Vec::with_capacity(recipe.particles);
    let mut pruned = 0;
    for _ in 0..recipe.particles {
        'generator: loop {
            let candidate = gen_particle();
            for particle in &particles {
                let d = particle.pos - candidate.pos;
                if d.norm() < 7e-10 {
                    pruned += 1;
                    continue 'generator;
                }
            }

            particles.push(candidate);
            break;
        }
    }
    eprintln!("Pruned {pruned} particles to get {}.", recipe.particles);

    // Create the universe :)
    let mut u = Universe::new(recipe.timestep, recipe.boundary, recipe.temperature)
        .start(recipe.start)
        .add_particles(&particles);

    // Initiate trajectory to save the states in.
    let mut traj = Trajectory::from_universe(&u, recipe.title.to_owned());
    traj.add_frame_from_universe(&u);

    // Run this thing!
    let iters_per_snapshot = recipe.timesteps() / recipe.snapshots();
    let n_iters = recipe.timesteps();
    let walltime_start = std::time::Instant::now();
    while u.time < recipe.end {
        u.step();
        if u.iteration % iters_per_snapshot == 0 {
            let remaining_iters = n_iters - u.iteration;
            let delta_walltime = std::time::Instant::now() - walltime_start;
            let t_per_iter = delta_walltime.as_secs_f64() / u.iteration as f64;
            let walltime_remaining = remaining_iters as f64 * t_per_iter;
            eprint!(
                "iter {}/{}, t = {:.3} ps, estimated remaining walltime is {walltime_remaining:.0} s    \r",
                u.iteration,
                recipe.timesteps(),
                u.time.picoseconds()
            );
            traj.add_frame_from_universe(&u);
        }
    }
    let walltime_end = std::time::Instant::now();
    let walltime_runtime = walltime_end - walltime_start;

    // Report some stats about the simulation.
    eprintln!(
        "\nSimulated {} particles at {} K for {} ns with a timestep of {} fs in {} s.",
        u.particles.len(),
        u.temperature,
        recipe.time().nanoseconds(),
        recipe.timestep.femtoseconds(),
        walltime_runtime.as_secs()
    );
    eprintln!(
        "    {:.3} ps / s    {:.3} ns / day",
        recipe.time().picoseconds() / walltime_runtime.as_secs_f64(),
        recipe.time().nanoseconds() / (walltime_runtime.as_secs_f64() / 60.0 / 60.0 / 24.0)
    );
    let gro = traj.to_gro();
    println!("{gro}");
}
