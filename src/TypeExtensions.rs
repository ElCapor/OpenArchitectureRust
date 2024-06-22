pub trait StringExt
{
    fn get_char_count(&self, char_ : char) -> usize;
}
impl StringExt for String
{
    fn get_char_count(&self, char_ : char) -> usize {
        return self.chars().filter(|c| *c == char_).count();
    }
}
