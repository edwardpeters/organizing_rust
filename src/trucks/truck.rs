crate::standard_prefix!();

pub struct Truck{
    engine: m::Engine,
    paint: u::Paint,
}

impl Truck{
    pub fn new(paint : u::Paint) -> Truck{
        Truck{
            engine : m::Engine,
            paint
        }
    }
}

impl u::Display for Truck{
    fn fmt(&self, f: &mut u::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Truck with a {} and {} paint.", self.engine, self.paint)
    }
}