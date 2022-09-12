use crate::tsp::utils::alias::{UniqueNodeKey, Step, Color, Km};
use crate::tsp::pathset::components::owners::owners_set::OwnersFixedSet;
use crate::tsp::pathset::components::nodes::node_id::NodeId;
use crate::tsp::utils::generator_node_key;

use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct OwnersByStep {
    // The space of all possible keys
    bbnnn : UniqueNodeKey,
    // Group of set of nodes id key by step
    dict : HashMap<Step, OwnersFixedSet>,
    // The last step
    max_step : Step,
    // If any step is empty, then is invalid
    valid : bool
}

impl OwnersByStep {
    pub fn new(bbnnn: UniqueNodeKey) -> Self { 
        let dict = HashMap::new();
        let max_step: Step = 0;
        let valid = true;
        Self { bbnnn, dict, max_step, valid } 
    }

    pub fn seed(n : Color, b_max : Km) -> Self {
        // Para poder hacer un join con origin
        // tenemos que añadir un km más
        let n = n + 1;
        let b_max = b_max + 1;
    
        let bbnnn= generator_node_key::get_max_unique_node_key(n, b_max);
        OwnersByStep::new(bbnnn)
    }

    pub fn derive(&self) -> Self {
        return self.clone();
    }

    pub fn empty_derive(&self) -> Self {
        return Self::new(self.bbnnn);
    }

    pub fn get_step_owners(&self, step : Step) -> Option<&OwnersFixedSet>{
        if self.dict.contains_key(&step) {
            let borrow_owners_step: Option<&OwnersFixedSet> = self.dict.get(&step);
            return borrow_owners_step;
        }else{
            return None;
        }
    }

    pub fn get_mut_step_owners(&mut self, step : Step) -> Option<&mut OwnersFixedSet>{
        if self.dict.contains_key(&step) {
            let mut_borrow_owners_step : Option<&mut OwnersFixedSet> = self.dict.get_mut(&step);
            return mut_borrow_owners_step;
        }else{
            return None;
        }
    }

    pub fn push(&mut self, node_id : &NodeId){
        self.push_key(node_id.step(), node_id.key())
    }
    pub fn push_key(&mut self, step : Step, key : UniqueNodeKey){
        self._if_dont_existe_create_step(step);

        match self.get_mut_step_owners(step) {
            Some(owners_in_step) => owners_in_step.push(key),
            None => {}
        }

        if step > self.max_step {
            self.max_step = step
        }
    }

    pub fn pop(&mut self, node_id : &NodeId){
        self.pop_key(node_id.step(), node_id.key())
    }
    pub fn pop_key(&mut self, step : Step, key : UniqueNodeKey){
        let mut flag_isempty = false;
        match self.get_mut_step_owners(step) {
            Some(owners_in_step) => {
                owners_in_step.pop(key);

                if owners_in_step.isempty() {
                    flag_isempty = true;
                }
            }
            None => {}
        }


        if flag_isempty {
            self.dict.remove(&step);
            self._to_invalid();
        }
    }

    pub fn isempty(&self, step : Step) -> bool {
        match self.get_step_owners(step) {
            None => return true,
            Some(owners_in_step) => {
                return owners_in_step.isempty();
            }
        }
    }

    pub fn have(&self, node_id : &NodeId) -> bool {
        return self.have_key(node_id.step(), node_id.key());
    }
    pub fn have_key(&self, step : Step, key : UniqueNodeKey) -> bool {
        match self.get_step_owners(step) {
            None => return false,
            Some(owners_in_step) => {
                return owners_in_step.have(key);
            }
        }
    }

    pub fn union(&mut self, owners_b : &OwnersByStep) {
        if self._can_be_valid_operation(&owners_b){
            //println!("Making union...");
            for step in 0..self.max_step()+1 {
               let borrow_owners_set_a = self.get_mut_step_owners(step).unwrap();
               let borrow_owners_set_b = owners_b.get_step_owners(step).unwrap();

               //println!("Step {} ...", step);
               borrow_owners_set_a.union(borrow_owners_set_b);
            }
        }else{
            self._to_invalid();
        }
    }

