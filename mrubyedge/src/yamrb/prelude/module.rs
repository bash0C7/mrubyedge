use std::rc::Rc;

use crate::{
    Error,
    yamrb::{
        helpers::{mrb_call_block, mrb_call_hook, mrb_define_cmethod},
        value::*,
        vm::VM,
    },
};

pub(crate) fn initialize_module(vm: &mut VM) {
    let module_class = vm.define_standard_class("Module");
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "include",
        Box::new(mrb_module_include),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "ancestors",
        Box::new(mrb_module_ancestors),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "define_method",
        Box::new(mrb_module_define_method),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "module_eval",
        Box::new(mrb_module_module_eval),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "class_eval",
        Box::new(mrb_module_module_eval),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "instance_methods",
        Box::new(mrb_module_instance_methods),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "method_defined?",
        Box::new(mrb_module_method_defined),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "instance_method",
        Box::new(mrb_module_instance_method),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "private_instance_methods",
        Box::new(mrb_module_private_instance_methods),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "const_defined?",
        Box::new(mrb_module_const_defined),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "const_get",
        Box::new(mrb_module_const_get),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "const_set",
        Box::new(mrb_module_const_set),
    );
    // The VM has no method visibility, so these only have to accept the
    // declaration and stay out of the way.
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "private",
        Box::new(mrb_module_visibility_noop),
    );
    mrb_define_cmethod(
        vm,
        module_class.clone(),
        "protected",
        Box::new(mrb_module_visibility_noop),
    );
    mrb_define_cmethod(
        vm,
        module_class,
        "public",
        Box::new(mrb_module_visibility_noop),
    );
}

// Module#instance_method(name): the method body as a Proc (not a full UnboundMethod).
fn mrb_module_instance_method(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let self_obj = vm.getself()?;
    let name_obj = args
        .first()
        .ok_or_else(|| Error::RuntimeError("Module#instance_method expects a name".to_string()))?;
    let name = method_name_of(name_obj, "Module#instance_method")?;

    let lookup = match &self_obj.value {
        RValue::Class(klass) => resolve_method(klass, &name).map(|(_, method)| method),
        RValue::Module(module) => build_module_lookup_chain(module)
            .iter()
            .find_map(|m| m.procs.borrow().get(&name).cloned()),
        _ => {
            return Err(Error::RuntimeError(
                "Module#instance_method must be called on class or module".to_string(),
            ));
        }
    };
    let method = lookup.ok_or_else(|| Error::NameError(name))?;
    Ok(RObject::proc(method).to_refcount_assigned())
}

// Module#private_instance_methods: no method visibility, so always empty.
fn mrb_module_private_instance_methods(
    _vm: &mut VM,
    _args: &[Rc<RObject>],
) -> Result<Rc<RObject>, Error> {
    Ok(RObject::array(vec![]).to_refcount_assigned())
}

// Module#const_defined?(name)
fn mrb_module_const_defined(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let module = self_as_module(vm, "Module#const_defined?")?;
    let name_obj = args
        .first()
        .ok_or_else(|| Error::RuntimeError("Module#const_defined? expects a name".to_string()))?;
    let name = method_name_of(name_obj, "Module#const_defined?")?;
    let defined = module.consts.borrow().contains_key(&name);
    Ok(RObject::boolean(defined).to_refcount_assigned())
}

// Module#const_get(name)
fn mrb_module_const_get(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let module = self_as_module(vm, "Module#const_get")?;
    let name_obj = args
        .first()
        .ok_or_else(|| Error::RuntimeError("Module#const_get expects a name".to_string()))?;
    let name = method_name_of(name_obj, "Module#const_get")?;
    let value = module.consts.borrow().get(&name).cloned();
    value.ok_or(Error::NameError(name))
}

// Module#const_set(name, value)
fn mrb_module_const_set(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let module = self_as_module(vm, "Module#const_set")?;
    let name_obj = args
        .first()
        .ok_or_else(|| Error::RuntimeError("Module#const_set expects a name".to_string()))?;
    let name = method_name_of(name_obj, "Module#const_set")?;
    let value = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| RObject::nil().to_refcount_assigned());
    module.consts.borrow_mut().insert(name, value.clone());
    Ok(value)
}

