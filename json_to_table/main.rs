use serde_json::Value;
use std::collections::BTreeMap;

fn val_to_string(val: &Value) -> String {
    match val {
        Value::String(string_val) => string_val.clone(),
        Value::Number(num_val) => num_val.to_string(),
        Value::Bool(bool_val) => bool_val.to_string(),
        Value::Null => "".to_string(),
        Value::Array(arr_val) => {
            let all_objects = arr_val.iter().all(|item| item.is_object()) && !arr_val.is_empty();
            if all_objects {
                let first_obj = arr_val[0].as_object().unwrap();
                let keys: Vec<String> = first_obj.keys().cloned().collect();

                let all_rows: Vec<Vec<String>> = arr_val
                    .iter()
                    .map(|item| {
                        let obj = item.as_object().unwrap();
                        keys.iter()
                            .map(|key| val_to_string(obj.get(key).unwrap_or(&Value::Null)))
                            .collect()
                    })
                    .collect();

                let col_widths: Vec<usize> = keys
                    .iter()
                    .enumerate()
                    .map(|(col_idx, key)| {
                        let max_cell = all_rows
                            .iter()
                            .map(|row| row.get(col_idx).map(|cell| cell.len()).unwrap_or(0))
                            .max()
                            .unwrap_or(0);
                        std::cmp::max(key.len(), max_cell)
                    })
                    .collect();

                let header_cells: Vec<String> = keys
                    .iter()
                    .enumerate()
                    .map(|(col_idx, key)| pad_right(key, col_widths[col_idx]))
                    .collect();

                let separator_cells: Vec<String> = col_widths
                    .iter()
                    .map(|width| "-".repeat(*width))
                    .collect();

                let mut lines = Vec::new();
                lines.push(header_cells.join("  "));
                lines.push(separator_cells.join("  "));
                for row in &all_rows {
                    let val_cells: Vec<String> = row
                        .iter()
                        .enumerate()
                        .map(|(col_idx, cell)| pad_right(cell, col_widths[col_idx]))
                        .collect();
                    lines.push(val_cells.join("  "));
                }
                lines.join("\n")
            } else {
                let items: Vec<String> =
                    arr_val.iter().map(|inner_val| val_to_string(inner_val)).collect();
                items.join(", ")
            }
        }
        Value::Object(map_val) => {
            let sub_keys: Vec<&String> = map_val.keys().collect();
            let sub_vals: Vec<String> = map_val
                .values()
                .map(|inner_val| val_to_string(inner_val))
                .collect();

            let sub_widths: Vec<usize> = sub_keys
                .iter()
                .enumerate()
                .map(|(col_idx, key)| {
                    std::cmp::max(
                        key.len(),
                        sub_vals.get(col_idx).map(|val| val.len()).unwrap_or(0),
                    )
                })
                .collect();

            let header_cells: Vec<String> = sub_keys
                .iter()
                .enumerate()
                .map(|(col_idx, key)| pad_right(key, sub_widths[col_idx]))
                .collect();

            let separator_cells: Vec<String> = sub_widths
                .iter()
                .map(|width| "-".repeat(*width))
                .collect();

            let val_cells: Vec<String> = sub_vals
                .iter()
                .enumerate()
                .map(|(col_idx, cell)| pad_right(cell, sub_widths[col_idx]))
                .collect();

            format!(
                "{}\n{}\n{}",
                header_cells.join("  "),
                separator_cells.join("  "),
                val_cells.join("  ")
            )
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
                                .map(
                                    |(key, val)| (
                                        key.clone(), val.as_str().unwrap_or("").to_string()
                                    )
                                )
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
    let padding = width.saturating_sub(content.chars().count());
    format!("{}{}", content, " ".repeat(padding))
}

fn append_rows(lines: &mut Vec<String>, formatted_rows: &[Vec<String>], column_widths: &[usize]) {
    for row in formatted_rows {
        let has_subtable = row.iter().any(|cell| cell.contains('\n'));
        if !has_subtable {
            let padded_cells: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(col_idx, cell)| pad_right(cell, column_widths[col_idx]))
                .collect();
            lines.push(format!("| {} |", padded_cells.join(" | ")));
        } else {
            let max_sublines = row
                .iter()
                .map(|cell| cell.lines().count())
                .max()
                .unwrap_or(1);

            let split_cells: Vec<Vec<String>> = row
                .iter()
                .map(|cell| cell.lines().map(|line| line.to_string()).collect())
                .collect();

            for line_idx in 0..max_sublines {
                let padded_cells: Vec<String> = split_cells
                    .iter()
                    .enumerate()
                    .map(|(col_idx, cell_lines)| {
                        let content = cell_lines
                            .get(line_idx)
                            .map(|line| line.as_str())
                            .unwrap_or("");
                        pad_right(content, column_widths[col_idx])
                    })
                    .collect();
                lines.push(format!("| {} |", padded_cells.join(" | ")));
            }
        }
    }
}

fn build_table(
    rows: &[Value],
    header_map: Option<&BTreeMap<String, String>>,
) -> Result<String, String> {
    if rows.is_empty() {
        return Err("No rows found".to_string());
    }

    let first_obj = rows[0].as_object().ok_or("First row is not an object")?;
    let columns: Vec<String> = first_obj.keys().cloned().collect();

    let display_headers: Vec<String> = columns
        .iter()
        .map(|col| {
            if let Some(map) = header_map {
                map.get(col).cloned().unwrap_or_else(|| col.clone())
            } else {
                col.clone()
            }
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
                .map(|row| {
                    row.get(col_idx)
                        .map(|cell| cell.lines().map(|line| line.chars.count()).max().unwrap_or(0))
                        .unwrap_or(0)
                })
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

    append_rows(&mut lines, &formatted_rows, &column_widths);

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

    let header_map = find_header_object(&parsed, &table_keys);
    build_table(&table_rows, header_map.as_ref())
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
