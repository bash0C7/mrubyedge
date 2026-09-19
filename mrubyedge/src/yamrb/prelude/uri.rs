use std::rc::Rc;

use crate::{
    Error,
    yamrb::{
        helpers::mrb_define_singleton_cmethod,
        value::{RHashMap, RObject, RSym, RValue},
        vm::VM,
    },
};

// `URI.encode_www_form(text: text)` — a bare keyword call with no
// positional argument at all — has no ENTER instruction (this is a cfunc)
// to fold `text: text` into a trailing Hash the way a Ruby callee's ENTER
// does when it declares no keyword parameter. Reading the call's kwargs
// back out when `args` is empty keeps that call shape working, the same
// way Hash#merge and Data.new already do.
fn bare_kwargs_as_hash(vm: &VM) -> Option<Rc<RObject>> {
    let kwargs = vm.get_kwargs()?;
    if kwargs.is_empty() {
        return None;
    }
    let mut map = RHashMap::default();
    for (name, value) in kwargs.into_iter() {
        let key = RObject::symbol(RSym::new(name)).to_refcount_assigned();
        if let Ok(hash_key) = key.as_hash_key() {
            map.insert(hash_key, (key, value));
        }
    }
    Some(RObject::hash(map).to_refcount_assigned())
}

pub(crate) fn initialize_uri(vm: &mut VM) {
    vm.define_module("URI", None);
    let uri = vm.get_const_by_name("URI").expect("URI module not found");

    mrb_define_singleton_cmethod(
        vm,
        uri.clone(),
        "encode_www_form",
        Box::new(mrb_uri_encode_www_form),
    );
    mrb_define_singleton_cmethod(
        vm,
        uri.clone(),
        "encode_www_form_component",
        Box::new(mrb_uri_encode_www_form_component),
    );
    mrb_define_singleton_cmethod(
        vm,
        uri,
        "decode_www_form_component",
        Box::new(mrb_uri_decode_www_form_component),
    );
}

// application/x-www-form-urlencoded: space -> `+`, `*-._` and alphanumerics stay, else `%XX`.
fn encode_component(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b' ' => out.push('+'),
            b'*' | b'-' | b'.' | b'_' => out.push(byte as char),
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => out.push(byte as char),
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}

// Not `u8::from_str_radix`: that reads a leading `+` as a sign, so `%+4` would
// decode to a byte instead of staying a literal percent sign.
fn hex_digit(byte: u8) -> Option<u8> {
    (byte as char).to_digit(16).map(|digit| digit as u8)
}

// Lenient where Ruby raises: a `%` that does not begin a complete hex pair stays
// a literal `%` and the walk moves on one byte, so `"100%満足"` decodes to itself
// where Ruby answers ArgumentError. Ordinary prose carries a bare `%`, and an
// embedded VM has nowhere useful to raise from a form decoder.
fn decode_component(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                match (hex_digit(bytes[i + 1]), hex_digit(bytes[i + 2])) {
                    (Some(high), Some(low)) => {
                        out.push(high << 4 | low);
                        i += 3;
                    }
                    _ => {
                        out.push(b'%');
                        i += 1;
                    }
                }
            }
            byte => {
                out.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

// The string a value contributes to the form; nil writes the key alone, with no `=`.
fn form_value(value: &Rc<RObject>) -> Result<Option<String>, Error> {
    match &value.value {
        RValue::Nil => Ok(None),
        RValue::String(_, _) => Ok(Some(value.as_ref().try_into()?)),
        RValue::Integer(i) => Ok(Some(i.to_string())),
        RValue::Float(f) => Ok(Some(f.to_string())),
        RValue::Bool(b) => Ok(Some(b.to_string())),
        RValue::Symbol(s) => Ok(Some(s.name.clone())),
        _ => Err(Error::ArgumentError(
            "URI.encode_www_form: value must be a String, Integer, Float, Symbol, true/false or nil"
                .to_string(),
        )),
    }
}

fn push_pair(out: &mut Vec<String>, key: &Rc<RObject>, value: &Rc<RObject>) -> Result<(), Error> {
    // A key converts the same way a value does; nil writes an empty key, as
    // Ruby's `URI.encode_www_form([[nil, 1]])` does.
    let key = encode_component(&form_value(key)?.unwrap_or_default());

    // An Array value repeats the key, the way Ruby expands `[["a", [1, 2]]]`.
    if let RValue::Array(items) = &value.value {
        for item in items.borrow().iter() {
            match form_value(item)? {
                Some(v) => out.push(format!("{}={}", key, encode_component(&v))),
                None => out.push(key.clone()),
            }
        }
        return Ok(());
    }

    match form_value(value)? {
        Some(v) => out.push(format!("{}={}", key, encode_component(&v))),
        None => out.push(key),
    }
    Ok(())
}

fn mrb_uri_encode_www_form(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let kwarg_hash = args.is_empty().then(|| bare_kwargs_as_hash(vm)).flatten();
    let enumerable = kwarg_hash
        .as_ref()
        .or_else(|| args.first())
        .ok_or_else(|| {
            Error::ArgumentError("URI.encode_www_form expects an argument".to_string())
        })?;

    let mut parts: Vec<String> = Vec::new();
    match &enumerable.value {
        // Ruby walks a Hash in insertion order; this VM's Hash does not keep one,
        // so the order of the pairs a Hash produces here is unspecified.
        RValue::Hash(hash) => {
            for (key, value) in hash.borrow().values() {
                push_pair(&mut parts, key, value)?;
            }
        }
        RValue::Array(pairs) => {
            for pair in pairs.borrow().iter() {
                let RValue::Array(pair) = &pair.value else {
                    return Err(Error::ArgumentError(
                        "URI.encode_www_form expects [key, value] pairs".to_string(),
                    ));
                };
                let pair = pair.borrow();
                let key = pair.first().ok_or_else(|| {
                    Error::ArgumentError("URI.encode_www_form: empty pair".to_string())
                })?;
                let nil = RObject::nil().to_refcount_assigned();
                let value = pair.get(1).unwrap_or(&nil);
                push_pair(&mut parts, key, value)?;
            }
        }
        _ => {
            return Err(Error::ArgumentError(
                "URI.encode_www_form expects a Hash or an Array of pairs".to_string(),
            ));
        }
    }

    Ok(Rc::new(RObject::string(parts.join("&"))))
}

fn mrb_uri_encode_www_form_component(
    _vm: &mut VM,
    args: &[Rc<RObject>],
) -> Result<Rc<RObject>, Error> {
    let value = args.first().ok_or_else(|| {
        Error::ArgumentError("URI.encode_www_form_component expects an argument".to_string())
    })?;
    let value = form_value(value)?.unwrap_or_default();
    Ok(Rc::new(RObject::string(encode_component(&value))))
}

fn mrb_uri_decode_www_form_component(
    _vm: &mut VM,
    args: &[Rc<RObject>],
) -> Result<Rc<RObject>, Error> {
    let value = args.first().ok_or_else(|| {
        Error::ArgumentError("URI.decode_www_form_component expects an argument".to_string())
    })?;
    if !matches!(value.value, RValue::String(_, _)) {
        return Err(Error::ArgumentError(
            "URI.decode_www_form_component expects a String".to_string(),
        ));
    }
    let value: String = value.as_ref().try_into()?;
    Ok(Rc::new(RObject::string(decode_component(&value))))
}
