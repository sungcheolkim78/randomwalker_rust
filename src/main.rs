// Copyright 2023 Sung-Cheol Kim

use rand::SeedableRng;
use std::io::Error;

use randomwalker_rust::Walker;
use randomwalker_rust::WalkerBuilder;

fn main() -> Result<(), Error> {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(100);

    println!("Random Walker Simulator");

    for _i in 0..10 {
        let mut w = WalkerBuilder::new().id(0).build();
        w.new()?;
        for _j in 0..100 {
            w.next(&mut rng);
            w.write()?;
            println!("{}", w);
        }
    }

    Ok(())
}
