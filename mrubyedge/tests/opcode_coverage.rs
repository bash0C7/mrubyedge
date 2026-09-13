extern crate mrubyedge;

mod helpers;
use helpers::*;

use std::collections::BTreeSet;

struct Section {
    name: String,
    expect: Expect,
    opcodes: BTreeSet<String>,
    ruby: String,
}

enum Expect {
    Int(i64),
    Str(String),
}

fn script() -> String {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/coverage/opcodes.rb");
    std::fs::read_to_string(path).unwrap()
}

fn parse(body: &str) -> (BTreeSet<String>, Vec<Section>) {
    let mut absent = BTreeSet::new();
    let mut sections: Vec<Section> = Vec::new();

    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("#absent ") {
            absent.extend(rest.split_whitespace().map(str::to_string));
        } else if let Some(rest) = line.strip_prefix("#section ") {
            let mut parts = rest.split('|');
            let name = parts.next().unwrap().trim().to_string();
            let expect = parts.next().unwrap().trim();
            let expect = match expect.split_once(' ') {
                Some(("int", v)) => Expect::Int(v.parse().unwrap()),
                Some(("str", v)) => Expect::Str(v.trim_matches('"').to_string()),
                _ => panic!("{name}: cannot read the expected value from {expect:?}"),
            };
            let opcodes = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(str::to_string)
                .collect();
            sections.push(Section {
                name,
                expect,
                opcodes,
                ruby: String::new(),
            });
        } else if let Some(section) = sections.last_mut() {
            section.ruby.push_str(line);
            section.ruby.push('\n');
        }
    }
    (absent, sections)
}

fn opcodes_of(binary: &[u8]) -> BTreeSet<String> {
    let rite = mrubyedge::rite::load(binary).unwrap();
    let mut found = BTreeSet::new();
    for irep in rite.irep.iter() {
        let mut stream: &[u8] = irep.insn;
        while !stream.is_empty() {
            let Ok((op, _operand, ext)) = mrubyedge::rite::insn::fetch_next(&mut stream) else {
                break;
            };
            found.insert(format!("{op:?}"));
            if ext != 0 {
                found.insert(format!("EXT{ext}"));
            }
        }
    }
    found
}

#[test]
fn every_section_emits_the_opcodes_it_names_test() {
    let body = script();
    let (_absent, sections) = parse(&body);

    for section in &sections {
        let ruby: &'static str = Box::leak(section.ruby.clone().into_boxed_str());
        let binary = mrbc_compile("coverage", ruby);
        let found = opcodes_of(&binary);

        // Assert
        let missing: Vec<&String> = section.opcodes.difference(&found).collect();
        assert!(
            missing.is_empty(),
            "section {} no longer emits {:?}",
            section.name,
            missing
        );
    }
}

#[test]
fn every_section_evaluates_to_the_value_it_names_test() {
    let body = script();
    let (_absent, sections) = parse(&body);

    for section in &sections {
        let ruby: &'static str = Box::leak(section.ruby.clone().into_boxed_str());
        let binary = mrbc_compile("coverage", ruby);
        let mut rite = mrubyedge::rite::load(&binary).unwrap();
        let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
        let result = vm
            .run()
            .unwrap_or_else(|e| panic!("section {} stopped with {e:?}", section.name));

        // Assert
        match &section.expect {
            Expect::Int(want) => {
                let got: i64 = result.as_ref().try_into().unwrap();
                assert_eq!(got, *want, "section {}", section.name);
            }
            Expect::Str(want) => {
                let got: String = result.as_ref().try_into().unwrap();
                assert_eq!(&got, want, "section {}", section.name);
            }
        }
    }
}

#[test]
fn the_opcodes_no_ruby_reaches_stay_unreachable_test() {
    let body = script();
    let (absent, sections) = parse(&body);

    for section in &sections {
        // Assert
        let reached: Vec<&String> = section.opcodes.intersection(&absent).collect();
        assert!(
            reached.is_empty(),
            "section {} reaches {:?}, which the script calls unreachable",
            section.name,
            reached
        );
    }
}

#[test]
fn the_sections_and_the_absent_list_account_for_every_opcode_test() {
    let body = script();
    let (absent, sections) = parse(&body);

    let mut covered: BTreeSet<String> = BTreeSet::new();
    for section in &sections {
        covered.extend(section.opcodes.iter().cloned());
    }
    let total = covered.len() + absent.len();

    // Assert
    assert!(covered.is_disjoint(&absent));
    assert_eq!(
        total, 119,
        "mruby 4.0 numbers 119 opcodes; the script accounts for {total}"
    );
}
