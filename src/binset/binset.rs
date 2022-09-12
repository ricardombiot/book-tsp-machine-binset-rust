use std::collections::{HashMap, HashSet};
use crate::binset::binset128::BinSet128;
use crate::binset::numeric_set::{UniqueKeyNumber, TNumericSet};

#[derive(Clone, Debug)]
pub struct BinSet {
    subsets : Option<HashMap<u64, BinSet128>>
}

impl BinSet {
    pub fn new() -> Self {
        let subsets = None;
        Self{ subsets }
    }

    pub fn get_subset_key(key : UniqueKeyNumber) -> u64 {
        if key == 0 {
            return 0;
        }else{
            let subset_num = key as f64 / 128.0; 
            return subset_num.floor() as u64;
        }
    }

}

impl PartialEq for BinSet {
    fn eq(&self, other: &Self) -> bool {
       if self.isempty() && other.isempty() {
           return true;
       }else{
           return self.subsets == other.subsets;
       }
    }
}
impl Eq for BinSet {}

impl TNumericSet for BinSet {
    fn push(&mut self, key : UniqueKeyNumber) {
        if self.subsets.is_none() {
            self.subsets = Some(HashMap::new());
        }

        let parts = self.subsets.as_mut().unwrap();
        let subset_key = BinSet::get_subset_key(key);
        match parts.get_mut(&subset_key) {
            Some(binary_set128) => {
                binary_set128.push(key);
            },
            None => {
                let mut binary_set128 = BinSet128::new();
                binary_set128.push(key);
                parts.insert(subset_key, binary_set128);
                
            }
        }
    }

    fn pop(&mut self, key : UniqueKeyNumber) {
        match &mut self.subsets {
            Some(parts) => {
                let subset_key = BinSet::get_subset_key(key);
                match parts.get_mut(&subset_key) {
                    Some(binary_set128) => {
                        binary_set128.pop(key);

                        if binary_set128.isempty() {
                            parts.remove_entry(&subset_key);
                        }
                    },
                    None => ()
                }
            }
            ,
            None => (),
        }  
    }

    fn to_empty(&mut self) {
        self.subsets = None;
    }

    fn isempty(&self) -> bool {
        if !self.subsets.is_none() {
            let parts  = self.subsets.as_ref().unwrap();
            return parts.keys().count() == 0;
        }

        return true;
    }

    fn have(&self, key : UniqueKeyNumber) -> bool { 
        match &self.subsets {
            Some(parts) => {
                let subset_key = BinSet::get_subset_key(key);
                match parts.get(&subset_key) {
                    Some(binary_set128) => {
                        binary_set128.have(key)
                    },
                    None => false
                }
            }
            ,
            None => false,
        }       
    }

    fn union(&mut self, set_b : &Self) {
       if self.isempty() {
           if !set_b.isempty() {
                let subsets_b_copy = set_b.subsets.as_ref().unwrap().clone();
                self.subsets = Some(subsets_b_copy);
           }
       }else{
            if !set_b.isempty() {
                let parts_b = set_b.subsets.as_ref().unwrap();
                let parts  = self.subsets.as_mut().unwrap();
                for subset_key in parts_b.keys() {
                    if parts.contains_key(subset_key){
                        let set_128_b = parts_b.get(&subset_key).unwrap();
                        let set_128 = parts.get_mut(&subset_key).unwrap();

                        set_128.union(set_128_b);
                    }else{
                        let binary_set_128_b_copy = parts_b.get(&subset_key).unwrap().clone();
                        parts.insert(*subset_key, binary_set_128_b_copy);
                    }
                }
            }
       }
    }

    fn intersection(&mut self, set_b : &Self) {
        if !self.isempty() {
            if set_b.isempty() {
                self.to_empty();
            }else{
                let parts_b = set_b.subsets.as_ref().unwrap();
                let parts  = self.subsets.as_mut().unwrap();
                let mut list_keys_b : HashSet<u64> = HashSet::new();
                for subset_key in parts_b.keys() {
                    if parts.contains_key(subset_key){
                        let set_128_b = parts_b.get(&subset_key).unwrap();
                        let set_128 = parts.get_mut(&subset_key).unwrap();

                        set_128.intersection(set_128_b);
                    }
                    list_keys_b.insert(*subset_key);
                }

                let list_keys_to_remove : Vec<u64> = parts.keys().filter_map(|subset_key| 
                                                            if !list_keys_b.contains(subset_key) {
                                                                Some(*subset_key)
                                                            }else{
                                                                None
                                                            })
                                                        .collect();
                                    
                for subset_key in list_keys_to_remove {
                    parts.remove_entry(&subset_key);
                }
            }
        }
    }

    fn count(&self) -> u32 {
        let mut total = 0u32;
        if !self.subsets.is_none() {
            let parts  = self.subsets.as_ref().unwrap();
           
            for (_subset_key, set128) in parts {
                total += set128.count();
            }     
        }

        return total;
    }
}