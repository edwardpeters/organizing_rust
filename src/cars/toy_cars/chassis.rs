crate::standard_prefix!();

pub struct Chassis;
impl Display for Chassis{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "a Chassis, but smaller and more toy-like")
    }
}