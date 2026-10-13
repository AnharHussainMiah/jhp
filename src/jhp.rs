use quick_js::Context;

pub fn compile(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut is_code_mode: bool = false;
    let mut buffer = String::new();
    let mut cursor: usize = 0;

    buffer.push_str(
        r#"
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
    globalThis.__consoleOutput;
    "#,
    );

    buffer.to_string()
}

pub fn js(jhp: String) -> Result<String, Box<dyn std::error::Error>> {
    let ctx = Context::new()?;
    let result = ctx.eval_as::<String>(&jhp)?;
    // println!("{result}");
    Ok(result)
}
