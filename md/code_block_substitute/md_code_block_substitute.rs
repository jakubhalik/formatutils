use std::collections::HashMap;
use std::env;
use std::io::{self, Read};

fn interpret_escapes(s: &str) -> String {
    let replaced: String = s.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r");
    replaced
}

fn md_code_block_substitute(
    input: &str, 
    rules: &HashMap<String, String>, 
    else_rule: Option<String>
) -> String {

    type Indexing = usize;

    let mut output: String = String::new();
    let mut current_index: Indexing = 0;
    const TICKS : &str = "```";

    while let Some(triple_tick_position) = input[current_index..].find(TICKS) {

        let absolute_position: Indexing = current_index + triple_tick_position;
        output.push_str(&input[current_index..absolute_position]);

        let mut after_ticks_index: Indexing = absolute_position + TICKS.len();
        let remainder: &str = &input[after_ticks_index..];

        let mut matched: bool = false;

        for (rule_key, rule_value) in rules {
            if remainder.starts_with(rule_key) {
                let sub: String = rule_value.replace("{}", &format!("{}{}", TICKS, rule_key));
                output.push_str(&sub);
                after_ticks_index += rule_key.len();
                matched = true;
                break;
            }

        }

        if !matched {
            if let Some(otherwise) = &else_rule {
                let sub: String = otherwise.replace("{}", &format!("{}", TICKS));
                output.push_str(&sub);
            } else {
                output.push_str(TICKS);
            }
        }

        current_index = after_ticks_index;
    }

    output.push_str(&input[current_index..]);
    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rules: HashMap<String, String> = HashMap::new();
    let mut else_rule: Option<String> = None;

    for arg in &args[1..] {
        if arg.starts_with("if_") {
            let parts: Vec<&str> = arg[3..].split('=').collect();
            if parts.len() == 2 {
                rules.insert(parts[0].to_string(), interpret_escapes(parts[1]));
            }
        } else if arg.starts_with("else=") {
            else_rule = Some(interpret_escapes(&arg[5..]));
        }
    }

    let stdin_data: String = io::stdin().lock().bytes()
        .filter_map(Result::ok)
        .map(|b| b as char)
        .collect();

    let result: String = md_code_block_substitute(&stdin_data, &rules, else_rule);

    println!("{}", result);
}
