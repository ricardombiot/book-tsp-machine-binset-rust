pub type UniqueKeyNumber = u32;

pub trait TNumericSet {
    fn push(&mut self, key : UniqueKeyNumber);
    fn pop(&mut self, key : UniqueKeyNumber);
    fn to_empty(&mut self);
    fn isempty(&self) -> bool;
    fn have(&self, key : UniqueKeyNumber) -> bool;

    fn union(&mut self, set_b : &Self);
    fn intersection(&mut self, set_b : &Self);

    fn count(&self) -> u32;
}