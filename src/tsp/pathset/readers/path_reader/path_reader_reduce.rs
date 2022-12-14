

use crate::tsp::pathset::readers::path_reader::PathSolutionReader;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

impl PathSolutionReader {
    pub(super) fn _clear_graph_by_owners(&mut self){
        // elemino los no visitados
        let nodes_to_delete = self._save_to_delete_all_nodes_dont_selected_line();
        if nodes_to_delete > 0 {
            self.graph.apply_node_deletes();
            self.graph.review_owners_all_graph();
        }

    }

    fn _save_to_delete_all_nodes_dont_selected_line(&mut self) -> u32 {
        let mut nodes_to_delete = 0;
        let line = self.graph.table_lines().get(&self.step);
        
        match line {
            Some(line) => {
                if self.next_node_id.is_some(){
                    let next_node_id = self.next_node_id.clone().unwrap();
                    for node_id in line.clone() {
                        if !node_id.eq(&next_node_id) {
                            self.graph.save_to_delete(&node_id);
                            //println!(" -> Save to Delete {:?}", &node_id);
                            nodes_to_delete += 1;
                        }else{
                            //println!(" -> Not Save to Delete {:?}", &node_id);
                        }
                    }
                }
            },
            _ => ()
        }

        return nodes_to_delete;
    }
}
