#![feature(core_intrinsics)]

//use std::fs::File;

use std::io::Write;

use itertools::{izip, zip};

use std::time::Instant;

type Datatype = f32;

const N: usize = 65536;
const D: usize = 100;
const DIST: Datatype = 100000.;
const G: Datatype = 6.674e-11;
const MASS: Datatype = 5.97e20;
const SOFT: Datatype = 1e-20;
const DT: Datatype = 1.;

#[derive(Clone, Copy)]
struct Pos {
    x: Datatype,
    y: Datatype,
    z: Datatype,
}
struct Force {
    x: Datatype,
    y: Datatype,
    z: Datatype,
}

#[derive(Clone, Copy)]
struct Body {
    xvi: Datatype,
    yvi: Datatype,
    zvi: Datatype,
    dpx: Datatype,
    dpy: Datatype,
    dpz: Datatype,
}

struct Galaxy {
    poses: Box<[Pos; N]>,
    bodies: Box<[Body; N]>,
    masses: Box<[Datatype; N]>,
}

impl Pos {
    pub fn new(x: Datatype, y: Datatype, z: Datatype) -> Self {
        Self { x: x, y: y, z: z }
    }

    fn add_body(&mut self, body: &Body) {
        self.x += body.dpx;
        self.y += body.dpy;
        self.z += body.dpz;
    }
}

impl Body {
    pub fn new() -> Self {
        Self {
            xvi: 0.,
            yvi: 0.,
            zvi: 0.,
            dpx: 0.,
            dpy: 0.,
            dpz: 0.,
        }
    }

    pub fn add_force(&self, force: &Force) -> (Datatype, Datatype, Datatype) {
        (self.xvi + force.x, self.yvi + force.y, self.zvi + force.z)
    }
}

impl Force {
    pub fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn new_with(x: Datatype, y: Datatype, z: Datatype) -> Self {
        Self { x: x, y: y, z: z }
    }
}

/*fn save_matrix(galaxy: &Galaxy) {
    let mut f = File::create("matrix_RUST.mat").expect("Unable to create file");
    galaxy.bodies.iter().for_each(|e| {
        write!(&mut f, "{:.8}\n", e.dpx).unwrap();
    });
}*/

#[inline]
fn run(galaxy: &mut Galaxy) {
    let bodies = &mut (galaxy.bodies);
    let poses = &mut (galaxy.poses);
    let masses = &(galaxy.masses);
    (0..D).for_each(|_| {
        let forces = poses.iter().zip(&masses[..]).map(|(pj, massj)| {
            let force =
                zip(&poses[..], &masses[..]).fold(Force::new(), |force_acc, (pi, massi)| {
                    let dx = pi.x - pj.x;
                    let dy = pi.y - pj.y;
                    let dz = pi.z - pj.z;
                    let dsquared = (dx * dx) + (dy * dy) + (dz * dz) + SOFT;
                    let d32 = 1. / dsquared.sqrt().powi(3);
                    let f = G * *massj * *massi;
                    Force::new_with(
                        force_acc.x + f * dx * d32,
                        force_acc.y + f * dy * d32,
                        force_acc.z + f * dz * d32,
                    )
                });

            Force::new_with(
                (force.x / *massj) * 0.5 * DT,
                (force.y / *massj) * 0.5 * DT,
                (force.z / *massj) * 0.5 * DT,
            )
        });

        bodies.iter_mut().zip(forces).for_each(|(body, force)| {
            let (dvx, dvy, dvz) = body.add_force(&force);

            body.xvi = dvx;
            body.yvi = dvy;
            body.zvi = dvz;
            body.dpx = dvx * DT;
            body.dpy = dvy * DT;
            body.dpz = dvz * DT;
        });

        poses
            .iter_mut()
            .zip(&bodies[..])
            .for_each(|(pos, body)| pos.add_body(body));
    });
}

fn alloc_data<T>() -> Box<[T; N]> {
    let matrix = unsafe {
        let layout = std::alloc::Layout::new::<[T; N]>();
        let ptr = std::alloc::alloc_zeroed(layout) as *mut [T; N];
        Box::from_raw(ptr)
    };

    matrix
}

fn load(galaxy: &mut Galaxy) {
    let n = N as Datatype;
    let mut sqrt_n = n.sqrt();
    if (sqrt_n * sqrt_n) != n {
        sqrt_n += 1.0;
    }

    izip!(
        &mut (galaxy.bodies[..]),
        &mut (galaxy.poses[..]),
        &mut (galaxy.masses[..])
    )
    .enumerate()
    .for_each(|(i, (body, pos, mass))| {
        *pos = Pos::new(
            ((i as Datatype) % sqrt_n) * DIST,
            DIST * (i as Datatype),
            5000.0,
        );

        *body = Body::new();

        *mass = MASS;
    });
}

fn main() {
    let mut galaxy = Galaxy {
        bodies: alloc_data(),
        poses: alloc_data(),
        masses: alloc_data(),
    };

    load(&mut galaxy);

    let now = Instant::now();
    run(&mut galaxy);

    let time = now.elapsed().as_millis();
    println!("{}", time);

    //save_matrix(&galaxy);

    let name: String = std::env::current_exe()
        .ok()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        .into();
    println!("{} {} {}", name, N, time);

    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open("times.csv")
        .unwrap();
    write!(&mut f, "{}, {}, {}\n", name, N, time).unwrap();
}
