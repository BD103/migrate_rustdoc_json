#![feature(optimize_attribute)]

#[optimize(none)]
pub fn do_not_optimize() {}

#[optimize(speed)]
pub fn optimize_speed() {}

#[optimize(size)]
pub fn optimize_size() {}
