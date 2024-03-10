crate::standard_prefix!();

pub struct ToyCar{
    engine: m::Engine, //mod_universal re-exports are passed down, so this is in scope
    paint: u::Paint, 
    chassis: m::Chassis,
    window: m::Window
}

impl ToyCar{
    pub fn new(paint : u::Paint) -> ToyCar{
        ToyCar{
            engine : m::Engine,
            paint,
            chassis : m::Chassis,
            window : m::Window
        }
    }
}

impl u::Display for ToyCar{
    fn fmt(&self, f: &mut u::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Toy Car with a {}, a {}, a {}, and a {:?}.", self.engine, self.paint, self.chassis, self.window)
    }
}