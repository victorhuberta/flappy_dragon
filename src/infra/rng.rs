use bracket_lib::prelude::RandomNumberGenerator;

#[cfg_attr(test, faux::create)]
pub struct MyRNG {
    rng: RandomNumberGenerator,
}

#[cfg_attr(test, faux::methods)]
impl MyRNG {
    pub fn new() -> Self {
        Self {
            rng: RandomNumberGenerator::new(),
        }
    }

    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        self.rng.range(min, max)
    }
}
