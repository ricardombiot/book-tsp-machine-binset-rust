use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::alias::{ActionId};
use crate::tsp::actions::database_actions::DatabaseActions;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;

pub fn run(db : &mut DatabaseActions, action_id : &ActionId){
    let action = db.get_action(action_id);

    match action {
        None => (),
        Some(action) => {
            if !action.was_execute(){
                reduce_map_up_parents(db, action_id);
                fixed_action_as_executed(db, action_id);
            }
        }
    }
}

fn fixed_action_as_executed(db : &mut DatabaseActions, action_id : &ActionId){
    let action = db.get_mut_action(action_id).unwrap();
    action.fixed_as_executed();
}

fn reduce_map_up_parents(db : &mut DatabaseActions, action_id : &ActionId) {
    //println!("reduce_map_up_parents");
    let action = db.get_action(action_id).unwrap();

    let parents = action.props_parents().clone();
    //println!("{:?}",parents);
    let up_color = action.up_color();

    for parent_id in parents.iter() {
        if db.can_use_it(parent_id) {
            db.use_it(parent_id);
            let action_parent = db.get_action(parent_id).unwrap();

            if action_parent.was_execute(){
                let dict_graphs_by_lenght = action_parent.props_graph();
    
                let mut list_derive_graphs = dict_graphs_by_lenght.dict().to_list_values();
                for graph_derive in list_derive_graphs.iter_mut() { 
                    graph_derive.up(up_color, action_id.clone());

                    if graph_derive.valid() {
                        let graph_join = graph_derive.to_owned();
                        push_graph_by_lenght(db, action_id, graph_join);
                    }else{
                        println!("free graph_derive");
                        drop(graph_derive.to_owned());
                    }
                }
            }
        }else{
            panic!("Cannot use ActionId: {}", &parent_id);
        }
    }

}

fn push_graph_by_lenght(db : &mut DatabaseActions, action_id : &ActionId, graph_join : PathGraph){
    if graph_join.valid(){
        let action = db.get_mut_action(action_id).unwrap();
        action.push_graph_by_lenght(graph_join);
    }
}


/*

    function reduce_map_up_parents!(db :: IDBActions, action :: Action)
        parents = Actions.get_parents(action)
        up_color = action.up_color

        # $ O(N) $
        for parent_id in parents
            action_parent = DatabaseInterface.get_action(db, parent_id)

            if Actions.was_execute(action_parent)
                dict_graphs = Actions.get_graph(action_parent)

                # $ O(N) $
                for (lenght, graph_parent) in dict_graphs
                    copy_graph = deepcopy(graph_parent)

                    # $ O(N^10) $
                    PathGraph.up!(copy_graph, up_color, action.id)

                    if copy_graph.valid
                        Actions.push_graph_by_lenght!(action, copy_graph)
                    end
                end
            end
        end
    end*/