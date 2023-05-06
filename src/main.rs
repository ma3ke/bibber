#![feature(const_for)]

use rand::{thread_rng, Rng};

use time::Time;
use trajectory::Trajectory;
use universe::{Particle, Universe};
use vec3::Vec3;

pub mod time;
pub mod trajectory;
pub mod universe;
pub mod vec3;

fn main() {
    let boundary = Vec3::new(1e-7, 1e-7, 1e-7);

    let mut rng = thread_rng();
    let mut gen_in_range = |bound: f64| rng.gen_range(-bound / 2.0..bound / 2.0);
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
    let mut particles = Vec::new();
    let n_start = 100;
    for _ in 0..n_start {
        particles.push(gen_particle())
    }
    let mut particles_pruned = Vec::new();
    'outer: for (ip, p) in particles.iter().enumerate() {
        'inner: for (iq, q) in particles.iter().enumerate() {
            if ip == iq {
                continue 'inner;
            }
            let d = p.pos - q.pos;
            if d.norm() < 7e-9 {
                continue 'outer;
            }
        }

        particles_pruned.push(*p)
    }
    eprintln!("{}/{n_start} survived", particles_pruned.len());
    let mut u = Universe::new(Time::from_femtoseconds(10.0))
        .boundary(boundary)
        .add_particles(&particles_pruned);

    let mut traj = Trajectory::from_universe(&u, "My universe".to_string());
    traj.add_frame_from_universe(&u);
    let until_time = Time::from_nanoseconds(0.10);
    let n_iters = (until_time.seconds() / u.dt.seconds()) as usize;
    let walltime_start = std::time::Instant::now();
    while u.time < until_time {
        ////eprintln!("==> {u:?}");
        if !u.step() {
            eprintln!("[t={} ns] Okay bye...", u.time.nanoseconds());
            break;
        }
        if u.iteration % 100 == 0 {
            let remaining_iters = n_iters - u.iteration;
            let delta_walltime = std::time::Instant::now() - walltime_start;
            let t_per_iter = delta_walltime.as_secs_f64() / u.iteration as f64;
            let walltime_remaining = remaining_iters as f64 * t_per_iter;
            eprint!(
                "t = {:>10.3} ps    est. rem. wall time {walltime_remaining:.0} s\r",
                u.time.picoseconds()
            );
            traj.add_frame_from_universe(&u);
        }
    }
    let gro = traj.to_gro();
    println!("{gro}");
}
