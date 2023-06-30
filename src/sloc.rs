#[derive(Debug, Clone, Copy)]
pub struct SourceLocation
{
    pub offset : usize,
    pub len    : usize,
}