    pub fn intersect(&mut self, owners_b : &OwnersByStep) {
        self._intersect_loop_short(owners_b, false);
    }
    pub fn intersect_quick(&mut self, owners_b : &OwnersByStep) {
        self._intersect_loop_short(owners_b, true);
    }
    fn _intersect_loop_short(&mut self, owners_b : &OwnersByStep, loop_short: bool) {
        let max_step : Step = std::cmp::min(self.max_step, owners_b.max_step());
        if self._are_both_valids(owners_b) {
            for step in 0..max_step+1 {
                let opt_owners_set_a = self.get_mut_step_owners(step);
                let opt_owners_set_b = owners_b.get_step_owners(step);

                match (opt_owners_set_a,opt_owners_set_b) {
                    (Some(borrow_owners_set_a), Some(borrow_owners_set_b)) => {
                        borrow_owners_set_a.intersect(borrow_owners_set_b);

                        if borrow_owners_set_a.isempty() {
                            self._to_invalid();
                        }
                    },
                    (_, _) => self._to_invalid(),
                }

                if !self.valid && loop_short {
                    return;
                }
            }
        }else{
            self._to_invalid();
        }
    }

    fn _to_invalid(&mut self){
        self.dict = HashMap::new();
        self.max_step = 0;

        self.valid = false;
    }
    
    pub fn count(&self, step : Step) -> usize {
        match self.get_step_owners(step) {
            None => return 0 as usize,
            Some(owners) => return owners.count()
        }
    }

    pub fn max_step(&self) -> Step {
        self.max_step
    }

    pub fn valid(&self) -> bool {
        self.valid
    }
    
}

impl PartialEq for OwnersByStep {
    fn eq(&self, owners_b: &Self) -> bool {
        if self.bbnnn != owners_b.bbnnn {
            return false;
        }else{
            if !self._can_be_valid_operation(owners_b) {
                return false;
            }else{
                let max_step = self.max_step();
                for step in 0..max_step {
                    let owners_set_a  = self.get_step_owners(step).unwrap();
                    let owners_set_b  = owners_b.get_step_owners(step).unwrap();

                    if !owners_set_a.eq(owners_set_b) {
                        return false;
                    }
                }

                return true;
            }
        }
    }
}


impl OwnersByStep {

    fn _create_step_set(&mut self, step : Step) {
        let owners = OwnersFixedSet::new(self.bbnnn);
        self.dict.insert(step, owners);
    }

    fn _if_dont_existe_create_step(&mut self, step : Step){
        if !self.dict.contains_key(&step) {
            self._create_step_set(step);

            if step > (0 as Step) {
                let last_step_dont_exist = !self.dict.contains_key(&(step-1));
                if last_step_dont_exist {
                    self.valid = false;
                }
            }
        }
    }

    fn _can_be_valid_operation(&self, owners_b : &OwnersByStep) -> bool {
        let both_eq_max_step = self.max_step == owners_b.max_step();
        return self._are_both_valids(owners_b) && both_eq_max_step;
    }

    fn _are_both_valids(&self, owners_b : &OwnersByStep) -> bool {
        return self.valid == true && owners_b.valid == true;
    }
}

/*
    function can_be_valid_operation(owners_a :: OwnersByStep, owners_b :: OwnersByStep) :: Bool
        both_valids = owners_a.valid == true && owners_b.valid == true
        both_eq_max_step = owners_a.max_step == owners_b.max_step

        both_valids && both_eq_max_step
    end

    function both_valids(owners_a :: OwnersByStep, owners_b :: OwnersByStep) :: Bool
        owners_a.valid == true && owners_b.valid == true
    end
*/


/*
union!(owners_a :: OwnersByStep, owners_b :: OwnersByStep)
        if can_be_valid_operation(owners_a, owners_b)
            for step in Step(0):owners_a.max_step
                step_set_a = get_step_set(owners_a, step)
                step_set_b = get_step_set(owners_b, step)

                OwnersSet.union!(step_set_a, step_set_b)
            end
        else
            owners_a.valid = false
        end */

/*
    function if_dont_existe_create_step(owners :: OwnersByStep, step :: Step)
        if !haskey(owners.dict, step)
            create_step_set(owners, step)

            if step > Step(0)
                last_step_dont_exist = !haskey(owners.dict, step-1)
                if last_step_dont_exist
                    owners.valid = false
                end
            end
        end
    end */