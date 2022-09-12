use crate::tsp::machine::machines::tsp_machine::TSPMachine;
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::machine::components::timeline::timeline_cell::TimelineCell;
use crate::tsp::machine::machines::tsp_machine::Km;

impl TSPMachine {
    pub fn get_actual_km(&self) -> Km {
        return self.actual_km;
    }

    pub fn get_one_solution_graph(&self) -> Option<PathGraph> {
        let origin_cell = self.get_cell_origin();

        match origin_cell {
            Some(origin_cell) if !origin_cell.parents().is_empty() => {
                let parent_id = origin_cell.parents().iter().next().unwrap();

                let action_solution = self.db.get_action(parent_id).unwrap();
                let graph = action_solution.get_max_graph();

                return Some(graph.clone());
            }
            _ => None
        }
    }

    fn get_cell_origin(&self) -> Option<&TimelineCell> {
        let cell_origin = self.timeline.get_cell(&self.actual_km, &self.color_origin);
        
        println!("Have cell origin: {}",  cell_origin.is_some());
        return cell_origin;
    }
}
