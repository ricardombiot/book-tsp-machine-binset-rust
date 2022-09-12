use crate::tsp::machine::machines::tsp_machine::TSPMachine;
use crate::tsp::utils::alias::{Color, Weight, Step,Km, ActionId};
use crate::tsp::actions::action::Action;
use std::cmp;

impl TSPMachine {
    pub(super) fn send_destines(&mut self, origin : &Color){
        let cell = self.timeline.get_cell(&self.actual_km, &origin).unwrap().clone();
        let action_id = cell.action_id();
        let opt_action = cell.get_action(&self.db, &action_id);
        let mut actions_to_reserve: Vec<ActionId> = Vec::new();

        match opt_action {
            Some(action) if action.valid() => {
                let parent_id = action.id();

                let list_destines: Vec<(Color, Weight)> = self.graf.get_destines(&origin);
                for (destine, weight) in list_destines {
                    if self._is_valid_destine(&action, &destine) {
                        let km_destine = self.actual_km + weight;
                        //println!("{} to {} in {} km", origin, destine, km_destine);

                        //if km_destine <= self.get_km_target(){
                        //}

                        self.timeline.push_parent(&km_destine, &destine, &parent_id);
                        actions_to_reserve.push(parent_id.clone());
                        //self.check_if_solution_recived(destine.clone(), km_destine.clone());
                        if destine == self.color_origin {
                            match self.km_solution_recived {
                                Some(km_solution_recived) => 
                                    self.km_solution_recived = Some(cmp::min(km_solution_recived, km_destine)),
                                _ => 
                                    self.km_solution_recived = Some(km_destine)
                            }

                            println!("{:?}",self.km_solution_recived);
                        }
                    }
                }
            }
            _ => (),
        }

        self.db.reserve_destines(&actions_to_reserve);
    }

    fn _is_valid_destine(&self, action : &Action, destine : &Color) -> bool {
        if destine == &self.color_origin {
            let valid_destine_as_origin = action.max_length_graph() == self.n as Step;
            //println!("Destine origin [{}] GraphLenght {} ", valid_destine_as_origin, action.max_length_graph());
            return valid_destine_as_origin;
        }else{
            return true;
        }   
    }

    fn check_if_solution_recived(&mut self, destine : Color, km_destine : Km) {
        if destine == self.color_origin {
            match self.km_solution_recived {
                Some(km_solution_recived) => 
                    self.km_solution_recived = Some(cmp::min(km_solution_recived, km_destine)),
                _ => 
                    self.km_solution_recived = Some(km_destine)
            }
        }
    }
}


/*

function send_destines!(machine :: TravellingSalesmanMachine, origin :: Color)
    action = TableTimeline.get_action_cell(machine.timeline, machine.db, machine.actual_km, origin)

    if action != nothing && action.valid
        parent_id = action.id
        km_destine_max = nothing
        #! [for] $ O(N) $
        for (destine, weight) in Graf.get_destines(machine.graf, origin)
            if is_valid_destine(machine, action, destine)
                km_destine = machine.actual_km + Km(weight)

                # Only will sent it, if can be optimal solution.
                if km_destine <= km_target(machine)
                    if km_destine_max == nothing
                        km_destine_max = km_destine
                    elseif km_destine > km_destine_max
                        km_destine_max = km_destine
                    end

                    TableTimeline.push_parent!(machine.timeline, km_destine, destine, parent_id)
                    check_if_solution_recived(machine, destine, km_destine)
                end
            end
        end

        if km_destine_max != nothing
            DatabaseMemoryController.register!(machine.db_controller, action.id, km_destine_max)
        end
    end
end

function is_valid_destine(machine :: TravellingSalesmanMachine, action :: Action, destine :: Color)
    if destine == machine.color_origin
        return action.max_length_graph == machine.n
    else
        return true
    end
end

# Only can return to origin if some graph lenght is equal to N.
function check_if_solution_recived(machine :: TravellingSalesmanMachine, destine :: Color, km_destine :: Km)
    if destine == machine.color_origin
        if machine.km_solution_recived != nothing
            machine.km_solution_recived = min(machine.km_solution_recived, km_destine)
        else
            machine.km_solution_recived = km_destine
        end
    end
end
*/