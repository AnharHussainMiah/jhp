use quick_js::Context;

use std::collections::HashMap;
use quick_js::JsValue;

struct JHPResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl TryFrom<JsValue> for JHPResponse {
    type Error = String;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let mut object = match value {
            JsValue::Object(object) => object,
            _ => return Err("Expected response object".into()),
        };

        let status = match object.remove("status") {
            Some(JsValue::Int(n)) => {
                u16::try_from(n).map_err(|_| "Invalid status code")?
            }
            _ => return Err("Missing or invalid status".into()),
        };

        let headers = match object.remove("headers") {
            Some(JsValue::Object(headers)) => {
                headers
                    .into_iter()
                    .map(|(key, value)| match value {
                        JsValue::String(value) => Ok((key, value)),
                        _ => Err(format!("Header '{key}' must be a string")),
                    })
                    .collect::<Result<HashMap<String, String>, String>>()?
            }
            _ => return Err("Missing or invalid headers".into()),
        };

        let body = match object.remove("body") {
            Some(JsValue::String(body)) => body,
            _ => return Err("Missing or invalid body".into()),
        };

        Ok(JHPResponse {
            status,
            headers,
            body,
        })
    }
}

pub fn compile(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut is_code_mode: bool = false;
    let mut buffer = String::new();
    let mut cursor: usize = 0;

    buffer.push_str(
        r#"
    RESPONSE = {
        status: 200,
            headers: {
                "content-type": "text/html; charset=utf-8",
                "cache-control": "no-cache"
            },
            body: ''
    };

    globalThis.__consoleOutput = "";

    globalThis.console = {
        log: function(...args) {
            globalThis.__consoleOutput += args.join(" ") + "\n";
        }
    };
    "#,
    );

    buffer.push('c');
    buffer.push('o');
    buffer.push('n');
    buffer.push('s');
    buffer.push('o');
    buffer.push('l');
    buffer.push('e');
    buffer.push('.');
    buffer.push('l');
    buffer.push('o');
    buffer.push('g');
    buffer.push('(');
    buffer.push('"');

    while cursor < chars.len() {
        let peek_ch1 = chars.get(cursor + 0).copied(); // <
        let peek_ch2 = chars.get(cursor + 1).copied(); // ?
        let peek_ch3 = chars.get(cursor + 2).copied(); // j
        let peek_ch4 = chars.get(cursor + 3).copied(); // h
        let peek_ch5 = chars.get(cursor + 4).copied(); // p

        if peek_ch1 == Some('<')
            && peek_ch2 == Some('?')
            && peek_ch3 == Some('j')
            && peek_ch4 == Some('h')
            && peek_ch5 == Some('p')
        {
            is_code_mode = true;
            cursor += 5;

            buffer.push('"');
            buffer.push(')');
            buffer.push(';');
        }

        if peek_ch1 == Some('?') && peek_ch2 == Some('>') {
            is_code_mode = false;
            cursor += 2;

            buffer.push('\n');
            buffer.push('c');
            buffer.push('o');
            buffer.push('n');
            buffer.push('s');
            buffer.push('o');
            buffer.push('l');
            buffer.push('e');
            buffer.push('.');
            buffer.push('l');
            buffer.push('o');
            buffer.push('g');
            buffer.push('(');
            buffer.push('"');
        }

        if !is_code_mode {
            if peek_ch1 == Some('\n') {
                // inject our closing e.g
                // ")\n
                // console.log("

                buffer.push('"');
                buffer.push(')');
                buffer.push(';');
                buffer.push('\n');
                buffer.push('c');
                buffer.push('o');
                buffer.push('n');
                buffer.push('s');
                buffer.push('o');
                buffer.push('l');
                buffer.push('e');
                buffer.push('.');
                buffer.push('l');
                buffer.push('o');
                buffer.push('g');
                buffer.push('(');
                buffer.push('"');
            }

            let peek_ch1 = chars.get(cursor + 0).copied(); // cursor has been moved, yeah yeah shadowing is bad I know ;)

            if cursor < chars.len() && peek_ch1 != Some('\n') {
                buffer.push(chars[cursor]);
            }
        } else {
            if cursor < chars.len() {
                buffer.push(chars[cursor]);
            }
        }

        cursor += 1;
    }

    if !is_code_mode {
        buffer.push('"');
        buffer.push(')');
        buffer.push(';');
        buffer.push('\n');
    }

    buffer.push_str(
        r#"
    RESPONSE.body = globalThis.__consoleOutput;
    RESPONSE;
    "#,
    );

    buffer.to_string()
}

pub fn js(jhp: String) -> Result<String, Box<dyn std::error::Error>> {
    let ctx = Context::new()?;

    // lets test out some host functions:

    ctx.add_callback("stdout", |m: String| {
        println!("STDOUT -> {}", m);
        m
    })?;

    ctx.add_callback("httpGet", |m: String| {
        format!("you said -> {}",m)
    })?;

    let result = ctx.eval(&jhp)?;
    let response = JHPResponse::try_from(result)?;

    Ok(response.body)
}
