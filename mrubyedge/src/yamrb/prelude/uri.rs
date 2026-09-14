use std::rc::Rc;

use crate::{
    Error,
    yamrb::{
        helpers::mrb_define_singleton_cmethod,
        value::{RObject, RValue},
        vm::VM,
    },
};

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
                let hex = &value[i + 1..i + 3];
                match u8::from_str_radix(hex, 16) {
                    Ok(byte) => {
                        out.push(byte);
                        i += 3;
                    }
                    Err(_) => {
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
    let key = match &key.value {
        RValue::Symbol(s) => s.name.clone(),
        RValue::Integer(i) => i.to_string(),
        _ => key.as_ref().try_into()?,
    };
    let key = encode_component(&key);

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

fn mrb_uri_encode_www_form(_vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let enumerable = args.first().ok_or_else(|| {
        Error::ArgumentError("URI.encode_www_form expects an argument".to_string())
    })?;

    let mut parts: Vec<String> = Vec::new();
    match &enumerable.value {
        RValue::Hash(hash) => {
            for (_, (key, value)) in hash.borrow().iter() {
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
    let value: String = args
        .first()
        .ok_or_else(|| {
            Error::ArgumentError("URI.decode_www_form_component expects an argument".to_string())
        })?
        .as_ref()
        .try_into()?;
    Ok(Rc::new(RObject::string(decode_component(&value))))
}
