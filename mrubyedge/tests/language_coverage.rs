extern crate mrubyedge;

mod helpers;
use helpers::*;

struct Section {
    name: String,
    expect: Expect,
    broken: bool,
    ruby: String,
}

enum Expect {
    Int(i64),
    Str(String),
}

fn script() -> String {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/coverage/language.rb");
    std::fs::read_to_string(path).unwrap()
}

fn parse(body: &str) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::new();
    for line in body.lines() {
        let marked = line
            .strip_prefix("#section ")
            .map(|rest| (false, rest))
            .or_else(|| line.strip_prefix("#broken ").map(|rest| (true, rest)));
        if let Some((broken, rest)) = marked {
            let mut parts = rest.split('|');
            let name = parts.next().unwrap().trim().to_string();
            let expect = parts.next().unwrap().trim();
            let expect = match expect.split_once(' ') {
                Some(("int", v)) => Expect::Int(v.parse().unwrap()),
                Some(("str", v)) => Expect::Str(v.trim_matches('"').to_string()),
                _ => panic!("{name}: cannot read the expected value from {expect:?}"),
            };
            sections.push(Section {
                name,
                expect,
                broken,
                ruby: String::new(),
            });
        } else if let Some(section) = sections.last_mut() {
            section.ruby.push_str(line);
            section.ruby.push('\n');
        }
    }
    sections
}

fn evaluate(section: &Section) -> Result<String, String> {
    let ruby: &'static str = Box::leak(section.ruby.clone().into_boxed_str());
    let binary = mrbc_compile("coverage", ruby);
    let mut rite = mrubyedge::rite::load(&binary).map_err(|e| format!("{e:?}"))?;
    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    let result = vm.run().map_err(|e| format!("{e:?}"))?;
    match &section.expect {
        Expect::Int(_) => TryInto::<i64>::try_into(result.as_ref())
            .map(|v| v.to_string())
            .map_err(|e| format!("{e:?}")),
        Expect::Str(_) => {
            TryInto::<String>::try_into(result.as_ref()).map_err(|e| format!("{e:?}"))
        }
    }
}

fn wanted(section: &Section) -> String {
    match &section.expect {
        Expect::Int(v) => v.to_string(),
        Expect::Str(v) => v.clone(),
    }
}

fn find<'a>(sections: &'a [Section], name: &str) -> &'a Section {
    sections
        .iter()
        .find(|s| s.name == name)
        .unwrap_or_else(|| panic!("coverage/language.rb has no section named {name}"))
}

fn assert_still_broken(name: &str) {
    let body = script();
    let sections = parse(&body);
    let section = find(&sections, name);
    assert!(section.broken, "section {name} is not marked broken");
    let got = evaluate(section);

    // Assert
    assert_ne!(
        got.as_deref().ok(),
        Some(wanted(section).as_str()),
        "section {name} gives the right answer now; move it back to #section and drop its ignored test"
    );
}

/// Every section the script marks broken, one ignored test each, so a run
/// reports them the way it reports any skipped test. `cargo test -- --ignored`
/// checks they are all still broken.
const BROKEN: &[&str] = &[
    "case_when_class",
    "equality_override",
    "array_inject",
    "array_reverse",
    "comparable_between",
    "post_argument",
    "masgn_with_post",
    "nested_constant_path",
    "logical_not_on_false",
    "next_inside_ensure",
];

#[test]
fn every_section_evaluates_to_the_value_ruby_gives_test() {
    let body = script();
    for section in parse(&body).iter().filter(|s| !s.broken) {
        let got = evaluate(section)
            .unwrap_or_else(|e| panic!("section {} stopped with {e}", section.name));

        // Assert
        assert_eq!(got, wanted(section), "section {}", section.name);
    }
}

#[test]
fn the_ignored_tests_name_every_broken_section_test() {
    let body = script();
    let marked: std::collections::BTreeSet<String> = parse(&body)
        .iter()
        .filter(|s| s.broken)
        .map(|s| s.name.clone())
        .collect();
    let listed: std::collections::BTreeSet<String> =
        BROKEN.iter().map(|name| name.to_string()).collect();

    // Assert
    assert_eq!(marked, listed);
}

#[test]
#[ignore = "case x when SomeClass compares the class with itself"]
fn broken_case_when_class_test() {
    assert_still_broken("case_when_class");
}

#[test]
#[ignore = "== compares identity and does not reach a user-defined =="]
fn broken_equality_override_test() {
    assert_still_broken("equality_override");
}

#[test]
#[ignore = "Array#inject is not defined"]
fn broken_array_inject_test() {
    assert_still_broken("array_inject");
}

#[test]
#[ignore = "Array#reverse is not defined"]
fn broken_array_reverse_test() {
    assert_still_broken("array_reverse");
}

#[test]
#[ignore = "Integer#between? is not defined"]
fn broken_comparable_between_test() {
    assert_still_broken("comparable_between");
}

#[test]
#[ignore = "a parameter after a rest parameter is left unassigned"]
fn broken_post_argument_test() {
    assert_still_broken("post_argument");
}

#[test]
#[ignore = "APOST with three operands is not implemented"]
fn broken_masgn_with_post_test() {
    assert_still_broken("masgn_with_post");
}

#[test]
#[ignore = "a constant two namespaces deep is not found"]
fn broken_nested_constant_path_test() {
    assert_still_broken("nested_constant_path");
}

#[test]
#[ignore = "FalseClass has no !"]
fn broken_logical_not_on_false_test() {
    assert_still_broken("logical_not_on_false");
}

#[test]
#[ignore = "next inside an ensure skips the ensure body"]
fn broken_next_inside_ensure_test() {
    assert_still_broken("next_inside_ensure");
}
