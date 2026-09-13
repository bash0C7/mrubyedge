use std::rc::Rc;

use crate::{
    Error,
    yamrb::{helpers::mrb_define_cmethod, value::*, vm::VM},
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
        module_class,
        "define_method",
        Box::new(mrb_module_define_method),
    );
}

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

fn method_name_of(obj: &Rc<RObject>, who: &str) -> Result<String, Error> {
    match &obj.value {
        RValue::Symbol(sym) => Ok(sym.name.clone()),
        RValue::String(..) => obj.as_ref().try_into(),
        _ => Err(Error::RuntimeError(format!(
            "{} expects a Symbol or String",
            who
        ))),
    }
}

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
        .map(|m| RObject::module(m.clone()).to_refcount_assigned())
        .collect();
    Ok(RObject::array(ancestors).to_refcount_assigned())
}
