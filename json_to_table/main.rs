use serde_json::Value;
use std::collections::BTreeMap;

fn val_to_string(val: &Value) -> String {
    match val {
        Value::String(string_val) => string_val.clone(),
        Value::Number(num_val) => num_val.to_string(),
        Value::Bool(bool_val) => bool_val.to_string(),
        Value::Null => "".to_string(),
        Value::Object(map_val) => {
            let pairs: Vec<String> = map_val
                .iter()
                .map(|(key, inner_val)| format!("{}:{}", key, val_to_string(inner_val)))
                .collect();
            pairs.join(" | ")
        }
        Value::Array(arr_val) => {
            let items: Vec<String> = arr_val.iter().map(|inner_val| val_to_string(inner_val)).collect();
            items.join(", ")
        }
    }
}

fn find_largest_object_array(val: &Value) -> Option<Vec<Value>> {
    let mut best: Option<Vec<Value>> = None;

    match val {
        Value::Array(arr_val) => {
            let all_objects = arr_val.iter().all(|item| item.is_object());
            if all_objects && !arr_val.is_empty() {
                let current_len = arr_val.len();
                let is_better = best.as_ref().map_or(true, |existing| current_len > existing.len());
                if is_better {
                    best = Some(arr_val.clone());
                }
            }
            for item in arr_val {
                if let Some(found) = find_largest_object_array(item) {
                    let is_better = best.as_ref().map_or(true, |existing| found.len() > existing.len());
                    if is_better {
                        best = Some(found);
                    }
                }
            }
        }
        Value::Object(map_val) => {
            for (_key, inner_val) in map_val {
                if let Some(found) = find_largest_object_array(inner_val) {
                    let is_better = best.as_ref().map_or(true, |existing| found.len() > existing.len());
                    if is_better {
                        best = Some(found);
                    }
                }
            }
        }
        _ => {}
    }

    best
}

fn find_header_object(root: &Value, table_keys: &[String]) -> Option<BTreeMap<String, String>> {
    match root {
        Value::Object(map_val) => {
            for (_key, inner_val) in map_val {
                if let Value::Object(candidate_map) = inner_val {
                    let candidate_keys: Vec<String> = candidate_map.keys().cloned().collect();
                    let mut sorted_candidate = candidate_keys.clone();
                    sorted_candidate.sort();
                    let mut sorted_table = table_keys.to_vec();
                    sorted_table.sort();
                    if sorted_candidate == sorted_table {
                        let all_strings = candidate_map.values().all(|val| val.is_string());
                        if all_strings {
                            let result: BTreeMap<String, String> = candidate_map
                                .iter()
                                .map(|(key, val)| (key.clone(), val.as_str().unwrap_or("").to_string()))
                                .collect();
                            return Some(result);
                        }
                    }
                }
                if let Some(found) = find_header_object(inner_val, table_keys) {
                    return Some(found);
                }
            }
        }
        Value::Array(arr_val) => {
            for item in arr_val {
                if let Some(found) = find_header_object(item, table_keys) {
                    return Some(found);
                }
            }
        }
        _ => {}
    }
    None
}

fn pad_right(content: &str, width: usize) -> String {
    let padding = width.saturating_sub(content.len());
    format!("{}{}", content, " ".repeat(padding))
}