// Module#private / #protected / #public: accepted and returned unchanged; no visibility modeled.
fn mrb_module_visibility_noop(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    match args.first() {
        Some(arg) => Ok(arg.clone()),
        None => Ok(RObject::nil().to_refcount_assigned()),
    }
}

// The receiver of a Module method, as a module. `Class` inherits from
// `Module`, so both arrive here.
fn self_as_module(vm: &mut VM, who: &str) -> Result<Rc<RModule>, Error> {
    let self_obj = vm.getself()?;
    match &self_obj.value {
        RValue::Class(klass) => Ok(klass.as_module()),
        RValue::Module(module) => Ok(module.clone()),
        _ => Err(Error::RuntimeError(format!(
            "{} must be called on class or module",
            who
        ))),
    }
}

// Module#define_method(name) { ... }: stores the block as the method body, environ intact.
fn mrb_module_define_method(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let module = self_as_module(vm, "Module#define_method")?;
    let name_obj = args
        .first()
        .ok_or_else(|| Error::RuntimeError("Module#define_method expects a name".to_string()))?;
    let name = method_name_of(name_obj, "Module#define_method")?;

    let body = args.get(1).ok_or_else(|| {
        Error::RuntimeError("Module#define_method expects a block or a Proc".to_string())
    })?;
    let mut method = match &body.value {
        RValue::Proc(p) => p.clone(),
        _ => {
            return Err(Error::RuntimeError(
                "Module#define_method expects a block or a Proc".to_string(),
            ));
        }
    };
    method.sym_id = Some(RSym::new(name.clone()));

    module.procs.borrow_mut().insert(name.clone(), method);
    Ok(RObject::symbol(RSym::new(name)).to_refcount_assigned())
}

// Module#module_eval { ... } / Module#class_eval { ... }: evaluates the block with the module as self.
fn mrb_module_module_eval(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let self_obj = vm.getself()?;
    match &self_obj.value {
        RValue::Class(_) | RValue::Module(_) => {}
        _ => {
            return Err(Error::RuntimeError(
                "Module#module_eval must be called on class or module".to_string(),
            ));
        }
    }
    let block = args
        .last()
        .ok_or_else(|| Error::RuntimeError("Module#module_eval expects a block".to_string()))?;
    match &block.value {
        RValue::Proc(_) => {}
        _ => {
            return Err(Error::RuntimeError(
                "Module#module_eval expects a block".to_string(),
            ));
        }
    }
    mrb_call_block(vm, block.clone(), Some(self_obj), &[], 0)
}

// Module#instance_methods(include_super = true) -> [Symbol]
fn mrb_module_instance_methods(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let self_obj = vm.getself()?;
    let include_super = !matches!(
        args.first().map(|a| &a.value),
        Some(RValue::Bool(false)) | Some(RValue::Nil)
    );

    let modules: Vec<Rc<RModule>> = match (&self_obj.value, include_super) {
        (RValue::Class(klass), true) => build_lookup_chain(klass)
            .iter()
            .map(|c| c.as_module())
            .collect(),
        (RValue::Class(klass), false) => vec![klass.as_module()],
        (RValue::Module(module), true) => build_module_lookup_chain(module),
        (RValue::Module(module), false) => vec![module.clone()],
        _ => {
            return Err(Error::RuntimeError(
                "Module#instance_methods must be called on class or module".to_string(),
            ));
        }
    };

    let mut names: Vec<String> = vec![];
    for module in modules.iter() {
        for name in module.procs.borrow().keys() {
            if !names.contains(name) {
                names.push(name.clone());
            }
        }
    }
    let syms: Vec<Rc<RObject>> = names
        .into_iter()
        .map(|name| RObject::symbol(RSym::new(name)).to_refcount_assigned())
        .collect();
    Ok(RObject::array(syms).to_refcount_assigned())
}

