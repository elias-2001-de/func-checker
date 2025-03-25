mod lexer;

use bool_algebra::{get_names, parse, validate_func};
use js_sys::{Array, Object, Reflect};
use lexer::lex;
use wasm_bindgen::prelude::*;

fn js2_vec(funcs: &JsValue) -> Result<Vec<String>, JsValue> {
    let mut result = Vec::new();

    let iterator = js_sys::try_iter(funcs)?.ok_or_else(|| "need to pass iterable JS values!")?;

    for x in iterator {
        let x = x?;
        if let Some(x) = x.as_string() {
            result.push(x)
        } else {
            return Err(x + JsValue::from_str(" is not a string"));
        }
    }

    Ok(result)
}

#[wasm_bindgen]
pub fn get_table(func: String) -> JsValue {
    let token = match lex(func.as_str(), &vec!['&', '|', '^', '!', '>', '<', '=']) {
        Ok(val) => val,
        Err(err) => return JsValue::from(&err),
    };
    if let Err(err) = validate_func(&token) {
        return JsValue::from(&err);
    }

    let names = get_names(&token);
    let names_js = Array::new();
    names.iter().for_each(|e| {
        names_js.push(&JsValue::from(e));
    });

    let table = match parse(&token) {
        Ok(val) => val,
        Err(err) => return JsValue::from(&err),
    };

    let table_js = Array::new();
    table.iter().for_each(|e| {
        table_js.push(&JsValue::from(*e));
    });

    let obj = Object::new();
    Reflect::set(&obj, &JsValue::from_str("names"), &names_js).unwrap();
    Reflect::set(&obj, &JsValue::from_str("table"), &table_js).unwrap();

    return obj.into();
}

#[wasm_bindgen]
pub fn check_funcs(funcs: &JsValue) -> Vec<JsValue> {
    let mut result = Vec::new();
    let mut tables = Vec::new();
    let mut names = Vec::new();

    let funcs = match js2_vec(funcs) {
        Err(err) => {
            result.push(JsValue::from_str("Error: ") + err);
            return result;
        }
        Ok(f) => f,
    };

    for f in funcs {
        let token = match lex(f.as_str(), &vec!['&', '|', '^', '!', '>', '<', '=']) {
            Err(err) => {
                result.push(JsValue::from_str(&format!("Error: {}", err)));
                tables.push(Vec::new());
                names.push(Vec::new());
                continue;
            }
            Ok(t) => t,
        };

        if let Err(err) = validate_func(&token) {
            result.push(JsValue::from_str(&format!("Error: {}", err)));
            tables.push(Vec::new());
            names.push(Vec::new());
            continue;
        }

        names.push(get_names(&token));

        match parse(&token) {
            Err(err) => {
                result.push(JsValue::from_str(&format!("Error: {}", err)));
                tables.push(Vec::new());
            }
            Ok(table) => {
                result.push(JsValue::NULL);
                tables.push(table);
            }
        }
    }

    assert_eq!(result.len(), tables.len());
    assert_eq!(result.len(), names.len());

    for i in 1..tables.len() {
        if result[i].is_null() {
            result[i] = if names[0] != names[i] {
                JsValue::from_str("Error: no matching names")
            } else if tables[0] == tables[i] {
                JsValue::from_str("Correct")
            } else {
                JsValue::from_str("Wrong")
            };
        }
    }

    result
}
