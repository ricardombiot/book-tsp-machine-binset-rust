use crate::tsp::utils::alias::{UniqueNodeKey};
use std::collections::HashSet;
use std::fmt;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct OwnersFixedSet {
    nobinary_set : HashSet<UniqueNodeKey>
}

impl OwnersFixedSet {
    pub fn new(_size_fixed : UniqueNodeKey) -> Self { 
        let nobinary_set: HashSet<UniqueNodeKey> = HashSet::new(); 
        Self { nobinary_set } 
    }

    pub fn push(&mut self, key : UniqueNodeKey){
        self.nobinary_set.insert(key);
    }

    pub fn pop(&mut self, key : UniqueNodeKey){
        self.nobinary_set.remove(&key);
    }

    pub fn to_empty(&mut self){
        let nobinary_set: HashSet<UniqueNodeKey> = HashSet::new(); 
        self.nobinary_set = nobinary_set;
    }

    pub fn isempty(&self) -> bool {
        return self.nobinary_set.is_empty()
    }

    pub fn have(&self, key : UniqueNodeKey) -> bool {
        return self.nobinary_set.contains(&key);
    }

    pub fn union(&mut self, owners_set_b : &OwnersFixedSet){
        for key in owners_set_b.nobinary_set.iter() {
            self.push(*key);
        }
    }

    pub fn intersect(&mut self, owners_set_b : &OwnersFixedSet){
        let mut keys_to_remove : Vec<UniqueNodeKey> = Vec::new();
        for key  in self.nobinary_set.iter() {
            if !owners_set_b.have(*key) {
                keys_to_remove.push(*key);
            }
        }

        for key in keys_to_remove.iter(){
            self.pop(*key);
        }
    }

    pub fn to_list(&mut self){
        //@TODO
        //self.nobinary_set.iter().collect()
    }

    pub fn get_set(&self) -> &HashSet<UniqueNodeKey> {
        &self.nobinary_set
    }

    pub fn count(&self) -> usize {
        return self.nobinary_set.len()
    }

}

impl PartialEq for OwnersFixedSet {
    fn eq(&self, other: &Self) -> bool {
        return self.nobinary_set.eq(&other.nobinary_set);
    }
}

impl fmt::Display for OwnersFixedSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self.nobinary_set);
    }
}
