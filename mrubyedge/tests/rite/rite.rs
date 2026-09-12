extern crate mrubyedge;

use super::helpers::*;

#[test]
fn test_rite_parse_hello_world() {
    let code = r#"
    puts "Hello, World!"
    "#;
    let binary = mrbc_compile("hello", code);
    let rite = mrubyedge::rite::load(&binary).unwrap();

    // Check binary header
    assert_eq!(&rite.binary_header.ident, b"RITE");
    assert_eq!(&rite.binary_header.major_version, b"04");
    assert_eq!(&rite.binary_header.minor_version, b"00");

    // Check IREP section
    assert_eq!(rite.irep.len(), 1);
    let irep = &rite.irep[0];

    // Should have at least some instructions
    assert!(!irep.insn.is_empty());

    // Should have some pool values (for the string)
    assert!(irep.plen > 0);
}

#[test]
fn test_rite_parse_with_local_variables() {
    let code = r#"
    def greet(name)
      message = "Hello, #{name}!"
      puts message
      message
    end
    
    greet("Alice")
    "#;
    let binary = mrbc_compile("greet", code);
    let rite = mrubyedge::rite::load(&binary).unwrap();

    // Check binary header
    assert_eq!(&rite.binary_header.ident, b"RITE");

    // Should have multiple ireps (main + function definition)
    assert!(rite.irep.len() == 2);
    assert!(rite.irep[0].lv.is_empty());
    assert!(rite.irep[1].lv.len() == 3); // name, (&nil), message

    // Check LVAR section exists
    let lvar = rite.lvar.unwrap();
    assert!(lvar.syms.len() == 2);

    let name = rite.irep[1].lv[0].as_ref().cloned().unwrap();
    assert_eq!(&name.to_string_lossy(), "name");

    let message = rite.irep[1].lv[2].as_ref().cloned().unwrap();
    assert_eq!(&message.to_string_lossy(), "message");
}

#[test]
fn test_rite_parse_pool_values() {
    let code = r#"
    a = "string"
    b = 42
    c = 3.14
    d = 9999999999
    [a, b, c, d]
    "#;
    let binary = mrbc_compile("pool", code);
    let rite = mrubyedge::rite::load(&binary).unwrap();

    assert_eq!(&rite.binary_header.ident, b"RITE");

    let irep = &rite.irep[0];
    assert!(rite.lvar.unwrap().syms.len() == 4);

    // Should have multiple pool values
    assert!(irep.pool.len() == 3);
    assert!(irep.lv.len() == 4);

    // Check pool value types
    use mrubyedge::rite::PoolValue;
    let has_string = irep
        .pool
        .iter()
        .any(|p| matches!(p, PoolValue::Str(_) | PoolValue::SStr(_)));
    let has_float = irep.pool.iter().any(|p| matches!(p, PoolValue::Float(_)));
    let has_int64 = irep.pool.iter().any(|p| matches!(p, PoolValue::Int64(_)));

    assert!(has_string, "Should have string in pool");
    assert!(has_float, "Should have float in pool");
    assert!(has_int64, "Should have int64 in pool");
}

#[test]
fn a_chunk_of_an_unknown_format_version_is_refused_test() {
    let binary = mrbc_compile("compiled", "1 + 1");

    // Bytes 4 and 5 of the header are the major version. mruby 3.x chunks are
    // refused too: the opcode numbering is not the one this VM decodes.
    for major in [b"02", b"03", b"05"] {
        let mut chunk = binary.clone();
        chunk[4..6].copy_from_slice(major);

        // Assert
        let err = mrubyedge::rite::load(&chunk).unwrap_err();
        assert!(matches!(err, mrubyedge::rite::Error::UnsupportedVersion(m) if &m == major));
    }
}

#[test]
fn a_chunk_whose_pool_holds_a_bignum_reads_the_entries_after_it_test() {
    let code = r#"
    big = 123456789012345678901234567890
    tail = "after"
    tail
    "#;
    let binary = mrbc_compile("bignum", code);

    let rite = mrubyedge::rite::load(&binary).unwrap();

    let irep = &rite.irep[0];
    use mrubyedge::rite::PoolValue;
    assert!(irep.pool.iter().any(|p| matches!(p, PoolValue::BigInt(_))));
    assert!(
        irep.pool.iter().any(
            |p| matches!(p, PoolValue::Str(s) | PoolValue::SStr(s) if s.as_slice() == b"after")
        )
    );
}

#[test]
fn a_chunk_that_does_not_start_with_rite_is_refused_test() {
    let binary = mrbc_compile("compiled", "1 + 1");

    for ident in [b"ZITE", b"RITF", b"\0\0\0\0"] {
        let mut chunk = binary.clone();
        chunk[0..4].copy_from_slice(ident);

        // Assert
        let err = mrubyedge::rite::load(&chunk).unwrap_err();
        assert!(matches!(err, mrubyedge::rite::Error::InvalidFormat));
    }
}

#[test]
fn a_chunk_from_a_newer_minor_version_is_refused_test() {
    let binary = mrbc_compile("compiled", "1 + 1");

    let mut chunk = binary.clone();
    chunk[6..8].copy_from_slice(b"01");
    let err = mrubyedge::rite::load(&chunk).unwrap_err();
    assert!(matches!(err, mrubyedge::rite::Error::InvalidFormat));

    // An older or equal minor version stays readable.
    let mut chunk = binary.clone();
    chunk[6..8].copy_from_slice(b"00");
    assert!(mrubyedge::rite::load(&chunk).is_ok());
}

#[test]
fn a_chunk_followed_by_other_bytes_reads_only_the_chunk_test() {
    let binary = mrbc_compile("compiled", "1 + 1");

    let mut chunk = binary.clone();
    chunk.extend_from_slice(b"trailing garbage");

    let mut rite = mrubyedge::rite::load(&chunk).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 2);
}

#[test]
fn a_chunk_whose_sections_run_out_before_the_end_marker_still_reads_test() {
    let binary = mrbc_compile("compiled", "1 + 1");

    // The END section is the last eight bytes; relabel it so the scan meets an
    // ident it does not know while the IREP section before it is already read.
    let mut chunk = binary.clone();
    let end = chunk.len() - 8;
    chunk[end..end + 4].copy_from_slice(b"XXXX");

    let mut rite = mrubyedge::rite::load(&chunk).unwrap();
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().unwrap();
    let result: i64 = result.as_ref().try_into().unwrap();

    // Assert
    assert_eq!(result, 2);
}
