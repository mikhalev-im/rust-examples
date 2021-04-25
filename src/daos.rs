// file not found for module `modules`
// help: to create the module `modules`, create file "src\daos\modules.rs" rustc(E0583)
#[path = "modules/mod.rs"] mod mod;
#[path = "modules/users/dao.rs"] mod dao;
#[path = "modules/users/mod.rs"] mod mod2;

// failed to resolve: maybe a missing crate `modules`?
// maybe a missing crate `modules`? rustc(E0433)

pub fn example() {
    // cannot find function `hello` in this scope
    dao::hello();
}
