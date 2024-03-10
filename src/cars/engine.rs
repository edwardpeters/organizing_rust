crate::standard_prefix!();

//I have a procedural macro for eliminating this boiler plate as well, if anyone wants it.
#[cfg(test)]
#[path = "tests/engine_tests.rs"]
mod engine_tests;

//Because Engine was re-exported on a mod level rather than a crate level, this doesn't conflict with the Engine from trucks.
#[derive(Debug)]
pub(crate) struct Engine;

impl u::Display for Engine{
    fn fmt(&self, f: &mut u::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A Car Engine")
    }
}