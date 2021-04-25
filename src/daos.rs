// file not found for module `modules`
// help: to create the module `modules`, create file "src\daos\modules.rs" rustc(E0583)
mod modules;

// failed to resolve: maybe a missing crate `modules`?
// maybe a missing crate `modules`? rustc(E0433)
use crate::modules::users::dao::{hello};

pub fn example() {
    // cannot find function `hello` in this scope
    hello();
}
