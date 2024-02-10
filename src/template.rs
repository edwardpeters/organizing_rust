//This is an empty module meeting the standard format, to copy as a starting place.

//This module re-exports things from child modules which should be included everywhere in the crate.
pub(crate) mod crate_universal{
    #![allow(unused_imports)]
}

//This module re-exports things which should be used in every child module.
mod mod_universal{
    #![allow(unused_imports)]
    pub use super::super::mod_universal::*; //Double-super lets multi-level nesting work
}

//This module re-exports things from child modules which should be on the public intrerface of the crate.
pub mod interface{
}