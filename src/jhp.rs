use quick_js::Context;

use crate::host;
use quick_js::JsValue;
use std::collections::HashMap;

#[derive(serde::Serialize)]
pub struct JHPResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl TryFrom<JsValue> for JHPResponse {
    type Error = String;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let mut object = match value {
            JsValue::Object(object) => object,
            _ => return Err("Expected response object".into()),
        };

        let status = match object.remove("status") {
            Some(JsValue::Int(n)) => u16::try_from(n).map_err(|_| "Invalid status code")?,
            _ => return Err("Missing or invalid status".into()),
        };

        let headers = match object.remove("headers") {
            Some(JsValue::Object(headers)) => headers
                .into_iter()
                .map(|(key, value)| match value {
                    JsValue::String(value) => Ok((key, value)),
                    _ => Err(format!("Header '{key}' must be a string")),
                })
                .collect::<Result<HashMap<String, String>, String>>()?,
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

pub fn js(jhp: String) -> Result<JHPResponse, Box<dyn std::error::Error>> {
    let ctx = Context::new()?;

    ctx.add_callback("LOG", |m: String| {
        println!("STDOUT| {}", m);
        m
    })?;

    /*
        File I/O

        Some perl inspiration here:

         * "slurp" : read a file given a file name e.g SLURP("foo.txt")
         * "spew"  : write a new file e.g SPEW("bar.txt", "some data here!")
         * "pour"  : append to an existing file, but create a new one if one does not exist e.g POUR("baz.txt", "another line")

    */

    ctx.add_callback("SLURP", |m: String| host::slurp(m))?;
    ctx.add_callback("SPEW", |f: String, c: String| host::spew(f, c))?;
    ctx.add_callback("POUR", |f: String, c: String| host::pour(f, c))?;

    /*
        Http Get + Post

        * Fetch (url, headers): empty string on failure, or json string, need to parse into json
        * Post (url, headers, body): same as above

    */

    ctx.add_callback("FETCH", |u: String, h: String| host::fetch(u, h))?;
    ctx.add_callback("POST", |u: String, h: String, b: String| {
        host::post(u, h, b)
    })?;

    let result = ctx.eval(&jhp)?;
    let response = JHPResponse::try_from(result)?;

    Ok(response)
}
