use crate::tsp::utils::alias::{UniqueNodeKey};
use std::fmt;
use std::fmt::Debug;

use crate::binset::binset::{BinSet};
use crate::binset::numeric_set::TNumericSet;

#[derive(Clone, Debug)]
pub struct OwnersFixedSet {
    binary_set : BinSet
}

impl OwnersFixedSet {
    pub fn new(_size_fixed : UniqueNodeKey) -> Self { 
        let binary_set: BinSet = BinSet::new(); 
        Self { binary_set } 
    }

    pub fn push(&mut self, key : UniqueNodeKey){
        self.binary_set.push(key);
    }

    pub fn pop(&mut self, key : UniqueNodeKey){
        self.binary_set.pop(key);
    }

    pub fn to_empty(&mut self){
        self.binary_set.to_empty();
    }

    pub fn isempty(&self) -> bool {
        return self.binary_set.isempty()
    }

    pub fn have(&self, key : UniqueNodeKey) -> bool {
        return self.binary_set.have(key);
    }

    pub fn union(&mut self, owners_set_b : &OwnersFixedSet){
        self.binary_set.union(&owners_set_b.binary_set)
    }

    pub fn intersect(&mut self, owners_set_b : &OwnersFixedSet){
        self.binary_set.intersection(&owners_set_b.binary_set);
    }

    pub fn to_list(&mut self){
        //@TODO
        //self.nobinary_set.iter().collect()
    }

   /* pub fn get_set(&self) -> &HashSet<UniqueNodeKey> {
        &self.nobinary_set
    }*/ 

    pub fn count(&self) -> usize {
        return self.binary_set.count() as usize
    }

}

impl PartialEq for OwnersFixedSet {
    fn eq(&self, other: &Self) -> bool {
        return self.binary_set.eq(&other.binary_set);
    }
}

impl fmt::Display for OwnersFixedSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //return write!(f, "{:?}", self.binary_set);
        write!(f, "not_display_binaryset")
    }
}
