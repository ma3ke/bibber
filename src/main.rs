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
                gen_in_range(boundary.x * 10.0),
                gen_in_range(boundary.y * 10.0),
                gen_in_range(boundary.z * 10.0),
            ),
            Vec3::zero(),
            1e-24,
        )
    };
    let mut particles = Vec::new();
    for _ in 0..100 {
        particles.push(gen_particle())
    }
    let mut u = Universe::new(Time::from_femtoseconds(1.0))
        .boundary(boundary)
        .add_particles(&particles);

    let mut traj = Trajectory::from_universe(&u, "My universe".to_string());
    traj.add_frame_from_universe(&u);
    let mut good_one = u.clone();
    let mut good_one_n = u.clone();
    while u.time < Time::from_nanoseconds(1000.0) {
        ////eprintln!("==> {u:?}");
        if !u.step() {
            eprintln!("[t={} ns] Okay bye...", u.time.nanoseconds());
            break;
        }
        traj.add_frame_from_universe(&u);
        good_one = good_one_n.clone();
        good_one_n = u.clone();
    }
    let mut closest = f64::INFINITY;
    for (i_p, p) in good_one.particles.iter().enumerate() {
        for (i_o, o) in good_one.particles.iter().enumerate() {
            if i_p == i_o {
                continue;
            }
            let d = (p.vel - o.vel).norm();
            if d < closest {
                eprintln!("new closest: {} nm", closest * 1e10);
                closest = d
            }
        }
    }
    eprintln!("Closest: {} nm", closest * 1e10);
    let gro = traj.to_gro();
    println!("{gro}");
}
