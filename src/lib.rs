
pub mod cars;
pub mod trucks;
pub mod template;
pub mod common_parts;

//This module should hold items you want to be exposed as u::<foo> in any file in this crate
mod crate_universal{
    #![allow(unused_imports)]
    pub use std::fmt::{Display, Formatter}; //Common utility items such as "Display" may be re-exported here (up to taste).
    pub use crate::cars::crate_universal::*;
    pub use crate::trucks::crate_universal::*;
    pub use crate::template::crate_universal::*;
    pub use crate::common_parts::*;
}

//This module isn't necessary, but it lets the standard_prefix work for top level modules as well. (see common_parts.rs)
mod mod_universal{
}

//This interface should re-export things which are meant to be used by other crates.
//As a default, this should just re-export the interface of each sub module; if a sub-module doesn't have anything for the crate interface, it just has an empty interface module.
pub mod interface{
    pub use crate::cars::interface::*;
    pub use crate::trucks::interface::*;
    pub use crate::common_parts::Paint;
}
pub use interface::*;

//These macros are optional, but its handy to reduce boiler plate, or if you want to tweak the formula for all modules at once.
pub mod macro_prefix{
    macro_rules! standard_prefix {
        () => {
            #[allow(unused_imports)]
            use super::mod_universal as m;
            #[allow(unused_imports)]
            use crate::crate_universal as u;
        };
    }
    //Tests are set up to have the same visibility as their parent module.
    #[cfg(test)]
    macro_rules! test_prefix {
        () => {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::super::mod_universal as m;
            #[allow(unused_imports)]
            use crate::crate_universal as u;
        };
    }
    pub(crate) use standard_prefix;
    #[cfg(test)]
    pub(crate) use test_prefix;
}
pub(crate) use macro_prefix::standard_prefix;
#[cfg(test)]
pub(crate) use macro_prefix::test_prefix;