const CONTENT: &str = r#"    
    lol
    let mut aéé = "lama";
    sticot
"#;

pub fn main()
{
    let mut aéé = "lama";
    let offset = 25;
    let len = 5;
    print_underlined(CONTENT, offset, len);
}

fn print_underlined(source: &str, offset: usize, len: usize)
{
    assert!(source.len() >= offset + len);

    let mut line = 1;
    let mut col = 1;
    let mut chars = source.chars();
    let mut pos = 0;
    let mut last_line_begin = pos;
    let mut c: char = '\0';
    while pos < offset
    {
        c = chars.next().expect("should be enough characters");
        pos += c.len_utf8();
        if c == '\n'
        {
            line += 1;
            col = 1;
            last_line_begin = pos;
        }
        else
        {
            col += 1;
        }
    }

    while pos < offset + len || c != '\n'
    {
        c = chars.next().expect("should be enough characters");
        pos += c.len_utf8();
    }
    let next_line_begin = pos;

    let identifier     : &str = &source[offset..(offset + len)];
    let src_line       : &str = &source[last_line_begin..next_line_begin - 1];
    let pre_underlined : String = " ".repeat(source[last_line_begin..offset].chars().count());
    let underlined     : String = "^".repeat(identifier.chars().count());
    let help           : String = format!("help: if this is intentional, prefix it with an underscore: `_{identifier}`");

    println!(
"warning: unused variable: `{identifier}`
 --> <source>:{line}:{col} 
  |
{line} | {src_line}
  | {pre_underlined}{underlined} {help}",
    );
}