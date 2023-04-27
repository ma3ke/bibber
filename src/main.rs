use rand::{thread_rng, Rng};

use time::Time;
use universe::{Particle, Universe};
use vec3::Vec3;

pub mod time;
pub mod universe;
pub mod vec3;

fn main() {
    let boundary = Vec3::new(1e-8, 1e-8, 1e-8);

    let mut rng = thread_rng();
    let mut gen_in_range = |bound: f64| rng.gen_range(-bound / 2.0..bound / 2.0);
    let mut gen_particle = || {
        Particle::new(
            Vec3::new(
                gen_in_range(boundary.x),
                gen_in_range(boundary.y),
                gen_in_range(boundary.z),
            ),
            Vec3::zero(),
            Vec3::zero(),
            1e-24,
        )
    };
    let particles = vec![gen_particle(), gen_particle(), gen_particle()];
    let mut u = Universe::new(Time::from_femtoseconds(1.0))
        .boundary(boundary)
        .add_particles(&particles);

    while u.time < Time::from_nanoseconds(0.01) {
        println!("{u:?}");
        u.step()
    }
}