fn build_table(rows: &[Value]) -> Result<String, String> {
    if rows.is_empty() {
        return Err("No rows found".to_string());
    }

    let first_obj = rows[0]
        .as_object()
        .ok_or("First row is not an object")?;

    let columns: Vec<String> = first_obj.keys().cloned().collect();

    let formatted_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            columns
                .iter()
                .map(|col| {
                    let cell_val = row.get(col).unwrap_or(&Value::Null);
                    val_to_string(cell_val)
                })
                .collect()
        })
        .collect();

    let column_widths: Vec<usize> = columns
        .iter()
        .enumerate()
        .map(|(col_idx, header)| {
            let max_cell = formatted_rows
                .iter()
                .map(|row| row.get(col_idx).map(|cell| cell.len()).unwrap_or(0))
                .max()
                .unwrap_or(0);
            std::cmp::max(header.len(), max_cell)
        })
        .collect();

    let mut lines: Vec<String> = Vec::new();

    let header_cells: Vec<String> = columns
        .iter()
        .enumerate()
        .map(|(col_idx, header)| pad_right(header, column_widths[col_idx]))
        .collect();
    lines.push(format!("| {} |", header_cells.join(" | ")));

    let separator_segments: Vec<String> = column_widths
        .iter()
        .map(|width| "-".repeat(*width))
        .collect();
    lines.push(format!("|-{}-|", separator_segments.join("-|-")));

    for row in &formatted_rows {
        let padded_cells: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(col_idx, cell)| pad_right(cell, column_widths[col_idx]))
            .collect();
        lines.push(format!("| {} |", padded_cells.join(" | ")));
    }

    Ok(lines.join("\n"))
}

fn build_table_with_header_descriptions(
    rows: &[Value],
    header_map: &BTreeMap<String, String>,
) -> Result<String, String> {
    if rows.is_empty() {
        return Err("No rows found".to_string());
    }

    let first_obj = rows[0]
        .as_object()
        .ok_or("First row is not an object")?;

    let columns: Vec<String> = first_obj.keys().cloned().collect();

    let display_headers: Vec<String> = columns
        .iter()
        .map(|col| {
            header_map
                .get(col)
                .cloned()
                .unwrap_or_else(|| col.clone())
        })
        .collect();

    let formatted_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            columns
                .iter()
                .map(|col| {
                    let cell_val = row.get(col).unwrap_or(&Value::Null);
                    val_to_string(cell_val)
                })
                .collect()
        })
        .collect();

    let column_widths: Vec<usize> = display_headers
        .iter()
        .enumerate()
        .map(|(col_idx, header)| {
            let max_cell = formatted_rows
                .iter()
                .map(|row| row.get(col_idx).map(|cell| cell.len()).unwrap_or(0))
                .max()
                .unwrap_or(0);
            std::cmp::max(header.len(), max_cell)
        })
        .collect();

    let mut lines: Vec<String> = Vec::new();

    let header_cells: Vec<String> = display_headers
        .iter()
        .enumerate()
        .map(|(col_idx, header)| pad_right(header, column_widths[col_idx]))
        .collect();
    lines.push(format!("| {} |", header_cells.join(" | ")));

    let separator_segments: Vec<String> = column_widths
        .iter()
        .map(|width| "-".repeat(*width))
        .collect();
    lines.push(format!("|-{}-|", separator_segments.join("-|-")));

    for row in &formatted_rows {
        let padded_cells: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(col_idx, cell)| pad_right(cell, column_widths[col_idx]))
            .collect();
        lines.push(format!("| {} |", padded_cells.join(" | ")));
    }

    Ok(lines.join("\n"))
}

fn run(input: &str) -> Result<String, String> {
    let parsed: Value =
        serde_json::from_str(input).map_err(|err| format!("Invalid JSON: {}", err))?;

    let table_rows =
        find_largest_object_array(&parsed).ok_or("No array of objects found in JSON")?;

    let first_obj = table_rows[0]
        .as_object()
        .ok_or("First row is not an object")?;
    let table_keys: Vec<String> = first_obj.keys().cloned().collect();

    if let Some(header_map) = find_header_object(&parsed, &table_keys) {
        build_table_with_header_descriptions(&table_rows, &header_map)
    } else {
        build_table(&table_rows)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input = if args.len() > 1 {
        std::fs::read_to_string(&args[1])
            .map_err(|err| format!("Failed to read file '{}': {}", &args[1], err))
    } else {
        std::io::read_to_string(std::io::stdin())
            .map_err(|err| format!("Failed to read stdin: {}", err))
    };

    match input {
        Ok(content) => match run(&content) {
            Ok(table) => println!("{}", table),
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
