use colored::*;

// Parsing file
pub(crate) fn handle_texts(_text: &str) -> String {
    let lines: Vec<String> = _text.lines().map(|line| {

        if line.starts_with("BOLD_ITALIC ") {
            line[12..]
                .bold().
                italic().
                white().
                to_string()
        } else if line.starts_with("BOLD ") {
            line[5..]
                .bold()
                .white()
                .to_string()
        } else if line.starts_with("CG ") {
            line[3..]
                .green()
                .to_string()
        } else {
            line.to_string()
        }
    }).collect();

    lines.join("\n")
}
