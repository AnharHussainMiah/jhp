use quick_js::Context;

pub fn compile(text: &str) -> String {
    let mut is_text_mode: bool = true;
    let mut buffer = String::new();

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

    let lines: Vec<&str> = text.split("\n").collect();

    for line in lines {
        if line.contains("<?jhp") && !line.contains("?>") {
            let transformed = line.replace("<?jhp", "')");
            buffer.push_str(&format!("console.log('{}\n", transformed));
            is_text_mode = false;
            continue;
        }

        if !line.contains("<?jhp") && line.contains("?>") {
            let transformed = line.replace("?>", "");
            buffer.push_str(&format!("{}\n", transformed));
            is_text_mode = true;
            continue;
        }

        if line != "<?jhp" && line != "?>" {
            if is_text_mode {
                if line.contains("<?jhp") && line.contains("?>") {
                    let tokens: Vec<char> = line.chars().collect();
                    let mut index = 0;
                    let mut is_text = true;

                    if !line.starts_with("<?jhp") {
                        buffer.push_str(&format!("console.log('"));
                    }

                    while index < tokens.len() {
                        if tokens[index..].starts_with(&['<', '?', 'j', 'h', 'p']) {
                            is_text = false;
                            buffer.push_str(&format!("');"));
                            buffer.push_str(&tokens[index].to_string());
                            index += 4;
                            continue;
                        }

                        if tokens[index..].starts_with(&['?', '>']) {
                            is_text = true;
                            buffer.push_str(&format!("console.log('"));
                            buffer.push_str(&tokens[index].to_string());
                            index += 2;
                            continue;
                        }

                        buffer.push_str(&tokens[index].to_string());
                        index += 1;
                    }

                    if is_text {
                        buffer.push_str(&format!("');"));
                    }
                } else {
                    buffer.push_str(&format!("console.log('{}');\n", line));
                }
            } else {
                buffer.push_str(&format!("{}\n", line));
            }
        }
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
