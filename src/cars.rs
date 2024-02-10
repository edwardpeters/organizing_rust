pub mod chassis;
pub mod engine;
pub mod car;

//This module re-exports things from child modules which should be included everywhere in the crate.
pub(crate) mod crate_universal{
    #![allow(unused_imports)]
    pub(crate) use super::car::Car;
}

//This module re-exports things which should be used in every child module.
mod mod_universal{
    #![allow(unused_imports)]
    pub(super) use crate::crate_universal::*;
    pub(super) use super::engine::Engine;
    pub(super) use super::chassis::*;
    pub(super) use super::engine::*;
}

//This module re-exports things from child modules which should be on the public intrerface of the crate.
pub mod interface{
    pub use super::car::Car;
}