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
use mrubyedge::yamrb::value::RSym;
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
