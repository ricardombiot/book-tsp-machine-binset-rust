use std::fmt;
use crate::binset::numeric_set::{UniqueKeyNumber, TNumericSet};


#[derive(Clone, Debug)]
pub struct BinSet128 {
    content : u128
}

impl BinSet128 {

    pub fn new() -> Self { 
        let content:u128 = 0; 
        Self { content } 
    }

    pub fn get_index_key(key : UniqueKeyNumber) -> u8 {
        return (key % 128) as u8
    }
}

impl PartialEq for BinSet128 {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}
impl Eq for BinSet128 {}

impl fmt::Display for BinSet128 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut txt = String::new();
        for n in 0..128 {
            let position = 127 - n; 
            if (self.content >> position & 1) == 1 {
                txt.push('1');
            }else{
                txt.push('0');
            }
        }
        //self.content
        //write!(f, "{:#b}", self.content)
        write!(f,"0b{}", txt)
    }
}

impl TNumericSet for BinSet128 {
    fn push(&mut self, key : UniqueKeyNumber) {
        let index = BinSet128::get_index_key(key);
        let mask = (1 as u128) << (127 - index) ;
        self.content |= mask;
    }

    fn pop(&mut self, key : UniqueKeyNumber) {
        let index = BinSet128::get_index_key(key);
        let mask = (1 as u128) << (127 - index) ;
        self.content ^= mask;
    }

    fn to_empty(&mut self) {
        self.content = 0;
    }

    fn isempty(&self) -> bool {
        return self.content == 0
    }

    fn have(&self, key : UniqueKeyNumber) -> bool {
        let index = BinSet128::get_index_key(key);
        let mask = (1 as u128) << (127 - index) ;

        let have_result = self.content & mask ;
        return have_result != 0;
    }

    fn union(&mut self, set_b : &Self) {
        self.content |= set_b.content;
    }

    fn intersection(&mut self, set_b : &Self) {
        self.content &= set_b.content;
    }

    fn count(&self) -> u32 {
        return self.content.count_ones();
    }
}