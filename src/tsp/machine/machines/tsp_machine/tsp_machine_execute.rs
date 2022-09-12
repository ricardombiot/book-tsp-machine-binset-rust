
use crate::tsp::machine::machines::tsp_machine::TSPMachine;
use crate::tsp::utils::alias::{Color, Km};
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use std::cmp;

impl TSPMachine {
    pub fn execute(&mut self){
        // # Hamiltonian: $ O(N) $
        //# TSP: $ O(B) $
        println!("Execute...");
        if self.make_step() {
            let km_to_halt = self.get_km_target();
            println!("KM: {}/{}", self.actual_km, km_to_halt);
            self.execute();
        }
    }

    fn make_step(&mut self) -> bool {
        let is_valid_do_step = !self.rules_to_halt_machine();
        if is_valid_do_step {
            self.execute_line();
            self.clean_db();
            self.actual_km += 1 as Km;
            return true;
        }else{
            return false;
        }
    }

    fn rules_to_halt_machine(&self) -> bool {
        let km_to_halt = self.get_km_target();
        return km_to_halt == self.actual_km
    }

    fn get_km_target(&self) -> Km {
        return match self.km_solution_recived {
            Some(km_solution_recived) => cmp::min(km_solution_recived, self.km_b),
            _ => self.km_b
        }
    }

    fn execute_line(&mut self) {
        match self.timeline.get(&self.actual_km) {
            Some(line) => {
                let list_origins = line.to_list_keys();
                for origin in list_origins {
                    let (is_valid, _action_id) = self.timeline.execute(&mut self.db, self.actual_km, origin);
                    
                    if is_valid {
                        //println!("Execute KM:{} Cell: {} -> OP: {} ({})", self.actual_km, origin, action_id.unwrap(), is_valid);
                        self.send_destines(&origin);
                    }
                }
            },
            _ => {
                println!("Step without cells...")
            }
        }
    }

    fn clean_db(&mut self){
        self.db.clean_db();
    }

}

