use std::collections::HashSet;

use crate::host::slurp;

#[derive(Debug)]
struct Include {
    start_position: usize,
    end_position: usize,
    name: String,
    children: Vec<Include>,
}

pub fn resolve(f: &str) -> String {
    let resolved = resolve_include_imports(
        &slurp(f.to_string()),
        &generate_includes_graph(&f, &mut HashSet::new()),
    );
    resolved
}

fn resolve_include_imports(content: &str, includes: &[Include]) -> String {
    let mut output = String::new();
    let mut cursor = 0;

    for include in includes {
        output.push_str(&content[cursor..include.start_position]);
        let child_content = slurp(include.name.clone());
        let expanded_content = resolve_include_imports(&child_content, &include.children);
        output.push_str(&expanded_content);
        cursor = include.end_position;
    }
    output.push_str(&content[cursor..]);

    output
}

fn generate_includes_graph(filename: &str, current_path: &mut HashSet<String>) -> Vec<Include> {
    if !current_path.insert(filename.to_string()) {
        return Vec::new();
    }

    let mut includes = extract_includes(filename);

    includes.retain_mut(|include| {
        if current_path.contains(&include.name) {
            return false;
        }
        include.children = generate_includes_graph(&include.name, current_path);
        true
    });

    current_path.remove(filename);
    includes
}

fn extract_includes(f: &str) -> Vec<Include> {
    // println!("DEBUG| attempting to extract '{}'", f);

    let content = slurp(f.to_string());
 
    let chars: Vec<char> = content.chars().collect();

    let mut include_stack: Vec<Include> = Vec::new();
    let mut is_include = false;
    let mut cursor: usize = 0;

    let mut include_start = 0;
    let mut include_end: usize;
    let mut include_name = String::new();

    while cursor < chars.len() {
        let peek_ch1 = chars.get(cursor + 0).copied(); // <
        let peek_ch2 = chars.get(cursor + 1).copied(); // #

        if peek_ch1 == Some('<') && peek_ch2 == Some('#') {
            // skip
            cursor += 2;
            include_start = cursor - 2;
            is_include = true;
        }

        if peek_ch1 == Some('#') && peek_ch2 == Some('>') {
            cursor += 2;
            include_end = cursor;
            is_include = false;

            include_stack.push(Include {
                start_position: include_start,
                end_position: include_end,
                name: format!("public/{}", include_name.trim().to_string()),
                children: Vec::new(),
            });

            include_name = String::new();
        }

        if is_include {
            include_name.push(chars[cursor]);
            cursor += 1;
            continue;
        }

        if cursor < chars.len() {
            cursor += 1;
        }
    }
    include_stack
}