// Module#method_defined?(name)
fn mrb_module_method_defined(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let name_obj = args
        .first()
        .ok_or_else(|| Error::RuntimeError("Module#method_defined? expects a name".to_string()))?;
    let name = method_name_of(name_obj, "Module#method_defined?")?;
    let found = mrb_module_instance_methods(vm, &[])?;
    let found = match &found.value {
        RValue::Array(a) => a.borrow().iter().any(|s| match &s.value {
            RValue::Symbol(sym) => sym.name == name,
            _ => false,
        }),
        _ => false,
    };
    Ok(RObject::boolean(found).to_refcount_assigned())
}

// Module.new -> an anonymous module; a block, if given, is module_eval'd against it.
pub(crate) fn mrb_module_class_new(
    vm: &mut VM,
    args: &[Rc<RObject>],
) -> Result<Rc<RObject>, Error> {
    let module = Rc::new(RModule::new(""));
    let module_obj = RObject::module(module).to_refcount_assigned();

    if let Some(block) = args.last()
        && matches!(&block.value, RValue::Proc(_))
    {
        mrb_call_block(vm, block.clone(), Some(module_obj.clone()), &[], 0)?;
    }
    Ok(module_obj)
}

fn mrb_module_include(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    if args.is_empty() {
        return Err(Error::RuntimeError(
            "Module#include expects at least one module".to_string(),
        ));
    }

    let arg0 = &args[0];
    let mixin = match &arg0.value {
        RValue::Module(module) => module.clone(),
        _ => {
            return Err(Error::RuntimeError(
                "Module#include expects module arguments".to_string(),
            ));
        }
    };

    let self_obj = vm.getself()?;
    match &self_obj.value {
        RValue::Class(klass) => mrb_include_module(klass, mixin)?,
        RValue::Module(module) => mrb_include_module(module, mixin)?,
        _ => {
            return Err(Error::RuntimeError(
                "Module#include must be called on class or module".to_string(),
            ));
        }
    };

    // `def self.included(base)` on the mixin, the usual place a module hangs
    // its ClassMethods off the including class.
    mrb_call_hook(
        vm,
        arg0.clone(),
        "included",
        std::slice::from_ref(&self_obj),
    )?;

    Ok(self_obj)
}

/// Public helper.
/// Includes `mixin` module into `target`.
pub fn mrb_include_module(target: &impl AsModule, mixin: Rc<RModule>) -> Result<(), Error> {
    let target = target.as_module();
    if Rc::ptr_eq(&target, &mixin) {
        return Err(Error::RuntimeError("cannot include itself".to_string()));
    }

    let already_present = {
        let modules = target.mixed_in_modules.borrow();
        modules.iter().any(|m| Rc::ptr_eq(m, &mixin))
    };

    if already_present {
        return Err(Error::RuntimeError("module already included".to_string()));
    }

    target.mixed_in_modules.borrow_mut().insert(0, mixin);
    Ok(())
}

fn mrb_module_ancestors(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let self_module = vm.getself()?;
    let target_module = match &self_module.value {
        RValue::Module(module) => module.clone(),
        _ => {
            return Err(Error::RuntimeError(
                "Module#ancestors must be called on class or module".to_string(),
            ));
        }
    };
    let ancestors: Vec<Rc<RObject>> = build_module_lookup_chain(&target_module)
        .iter()
        .map(|m| RObject::module_of(m.clone(), vm))
        .collect();
    Ok(RObject::array(ancestors).to_refcount_assigned())
}

// A method name given as either a Symbol or a String.
pub(crate) fn method_name_of(obj: &Rc<RObject>, who: &str) -> Result<String, Error> {
    match &obj.value {
        RValue::Symbol(sym) => Ok(sym.name.clone()),
        RValue::String(..) => obj.as_ref().try_into(),
        _ => Err(Error::RuntimeError(format!(
            "{} expects a Symbol or String",
            who
        ))),
    }
}
