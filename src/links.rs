use unicode_width::UnicodeWidthChar;

const URL_SCHEMES: [&str; 3] = ["https://", "http://", "file://"];

pub(crate) fn url_at_cell(text: &str, row: u16, col: u16) -> Option<String> {
    let line = text.lines().nth(row as usize)?;
    url_in_line_at_cell(line, col)
}

fn url_in_line_at_cell(line: &str, col: u16) -> Option<String> {
    for (start, _) in line.char_indices() {
        if !starts_url_scheme(line, start) || !is_url_start_boundary(line, start) {
            continue;
        }

        let raw_end = scan_url_end(line, start);
        let end = trim_url_end(&line[start..raw_end])? + start;
        let start_col = display_col_at_byte(line, start);
        let end_col = display_col_at_byte(line, end);
        let target = usize::from(col);
        if target >= start_col && target < end_col {
            return Some(line[start..end].to_string());
        }
    }

    None
}

fn starts_url_scheme(line: &str, byte_idx: usize) -> bool {
    let rest = &line[byte_idx..];
    URL_SCHEMES.iter().any(|scheme| rest.starts_with(scheme))
}

fn is_url_start_boundary(line: &str, byte_idx: usize) -> bool {
    let Some(prev) = line[..byte_idx].chars().next_back() else {
        return true;
    };
    prev.is_whitespace() || matches!(prev, '(' | '[' | '{' | '<' | '"' | '\'')
}

fn scan_url_end(line: &str, start: usize) -> usize {
    for (offset, ch) in line[start..].char_indices() {
        if ch.is_whitespace() || matches!(ch, '<' | '>' | '"' | '\'') {
            return start + offset;
        }
    }
    line.len()
}

fn trim_url_end(url: &str) -> Option<usize> {
    let mut end = url.len();
    while end > 0 {
        let ch = url[..end].chars().next_back()?;
        if should_trim_trailing(url, end, ch) {
            end -= ch.len_utf8();
        } else {
            break;
        }
    }
    (end > 0).then_some(end)
}

fn should_trim_trailing(url: &str, end: usize, ch: char) -> bool {
    match ch {
        '.' | ',' | ';' | ':' | '!' | '?' => true,
        ')' => closing_outnumbers_opening(&url[..end], '(', ')'),
        ']' => closing_outnumbers_opening(&url[..end], '[', ']'),
        '}' => closing_outnumbers_opening(&url[..end], '{', '}'),
        _ => false,
    }
}

fn closing_outnumbers_opening(text: &str, opening: char, closing: char) -> bool {
    text.chars().filter(|ch| *ch == closing).count()
        > text.chars().filter(|ch| *ch == opening).count()
}

fn display_col_at_byte(line: &str, byte_idx: usize) -> usize {
    line[..byte_idx]
        .chars()
        .map(|ch| UnicodeWidthChar::width(ch).unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_http_url_under_clicked_cell() {
        let text = "open https://example.com/docs now\n";

        assert_eq!(
            url_at_cell(text, 0, 8).as_deref(),
            Some("https://example.com/docs")
        );
        assert_eq!(url_at_cell(text, 0, 4), None);
    }

    #[test]
    fn trims_sentence_punctuation() {
        let text = "see (https://example.com/docs).\n";

        assert_eq!(
            url_at_cell(text, 0, 6).as_deref(),
            Some("https://example.com/docs")
        );
    }

    #[test]
    fn keeps_balanced_parentheses_inside_url() {
        let text = "see https://example.com/a(b)\n";

        assert_eq!(
            url_at_cell(text, 0, 12).as_deref(),
            Some("https://example.com/a(b)")
        );
    }

    #[test]
    fn respects_display_width_before_url() {
        let text = "λ https://example.com\n";

        assert_eq!(url_at_cell(text, 0, 1), None);
        assert_eq!(
            url_at_cell(text, 0, 3).as_deref(),
            Some("https://example.com")
        );
    }

    #[test]
    fn supports_file_urls() {
        let text = "file:///tmp/example.txt\n";

        assert_eq!(
            url_at_cell(text, 0, 2).as_deref(),
            Some("file:///tmp/example.txt")
        );
    }
}
