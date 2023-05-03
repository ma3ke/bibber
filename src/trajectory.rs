use crate::{
    time::Time,
    universe::{Particle, Universe},
    vec3::Vec3,
};

pub struct Frame {
    time: Time,
    particles: Vec<Particle>,
}

pub struct Trajectory {
    title: String,
    n_particles: usize,
    frames: Vec<Frame>,
    bounding_box: Vec3,
}

impl Trajectory {
    pub fn from_universe(u: &Universe, title: String) -> Self {
        Self {
            title,
            n_particles: u.particles.len(),
            frames: Vec::new(),
            bounding_box: u.boundary,
        }
    }

    pub fn add_frame_from_universe(&mut self, u: &Universe) {
        self.frames.push(Frame {
            time: u.time,
            particles: u.particles.clone(),
        })
    }

    pub fn to_gro(&self) -> String {
        let mut s = String::new();
        for frame in &self.frames {
            s.push_str(&format!(
                "{}, t= {}\n{}\n",
                self.title,
                frame.time.picoseconds(),
                self.n_particles,
            ));
            for (index, particle) in frame.particles.iter().enumerate() {
                let Vec3 { x, y, z } = particle.pos * 1e9; // in nm
                let Vec3 {
                    x: v_x,
                    y: v_y,
                    z: v_z,
                } = particle.vel * 1e-3; // in km/s
                s.push_str(&format!(
                    "{index:>5}DUMMY  DUM{index:>5}{:8.3}{:8.3}{:8.3}{:8.4}{:8.4}{:8.4}\n",
                    x, y, z, v_x, v_y, v_z
                ));
            }
            let Vec3 {
                x: bb_x,
                y: bb_y,
                z: bb_z,
            } = self.bounding_box;
            s.push_str(&format!("{bb_x} {bb_y} {bb_z}\n"))
        }

        s
    }
}
