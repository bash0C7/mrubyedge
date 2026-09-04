// The opcodes mruby 4.0 (RITE0400) added, one by one.
//
// The harness compiles with mruby 3.3, so these IREPs are built by hand the
// way examples/newvm.rs does, and each pins the semantics mruby 4.0's vm.c
// gives the instruction.
extern crate mrubyedge;

use std::rc::Rc;

use mrubyedge::Error;
use mrubyedge::rite::insn::Fetched::{self, *};
use mrubyedge::rite::insn::OpCode::{self, *};
use mrubyedge::yamrb::op::Op;
use mrubyedge::yamrb::value::{RObject, RSym, RValue};
use mrubyedge::yamrb::vm::{IREP, VM};

fn irep(
    id: usize,
    nregs: usize,
    code: &[(OpCode, Fetched)],
    syms: &[&str],
    reps: Vec<IREP>,
) -> IREP {
    let mut pos = 0;
    let code = code
        .iter()
        .map(|(opcode, operand)| {
            let len = 1 + operand.len();
            let op = Op::new(*opcode, *operand, pos, len);
            pos += len;
            op
        })
        .collect();
    IREP {
        __id: id,
        nlocals: 1,
        nregs,
        rlen: reps.len(),
        code,
        syms: syms.iter().map(|s| RSym::new(s.to_string())).collect(),
        pool: Vec::new(),
        reps: reps.into_iter().map(Rc::new).collect(),
        lv: None,
        catch_target_pos: Vec::new(),
        catch_ranges: Vec::new(),
    }
}

// def m(&nil); 1; end  -- ENTER with n1 (bit 23) set.
fn method_refusing_a_block() -> IREP {
    irep(
        1,
        2,
        &[(ENTER, W(1 << 23)), (LOADI_1, B(1)), (RETURN, B(1))],
        &[],
        vec![],
    )
}

fn empty_block() -> IREP {
    irep(
        2,
        2,
        &[(ENTER, W(0)), (LOADNIL, B(1)), (RETURN, B(1))],
        &[],
        vec![],
    )
}

