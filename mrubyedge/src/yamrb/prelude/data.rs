use std::cell::Cell;
use std::rc::Rc;

use crate::{
    Error,
    yamrb::{
        helpers::{mrb_define_class_cmethod, mrb_define_cmethod, mrb_funcall},
        value::{RClass, RHashMap, RObject, RSym, RValue},
        vm::VM,
    },
};

// The generated classes are anonymous in Ruby, but RObject::class caches
// class wrappers by name, so each one still needs a name of its own.
thread_local! {
    static DEFINED_COUNT: Cell<usize> = const { Cell::new(0) };
}

// Where a generated class keeps its member list.
const MEMBERS: &str = "__data_members__";

// A Data instance, the class behind it, and the members that class declares.
type DataSelf = (Rc<RObject>, Rc<RClass>, Vec<RSym>);

pub(crate) fn initialize_data(vm: &mut VM) {
    let data_class = vm.define_standard_class("Data");
    mrb_define_class_cmethod(vm, data_class, "define", Box::new(mrb_data_define));
}

fn members_of(class: &Rc<RClass>) -> Vec<RSym> {
    let members = class.consts.borrow().get(MEMBERS).cloned();
    match members.as_ref().map(|m| &m.value) {
        Some(RValue::Array(names)) => names
            .borrow()
            .iter()
            .filter_map(|name| match &name.value {
                RValue::Symbol(sym) => Some(sym.clone()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn self_class(vm: &mut VM, who: &str) -> Result<Rc<RClass>, Error> {
    match &vm.getself()?.value {
        RValue::Class(class) => Ok(class.clone()),
        _ => Err(Error::RuntimeError(format!(
            "{} must be called on a class",
            who
        ))),
    }
}

fn self_members(vm: &mut VM, who: &str) -> Result<DataSelf, Error> {
    let this = vm.getself()?;
    let class = match &this.value {
        RValue::Instance(instance) => instance.class.clone(),
        _ => {
            return Err(Error::RuntimeError(format!(
                "{} must be called on a Data",
                who
            )));
        }
    };
    let members = members_of(&class);
    Ok((this, class, members))
}

fn mrb_data_define(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let mut members = Vec::with_capacity(args.len());
    for arg in args.iter() {
        match &arg.value {
            RValue::Symbol(sym) => members.push(sym.clone()),
            // A block would arrive here too; Data.define takes one to add
            // methods, which needs a Ruby-level class body we cannot run.
            RValue::String(_, _) => members.push(RSym::new(arg.as_ref().try_into()?)),
            _ => {
                return Err(Error::ArgumentError(
                    "Data.define expects Symbol member names".to_string(),
                ));
            }
        }
    }

    let serial = DEFINED_COUNT.with(|count| {
        let next = count.get() + 1;
        count.set(next);
        next
    });
    let superclass = vm.get_class_by_name("Data");
    let class = Rc::new(RClass::new(
        &format!("Data:{}", serial),
        Some(superclass),
        None,
    ));
    class.update_module_weakref();
    class.consts.borrow_mut().insert(
        MEMBERS.to_string(),
        RObject::array(
            members
                .iter()
                .map(|name| RObject::symbol(name.clone()).to_refcount_assigned())
                .collect(),
        )
        .to_refcount_assigned(),
    );

    for name in members.iter() {
        let ivar = format!("@{}", name.name);
        let reader = move |vm: &mut VM, _args: &[Rc<RObject>]| {
            let this = vm.getself()?;
            Ok(this.get_ivar(&ivar))
        };
        mrb_define_cmethod(vm, class.clone(), &name.name, Box::new(reader));
    }

    mrb_define_class_cmethod(vm, class.clone(), "new", Box::new(mrb_data_new));
    mrb_define_class_cmethod(vm, class.clone(), "[]", Box::new(mrb_data_new));
    mrb_define_class_cmethod(
        vm,
        class.clone(),
        "members",
        Box::new(mrb_data_class_members),
    );
    mrb_define_cmethod(vm, class.clone(), "members", Box::new(mrb_data_members));
    mrb_define_cmethod(vm, class.clone(), "to_h", Box::new(mrb_data_to_h));
    mrb_define_cmethod(vm, class.clone(), "with", Box::new(mrb_data_with));
    mrb_define_cmethod(vm, class.clone(), "==", Box::new(mrb_data_eq));
    mrb_define_cmethod(vm, class.clone(), "eql?", Box::new(mrb_data_eq));
    mrb_define_cmethod(vm, class.clone(), "inspect", Box::new(mrb_data_inspect));
    mrb_define_cmethod(vm, class.clone(), "to_s", Box::new(mrb_data_inspect));

    Ok(RObject::class(class, vm))
}

// `Point.new(1, 2)` and `Point.new(x: 1, y: 2)` both build the same thing.
fn mrb_data_new(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let class = self_class(vm, "Data.new")?;
    let members = members_of(&class);
    let kwargs = vm.get_kwargs().unwrap_or_default();

    if !kwargs.is_empty() {
        for key in kwargs.keys() {
            if !members.iter().any(|name| name.name == *key) {
                return Err(Error::ArgumentError(format!("unknown keyword: :{}", key)));
            }
        }
    } else if args.len() > members.len() {
        return Err(Error::ArgumentError(format!(
            "wrong number of arguments (given {}, expected 0..{})",
            args.len(),
            members.len()
        )));
    }

    let instance = RObject::instance(class).to_refcount_assigned();
    for (i, name) in members.iter().enumerate() {
        let value = if kwargs.is_empty() {
            args.get(i).cloned()
        } else {
            kwargs.get(&name.name).cloned()
        };
        let value = value
            .ok_or_else(|| Error::ArgumentError(format!("missing keyword: :{}", name.name)))?;
        instance.set_ivar(&format!("@{}", name.name), value);
    }

    Ok(instance)
}

fn mrb_data_class_members(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let class = self_class(vm, "Data.members")?;
    Ok(RObject::array(
        members_of(&class)
            .into_iter()
            .map(|name| RObject::symbol(name).to_refcount_assigned())
            .collect(),
    )
    .to_refcount_assigned())
}

fn mrb_data_members(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let (_, _, members) = self_members(vm, "Data#members")?;
    Ok(RObject::array(
        members
            .into_iter()
            .map(|name| RObject::symbol(name).to_refcount_assigned())
            .collect(),
    )
    .to_refcount_assigned())
}

fn mrb_data_to_h(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let (this, _, members) = self_members(vm, "Data#to_h")?;
    let mut map = RHashMap::default();
    for name in members.iter() {
        let key = RObject::symbol(name.clone()).to_refcount_assigned();
        let value = this.get_ivar(&format!("@{}", name.name));
        map.insert(key.as_hash_key()?, (key, value));
    }
    Ok(RObject::hash(map).to_refcount_assigned())
}

// `point.with(y: 3)` -- a copy carrying the named members replaced.
fn mrb_data_with(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let (this, class, members) = self_members(vm, "Data#with")?;
    let overrides = vm.get_kwargs().unwrap_or_default();
    for key in overrides.keys() {
        if !members.iter().any(|name| name.name == *key) {
            return Err(Error::ArgumentError(format!("unknown keyword: :{}", key)));
        }
    }

    let copy = RObject::instance(class).to_refcount_assigned();
    for name in members.iter() {
        let value = match overrides.get(&name.name) {
            Some(value) => value.clone(),
            None => this.get_ivar(&format!("@{}", name.name)),
        };
        copy.set_ivar(&format!("@{}", name.name), value);
    }
    Ok(copy)
}

fn mrb_data_eq(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let (this, class, members) = self_members(vm, "Data#==")?;
    let Some(other) = args.first() else {
        return Err(Error::ArgumentError(
            "Data#== expects an argument".to_string(),
        ));
    };
    let same_class = match &other.value {
        RValue::Instance(instance) => Rc::ptr_eq(&instance.class, &class),
        _ => false,
    };
    if !same_class {
        return Ok(RObject::boolean(false).to_refcount_assigned());
    }

    for name in members.iter() {
        let ivar = format!("@{}", name.name);
        let mine = this.get_ivar(&ivar);
        let theirs = other.get_ivar(&ivar);
        let equal = mrb_funcall(vm, Some(mine), "==", &[theirs])?;
        if !matches!(equal.value, RValue::Bool(true)) {
            return Ok(RObject::boolean(false).to_refcount_assigned());
        }
    }
    Ok(RObject::boolean(true).to_refcount_assigned())
}

fn mrb_data_inspect(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let (this, _, members) = self_members(vm, "Data#inspect")?;
    let mut parts = Vec::with_capacity(members.len());
    for name in members.iter() {
        let value = this.get_ivar(&format!("@{}", name.name));
        let shown = mrb_funcall(vm, Some(value), "inspect", &[])?;
        let shown: String = shown.as_ref().try_into()?;
        parts.push(format!("{}={}", name.name, shown));
    }
    Ok(Rc::new(RObject::string(format!(
        "#<data {}>",
        parts.join(", ")
    ))))
}
