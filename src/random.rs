extern crate libc;

use geom::{Point, Size, Coordinate};

pub trait Random {
  fn new_point(&mut self, max: Size) -> Point;
}

pub struct OSRandom {}

impl Random for OSRandom {
  fn new_point(&mut self, max: Size) -> Point {
    let x = (unsafe { libc::rand() } % max.width as libc::c_int) as Coordinate;
    let y = (unsafe { libc::rand() } % max.height as libc::c_int) as Coordinate;
    Point::new(x, y)
  }
}