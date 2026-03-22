#[derive(Debug)]
pub enum Errors
{
    IllegalBase64String,
    IllegalBase64Char(char),
}