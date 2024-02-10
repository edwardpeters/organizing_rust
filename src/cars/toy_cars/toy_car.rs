crate::standard_prefix!();

pub struct ToyCar{
    engine: Engine, //mod_universal re-exports are passed down, so this is in scope
    paint: Paint, 
    chassis: Chassis,
    window: Window
}

impl ToyCar{
    pub fn new(paint : Paint) -> ToyCar{
        ToyCar{
            engine : Engine,
            paint,
            chassis : Chassis,
            window : Window
        }
    }
}

impl Display for ToyCar{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Toy Car with a {}, a {}, a {}, and a {:?}.", self.engine, self.paint, self.chassis, self.window)
    }
}