use crate::sloc::SourceLocation;

pub fn format_err_message(source: &str, sloc: SourceLocation, source_filepath: Option<&str>, description: &str, help: &str) -> String
{
    let SourceLocation{offset, len} = sloc;
    let SubjectInfo{line, col, subject_line_begin, next_line_begin} = SubjectInfo::from_source(source, offset, len);

    let subject_line_end = usize::max(offset + len, next_line_begin - 1);

    let subject        : &str   = &source[offset..(offset + len)];
    let subject_line   : &str   = &source[subject_line_begin..subject_line_end];
    let pre_underlined : String = " ".repeat(source[subject_line_begin..offset].chars().count());
    let underlined     : String = "^".repeat(subject.chars().count());

    let filepath = source_filepath.unwrap_or("<source>");

    format!(
"{description}
 --> {filepath}:{line}:{col} 
  |
{line} | {subject_line}
  | {pre_underlined}{underlined} {help}",
    )
}

struct SubjectInfo
{
    line               : usize,
    col                : usize,
    subject_line_begin : usize,
    next_line_begin    : usize,
}

impl SubjectInfo
{
    fn from_source(source: &str, offset: usize, len: usize) -> Self
    {
        assert!(source.len() >= offset + len);

        let mut line = 1;
        let mut col = 1;
        let mut chars = source.chars();
        let mut pos = 0;
        let mut subject_line_begin = pos;
        let mut c: char;
        while pos < offset
        {
            c = chars.next().expect("should be enough characters");
            pos += c.len_utf8();
            if c == '\n'
            {
                line += 1;
                col = 1;
                subject_line_begin = pos;
            }
            else
            {
                col += 1;
            }
        }

        while pos < offset + len
        {
            c = chars.next().expect("should be enough characters");
            pos += c.len_utf8();
        }

        while match chars.next() {
            Some('\n') | None => false,
            Some(c) => { pos += c.len_utf8(); true },
        } {}
        let next_line_begin = pos;

        Self { line, col, subject_line_begin, next_line_begin }
    }
}