pub mod toy_car;
pub mod chassis;
//This module re-exports things from child modules which should be included everywhere in the crate.
pub(crate) mod crate_universal{
    #![allow(unused_imports)]
}

//This module re-exports things which should be used in every child module.
mod mod_universal{
    #![allow(unused_imports)]
    pub(super) use super::super::mod_universal::{*, Chassis as _}; //Double-super lets multi-level nesting work
    pub(super) use super::chassis::Chassis;
}

//This module re-exports things from child modules which should be on the public intrerface of the crate.
pub mod interface{
    pub use super::toy_car::ToyCar;
}