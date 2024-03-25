use regex::Regex;

pub fn remove_comments(lines: &Vec<String>) -> Vec<String> {
    remove_single_line_comments(&remove_multi_line_comments(lines))
}

fn remove_multi_line_comments(lines: &Vec<String>) -> Vec<String> {
    return lines.clone();

    let mut in_comment: bool = false;
    let mut res: Vec<String> = vec![];
    for line in lines {
        let mut i = 0;
        while i < line.len() - 1 {
            if (line.chars().nth(i).unwrap() == '/' && line.chars().nth(i).unwrap() == '*') {
                // found open
                // TODO:
                i += 1;
            } else if (line.chars().nth(i).unwrap() == '*' && line.chars().nth(i).unwrap() == '/') {
                // found close
                // TODO:
                i += 1;
            }
        }
    }
    res
}

pub fn spaces_to_tabs(lines: &Vec<String>, count: i32) -> Vec<String> {
    todo!();
}

// TODO: optimize
fn remove_single_line_comments(lines: &Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for line in lines {
        if Regex::new(r"[ \t]*//").unwrap().is_match(line) {
            continue;
        }
        res.push(line.clone());
    }
    return res;
}

pub fn keep_lines_in_range(
    min_length: usize,
    max_length: usize,
    lines: &Vec<String>,
) -> Vec<String> {
    lines
        .iter()
        .cloned()
        .filter(|line| line.len() < max_length && line.len() >= min_length)
        .collect()
}
