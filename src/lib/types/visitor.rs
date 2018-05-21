

pub trait Visitor<'a> {
    type Type;
    fn visit(&mut self, target: &Self::Type);
    fn get_visited(&self) -> usize;
    fn report(&self);
}