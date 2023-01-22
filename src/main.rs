// Copyright 2023 Sung-Cheol Kim
//

use rand::{Rng, SeedableRng};

#[derive(Debug, Default)]
struct Walker {
    x: f64,
    y: f64,
    z: f64,
}

impl Walker {
    fn next<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let delta: f64 = rng.gen();
        self.x += delta;
        let delta: f64 = rng.gen();
        self.y += delta;
        let delta: f64 = rng.gen();
        self.z += delta;
    }
}

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    let mut w = Walker { x:0.0, y:0.0, z:0.0 };

    println!("Random Walker Simulator");

    for _i in 0..10 {
        w.next(&mut rng);
        println!("{:?}", w);
    }
}
