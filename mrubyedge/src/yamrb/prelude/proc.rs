use std::rc::Rc;

use crate::{
    Error,
    yamrb::{
        helpers::{mrb_call_block, mrb_define_class_cmethod, mrb_define_cmethod},
        value::*,
        vm::{Breadcrumb, VM},
    },
};

pub(crate) fn initialize_proc(vm: &mut VM) {
    let proc_class = vm.define_standard_class("Proc");

    mrb_define_class_cmethod(vm, proc_class.clone(), "new", Box::new(mrb_proc_new));

    mrb_define_cmethod(vm, proc_class.clone(), "call", Box::new(mrb_proc_call));
    mrb_define_cmethod(vm, proc_class.clone(), "[]", Box::new(mrb_proc_call));
    mrb_define_cmethod(vm, proc_class, "arity", Box::new(mrb_proc_arity));
}

// Proc#arity, read back from the ENTER instruction at the head of the body:
// the required count, negated and shifted by one when the signature is
// variadic. A body written in Rust has no ENTER, so it answers -1.
fn mrb_proc_arity(vm: &mut VM, _args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    use crate::rite::insn::{Fetched, OpCode};
    use crate::yamrb::optable::EnterArgInfo;

    let this = vm.getself()?;
    let proc_ = match &this.value {
        RValue::Proc(p) => p.clone(),
        _ => {
            return Err(Error::RuntimeError(
                "Proc#arity must be called on a Proc".to_string(),
            ));
        }
    };
    let arity = match proc_
        .irep
        .as_ref()
        .and_then(|irep| irep.code.first().cloned())
    {
        Some(op) if matches!(op.code, OpCode::ENTER) => {
            let operand = match op.operand {
                Fetched::W(w) => w,
                _ => 0,
            };
            let info = EnterArgInfo::from(operand);
            let required = info.m1 as i64 + info.m2 as i64;
            if info.o > 0 || info.r > 0 {
                -(required + 1)
            } else {
                required
            }
        }
        Some(_) => 0,
        None => -1,
    };
    Ok(RObject::integer(arity).to_refcount_assigned())
}

fn mrb_proc_new(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let block = args[0].clone();
    Ok(block)
}

pub fn mrb_proc_call(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    // handle Proc#call as special
    let cur = vm
        .current_breadcrumb
        .take()
        .expect("empty breadcrumb on call");
    let new_breadcrumb = Rc::new(Breadcrumb {
        upper: cur.upper.clone(),
        caller: Some("Proc#call".to_string()),
        event: "_proc_call_via_method",
        return_reg: cur.return_reg,
    });
    vm.current_breadcrumb.replace(new_breadcrumb);

    let this = vm.getself()?;
    mrb_call_block(vm, this.clone(), None, args, 0)
}
