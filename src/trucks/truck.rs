crate::standard_prefix!();

pub struct Truck{
    engine: Engine,
    paint: Paint,
}

impl Truck{
    pub fn new(paint : Paint) -> Truck{
        Truck{
            engine : Engine,
            paint
        }
    }
}

impl Display for Truck{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Truck with a {} and {} paint.", self.engine, self.paint)
    }
}