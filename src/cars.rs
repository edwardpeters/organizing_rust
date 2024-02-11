pub mod chassis;
pub mod engine;
pub mod car;
pub mod toy_cars;

//This module re-exports things from child modules which should be included everywhere in the crate.
//Here we say that every module should see "Car"
pub(crate) mod crate_universal{
    #![allow(unused_imports)]
    pub(crate) use super::car::Car;
}

//This module re-exports things which should be used in every child module.
//Here we mostly want every module to have full visibility over each other, for now.
mod mod_universal{
    #![allow(unused_imports)]
    pub use super::super::mod_universal::*; //Double-super lets multi-level nesting work
    pub(super) use super::engine::Engine;
    pub(super) use super::chassis::*;
    pub(super) use super::engine::*;
}

//This module re-exports things from child modules which should be on the public intrerface of the crate.
//Here we say that Car should be on the public interface, plus anything from the toy_cars module.
pub mod interface{
    pub use super::car::Car;
    pub use super::toy_cars::interface::*;
}