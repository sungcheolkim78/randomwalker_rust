// Copyright 2023 Sung-Cheol Kim

use std::fs::File;
use std::io::{Write, Error};
use rand::Rng;
use std::fmt;


#[derive(Debug, Default, PartialEq)]
pub struct Walker {
    pub id: usize,
    pub t: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Walker {
    pub fn builder() -> WalkerBuilder {
        WalkerBuilder::default()
    }
    pub fn next<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.x = if rng.gen_bool(0.5) { self.x + 1.0f64 } else { self.x - 1.0f64 };
        self.y = if rng.gen_bool(0.5) { self.y + 1.0f64 } else { self.y - 1.0f64 };
        self.z = if rng.gen_bool(0.5) { self.z + 1.0f64 } else { self.z - 1.0f64 };
        self.t += 1.0f64;
    }
    pub fn new(&self) -> Result<(), Error> {
        let mut f = File::create("data/walker_records.csv")?;
        write!(f, "id,t,x,y,z\n")?;
        Ok(())
    }
    pub fn write(&self) -> Result<(), Error> {
        let mut f = File::options().append(true).open("data/walker_records.csv")?;
        write!(f, "{},{},{},{},{}\n", self.id, self.t, self.x, self.y, self.z)?;
        Ok(())
    }
}


impl fmt::Display for Walker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "W{} ({}, {}, {}) t={}", self.id, self.x, self.y, self.z, self.t)
    }
}

#[derive(Default)]
pub struct WalkerBuilder {
    id: usize,
    t: f64,
    x: f64,
    y: f64,
    z: f64,
}

impl WalkerBuilder {
    pub fn new(/* ... */) -> WalkerBuilder {
        WalkerBuilder { id: 0usize, t: 0f64, x:0f64, y:0f64, z:0f64 }
    }
    pub fn id(mut self, id: usize) -> WalkerBuilder {
        self.id = id;
        self
    }
    pub fn t(mut self, t: f64) -> WalkerBuilder {
        self.t = t;
        self
    }
    pub fn xyz(mut self, x: f64, y: f64, z: f64) -> WalkerBuilder {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }
    pub fn build(self) -> Walker {
        Walker { id: self.id, t: self.t, x: self.x, y: self.y, z: self.z }
    }
}


#[cfg(test)]
mod test {
    use super::Walker;
    use super::WalkerBuilder;
    use std::io::Error;

    #[test]
    fn test_walker_write() -> Result<(), Error> {
        let w = Walker { id: 0, x:0.0, y:0.0, z:0.0, t:0.0 };
        w.new()?;
        w.write()?;
        println!("{}", w);
        Ok(())
    }

    #[test]
    fn builder_test() {
        let walker = Walker { id: 3usize, t: 0f64, x: 0f64, y: 0f64, z: 0f64 };
        let walker0: Walker = WalkerBuilder::new().id(3usize).t(0f64).xyz(0.0, 0.0, 0.0).build();

        assert_eq!(walker, walker0);
    }
}
