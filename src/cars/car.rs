crate::standard_prefix!();

//I have a procedural macro for eliminating this boiler plate as well, if anyone wants it.
#[cfg(test)]
#[path = "tests/car_tests.rs"]
mod car_tests;

//The car components were re-exported by mod_universal, so they are available here.
pub struct Car{
    chassis : m::Chassis,
    engine : m::Engine,
    paint : u::Paint,
    windows : Vec<m::Window>
}

impl Car{
    pub fn new(paint : u::Paint) -> Car{
        Car{
            chassis : m::Chassis,
            engine : m::Engine,
            paint,
            windows : Vec::new()
        }
    }
}

//Common traits such as "Display" are re-exported by crate_universal
impl u::Display for Car{
    fn fmt(&self, f: &mut u::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Car with {} windows, a {:?}, a {}, and {} paint.", self.windows.len(), self.chassis, self.engine, self.paint)
    }
}