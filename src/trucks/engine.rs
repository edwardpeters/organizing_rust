crate::standard_prefix!();

//Because Engine was re-exported on a mod level rather than a crate level, this doesn't conflict with the Engine from cars.
#[derive(Debug)]
pub(crate) struct Engine;

impl u::Display for Engine{
    fn fmt(&self, f: &mut u::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A Truck Engine")
    }
}