#[test]
fn enter_with_n1_refuses_a_block_test() {
    // def m(&nil); 1; end; m { }
    let main = irep(
        0,
        4,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (BLOCK, BB(2, 1)),
            (SSENDB, BBB(1, 0, 0)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["m"],
        vec![method_refusing_a_block(), empty_block()],
    );
    let mut vm = VM::new_by_raw_irep(main);
    let err = vm.run().unwrap_err();
    let err = err.downcast_ref::<Error>().expect("a VM error");

    // Assert
    assert!(
        matches!(err, Error::ArgumentError(msg) if msg == "no block accepted"),
        "{:?}",
        err
    );
}

#[test]
fn enter_with_n1_runs_without_a_block_test() {
    // def m(&nil); 1; end; m
    let main = irep(
        0,
        4,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (SSEND, BBB(1, 0, 0)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["m"],
        vec![method_refusing_a_block()],
    );
    let mut vm = VM::new_by_raw_irep(main);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 1);
}

// `def m; <body>; end; m` with the body handed in as instructions. Returns
// what the call answered, alongside the VM for anything else to inspect.
fn call_method(nregs: usize, body: &[(OpCode, Fetched)]) -> (VM, Rc<RObject>) {
    let method = irep(1, nregs, body, &[], vec![]);
    let main = irep(
        0,
        4,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (SSEND, BBB(1, 0, 0)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["m"],
        vec![method],
    );
    let mut vm = VM::new_by_raw_irep(main);
    let result = vm.run().unwrap();
    (vm, result)
}

#[test]
fn retself_returns_the_receiver_test() {
    // def m; self; end; m == self
    let method = irep(1, 2, &[(RETSELF, Z)], &[], vec![]);
    let main = irep(
        0,
        4,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (SSEND, BBB(1, 0, 0)),
            (LOADSELF, B(2)),
            (EQ, B(1)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["m"],
        vec![method],
    );
    let mut vm = VM::new_by_raw_irep(main);
    let result = vm.run().unwrap();

    // Assert
    assert!(matches!(result.value, RValue::Bool(true)));
}

#[test]
fn retself_is_not_the_last_value_computed_test() {
    // def m; 1; self; end; m == 1
    let method = irep(1, 2, &[(LOADI_1, B(1)), (RETSELF, Z)], &[], vec![]);
    let main = irep(
        0,
        4,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (SSEND, BBB(1, 0, 0)),
            (LOADI_1, B(2)),
            (EQ, B(1)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["m"],
        vec![method],
    );
    let mut vm = VM::new_by_raw_irep(main);
    let result = vm.run().unwrap();

    // Assert
    assert!(matches!(result.value, RValue::Bool(false)));
}

#[test]
fn retnil_returns_nil_test() {
    let (_, result) = call_method(2, &[(LOADI_1, B(1)), (RETNIL, Z)]);

    // Assert
    assert!(result.is_nil());
}

#[test]
fn rettrue_returns_true_test() {
    let (_, result) = call_method(2, &[(LOADI_1, B(1)), (RETTRUE, Z)]);

    // Assert
    assert!(matches!(result.value, RValue::Bool(true)));
}

#[test]
fn retfalse_returns_false_test() {
    let (_, result) = call_method(2, &[(LOADI_1, B(1)), (RETFALSE, Z)]);

    // Assert
    assert!(matches!(result.value, RValue::Bool(false)));
}

// `def m; 7; end`, the method every send test calls.
fn method_returning_7() -> IREP {
    irep(1, 2, &[(LOADI_7, B(1)), (RETURN, B(1))], &[], vec![])
}

fn run_main(
    nregs: usize,
    code: &[(OpCode, Fetched)],
    syms: &[&str],
    reps: Vec<IREP>,
) -> Rc<RObject> {
    let mut vm = VM::new_by_raw_irep(irep(0, nregs, code, syms, reps));
    vm.run().unwrap()
}

#[test]
fn ssend0_sends_to_self_with_no_arguments_test() {
    // def m; 7; end; m
    let result = run_main(
        4,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (SSEND0, BB(1, 0)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["m"],
        vec![method_returning_7()],
    );
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 7);
}

#[test]
fn send0_sends_to_the_receiver_in_the_register_test() {
    // def m; 7; end; self.m
    let result = run_main(
        4,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (LOADSELF, B(1)),
            (SEND0, BB(1, 0)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["m"],
        vec![method_returning_7()],
    );
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 7);
}

#[test]
fn send0_reaches_a_method_written_in_rust_test() {
    // 3.to_s
    let result = run_main(
        3,
        &[
            (LOADI_3, B(1)),
            (SEND0, BB(1, 0)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["to_s"],
        vec![],
    );
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, "3");
}

#[test]
fn getidx0_reads_the_first_element_test() {
    // [5, 6][0]
    let result = run_main(
        5,
        &[
            (LOADI_5, B(2)),
            (LOADI_6, B(3)),
            (ARRAY, BB(2, 2)),
            (GETIDX0, BB(1, 2)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &[],
        vec![],
    );
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 5);
}

#[test]
fn getidx0_asks_an_object_for_its_own_index_method_test() {
    // def [](i); 42; end; self[0]
    let index_method = irep(
        1,
        3,
        &[(ENTER, W(1 << 18)), (LOADI, BB(2, 42)), (RETURN, B(2))],
        &[],
        vec![],
    );
    let result = run_main(
        5,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (LOADSELF, B(2)),
            (GETIDX0, BB(1, 2)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["[]"],
        vec![index_method],
    );
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 42);
}

#[test]
fn addilv_adds_the_immediate_to_an_integer_local_test() {
    // a = 1; a += 4
    let result = run_main(
        4,
        &[
            (LOADI_1, B(1)),
            (ADDILV, BBB(1, 2, 4)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &[],
        vec![],
    );
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 5);
}

#[test]
fn subilv_subtracts_the_immediate_from_an_integer_local_test() {
    // a = 7; a -= 2
    let result = run_main(
        4,
        &[
            (LOADI_7, B(1)),
            (SUBILV, BBB(1, 2, 2)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &[],
        vec![],
    );
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 5);
}

#[test]
fn addilv_adds_to_a_float_local_test() {
    // a = 1.to_f; a += 4
    let result = run_main(
        4,
        &[
            (LOADI_1, B(1)),
            (SEND0, BB(1, 0)),
            (ADDILV, BBB(1, 2, 4)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["to_f"],
        vec![],
    );

    // Assert
    assert!(
        matches!(result.value, RValue::Float(f) if f == 5.0),
        "{:?}",
        result
    );
}

#[test]
fn addilv_sends_plus_to_anything_else_test() {
    // def +(n); n; end; a = self; a += 4
    let plus = irep(1, 3, &[(ENTER, W(1 << 18)), (RETURN, B(1))], &[], vec![]);
    let result = run_main(
        5,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (LOADSELF, B(1)),
            (ADDILV, BBB(1, 3, 4)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["+"],
        vec![plus],
    );
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 4);
}

#[test]
fn subilv_sends_minus_to_anything_else_test() {
    // def -(n); 6; end; a = self; a -= 4
    let minus = irep(
        1,
        3,
        &[(ENTER, W(1 << 18)), (LOADI_6, B(2)), (RETURN, B(2))],
        &[],
        vec![],
    );
    let result = run_main(
        5,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (LOADSELF, B(1)),
            (SUBILV, BBB(1, 3, 4)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &["-"],
        vec![minus],
    );
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 6);
}

#[test]
fn addilv_leaves_the_other_locals_alone_when_it_sends_test() {
    // def +(n); n; end; other = 3; a = self; a += 4; other
    let plus = irep(1, 3, &[(ENTER, W(1 << 18)), (RETURN, B(1))], &[], vec![]);
    let result = run_main(
        6,
        &[
            (TCLASS, B(1)),
            (METHOD, BB(2, 0)),
            (DEF, BB(1, 0)),
            (LOADI_3, B(2)),
            (LOADSELF, B(1)),
            (ADDILV, BBB(1, 4, 4)),
            (RETURN, B(2)),
            (STOP, Z),
        ],
        &["+"],
        vec![plus],
    );
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 3);
}

#[test]
fn addilv_on_an_object_without_plus_is_a_no_method_error_test() {
    // a = nil; a += 1
    let mut vm = VM::new_by_raw_irep(irep(
        0,
        4,
        &[
            (LOADNIL, B(1)),
            (ADDILV, BBB(1, 2, 1)),
            (RETURN, B(1)),
            (STOP, Z),
        ],
        &[],
        vec![],
    ));
    let err = vm.run().unwrap_err();
    let err = err.downcast_ref::<Error>().expect("a VM error");

    // Assert
    assert!(matches!(err, Error::NoMethodError(_)), "{:?}", err);
}
