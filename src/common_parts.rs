crate::standard_prefix!();

#[derive(Debug)]
pub enum Paint{
    Red,
    Green,
    SchoolBusYellow
}

impl Display for Paint{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Paint::Red => write!(f, "Red"),
            Paint::Green => write!(f, "Green"),
            Paint::SchoolBusYellow => write!(f, "School Bus Yellow")
        }
    }
}