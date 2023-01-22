// Copyright 2023 Sung-Cheol Kim
//

use rand::{Rng, SeedableRng};

#[derive(Debug, Default)]
struct Walker {
    x: f64,
    y: f64,
    z: f64,
    t: f64,
}

impl Walker {
    fn next<R: Rng + ?Sized>(&mut self, dt: f64, rng: &mut R) {
        self.x += rng.gen::<f64>();
        self.y += rng.gen::<f64>();
        self.z += rng.gen::<f64>();
        self.t += dt;
    }
}

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    let mut w = Walker { x:0.0, y:0.0, z:0.0, t:0.0 };

    println!("Random Walker Simulator");

    for _i in 0..10 {
        w.next(0.1f64, &mut rng);
        println!("{:?}", w);
    }
}
