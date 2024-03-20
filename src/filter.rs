pub fn remove_comments(lines: &Vec<String>) -> Vec<String> {
    remove_single_line_comments(&remove_multi_line_comments(lines))
}

fn remove_multi_line_comments(lines: &Vec<String>) -> Vec<String> {
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

/**/

fn remove_single_line_comments(lines: &Vec<String>) -> Vec<String> {
    // TODO:
    vec![]
}
