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
fn every_section_marked_broken_is_still_broken_test() {
    let body = script();
    for section in parse(&body).iter().filter(|s| s.broken) {
        let got = evaluate(section);

        // Assert
        assert_ne!(
            got.as_deref().ok(),
            Some(wanted(section).as_str()),
            "section {} gives the right answer now; move it back to #section",
            section.name
        );
    }
}
