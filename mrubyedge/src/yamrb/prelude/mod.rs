//! Prelude that wires up the built-in Ruby-like standard library for yamrb.
//! Each submodule exposes initializers that install core classes and constants
//! into a [`VM`] so user bytecode starts with the expected environment.

use super::vm::VM;

pub mod array;
pub mod class;
pub mod data;
pub mod enumerable;
pub mod exception;
pub mod falseclass;
pub mod float;
pub mod hash;
pub mod integer;
pub mod module;
pub mod nilclass;
pub mod object;
pub mod proc;
pub mod range;
pub mod shared_memory;
pub mod string;
pub mod symbol;
pub mod trueclass;

#[cfg(feature = "mruby-random")]
pub mod rand;

#[cfg(feature = "mruby-regexp")]
pub mod regexp;

#[cfg(feature = "mruby-uri")]
pub mod uri;

pub fn prelude(vm: &mut VM) {
    object::initialize_object(vm);
    exception::initialize_exception(vm);
    module::initialize_module(vm);
    class::initialize_class(vm);
    data::initialize_data(vm);
    integer::initialize_integer(vm);
    nilclass::initialize_nilclass(vm);
    trueclass::initialize_trueclass(vm);
    falseclass::initialize_falseclass(vm);
    symbol::initialize_symbol(vm);
    proc::initialize_proc(vm);
    string::initialize_string(vm);
    enumerable::initialize_enumerable(vm);
    array::initialize_array(vm);
    hash::initialize_hash(vm);
    range::initialize_range(vm);
    shared_memory::initialize_shared_memory(vm);
    float::initialize_float(vm);
    #[cfg(feature = "mruby-random")]
    rand::initialize_rand(vm);
    #[cfg(feature = "mruby-regexp")]
    regexp::initialize_regexp(vm);
    #[cfg(feature = "mruby-uri")]
    uri::initialize_uri(vm);
    initialize_env(vm);
}

// `ENV` as a plain Hash: carries the host's variables where there is a host
// to ask, else stays empty. The `wasi` feature alone cannot decide this:
// Cargo unifies features across the graph, so a wasm32-unknown-unknown build
// gets `wasi` switched on as soon as anything in the tree asks for it, and
// std::env::vars() panics there. Ask the target as well.
fn initialize_env(vm: &mut VM) {
    let env = hash::mrb_hash_new(vm, &[]).expect("ENV hash");
    #[cfg(all(feature = "wasi", any(not(target_family = "wasm"), target_os = "wasi")))]
    for (key, value) in std::env::vars() {
        use super::value::RObject;
        let key = RObject::string(key).to_refcount_assigned();
        let value = RObject::string(value).to_refcount_assigned();
        hash::mrb_hash_set_index(env.clone(), key, value).expect("ENV entry");
    }
    vm.consts.insert("ENV".to_string(), env);
}
