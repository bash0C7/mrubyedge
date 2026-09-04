// mruby 4.0 (RITE0400) bytecode.
//
// The test harness compiles with mruby 3.3, so a 4.0 chunk cannot be built
// here. tests/fixtures/mruby40.mrb is checked in instead: PicoRuby 4.0.3's
// own mrbc compiled tests/fixtures/mruby40.rb (`mrbc -o mruby40.mrb
// mruby40.rb`), which is written to reach every opcode 4.0 added except
// MATCHERR (that one needs a `case/in` that fails to match). Recompile it
// whenever mruby40.rb changes.
extern crate mrubyedge;

use mrubyedge::rite::insn::{OpCode, RiteVersion};

const MRUBY40: &[u8] = include_bytes!("fixtures/mruby40.mrb");

#[test]
fn the_fixture_really_is_mruby_40_bytecode_test() {
    let rite = mrubyedge::rite::load(MRUBY40).unwrap();

    // Assert
    assert_eq!(&rite.binary_header.ident, b"RITE");
    assert_eq!(&rite.binary_header.major_version, b"04");
    assert_eq!(
        RiteVersion::from_major(&rite.binary_header.major_version).unwrap(),
        RiteVersion::V4
    );
}

#[test]
fn a_chunk_of_an_unknown_format_version_is_refused_test() {
    // Bytes 4 and 5 of the header are the major version.
    for major in [b"02", b"05"] {
        let mut chunk = MRUBY40.to_vec();
        chunk[4..6].copy_from_slice(major);

        // Assert
        assert!(mrubyedge::rite::load(&chunk).is_err());
    }
}

#[test]
fn the_fixture_reaches_the_opcodes_4_0_added_test() {
    let rite = mrubyedge::rite::load(MRUBY40).unwrap();
    let version = RiteVersion::from_major(&rite.binary_header.major_version).unwrap();

    let mut seen: Vec<String> = Vec::new();
    for irep in rite.irep.iter() {
        let mut insns: &[u8] = irep.insn;
        while !insns.is_empty() {
            let (opcode, fetch) = version.decode(insns[0]).unwrap();
            fetch(&mut insns).unwrap();
            let name = format!("{:?}", opcode);
            if !seen.contains(&name) {
                seen.push(name);
            }
        }
    }

    // Assert
    for expected in [
        "GETIDX0", "SSEND0", "SEND0", "BLKCALL", "RETSELF", "RETNIL", "RETTRUE", "RETFALSE",
        "ADDILV", "SUBILV", "TDEF", "SDEF",
    ] {
        assert!(
            seen.iter().any(|name| name == expected),
            "the fixture no longer reaches {}; seen: {:?}",
            expected,
            seen
        );
    }
}

#[test]
fn the_two_numberings_diverge_where_4_0_inserted_an_opcode_test() {
    // Identical up to 35, then 4.0 slips GETIDX0 in and everything shifts.
    for byte in 0..=35u8 {
        let (v3, _) = RiteVersion::V3.decode(byte).unwrap();
        let (v4, _) = RiteVersion::V4.decode(byte).unwrap();
        assert_eq!(
            format!("{:?}", v3),
            format!("{:?}", v4),
            "byte {} should mean the same thing in both",
            byte
        );
    }

    let (v3, _) = RiteVersion::V3.decode(36).unwrap();
    let (v4, _) = RiteVersion::V4.decode(36).unwrap();

    // Assert
    assert!(matches!(v3, OpCode::SETIDX));
    assert!(matches!(v4, OpCode::GETIDX0));

    // 3.3 has 106 opcodes, 4.0 has 119.
    assert!(RiteVersion::V3.decode(105).is_ok());
    assert!(RiteVersion::V3.decode(106).is_err());
    assert!(RiteVersion::V4.decode(118).is_ok());
    assert!(RiteVersion::V4.decode(119).is_err());
}

#[test]
fn mruby_40_bytecode_runs_test() {
    let mut rite = mrubyedge::rite::load(MRUBY40).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: String = result.as_ref().try_into().unwrap();

    // Assert
    // counter.value | locals | first_of | through_block | bare | empty.nil? |
    // yes | no, see tests/fixtures/mruby40.rb
    assert_eq!(result, "12,5-1-3,1,6,7,1,1,1");
}
