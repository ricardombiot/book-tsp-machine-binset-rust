
use std::{collections::HashMap};
use crate::tsp::pathset::components::owners::owners::OwnersByStep;
use crate::tsp::utils::alias::{Step, ActionId};
use crate::tsp::pathset::graph::path_graph::PathGraph;
use crate::tsp::utils::inmutable_dict::InmutableDictCommons;
use crate::tsp::pathset::components::nodes::node_id::{NodeId, NodesIdSet};
use crate::tsp::pathset::components::nodes::node::Node;
use crate::tsp::pathset::components::edges::edge::Edge;
use crate::tsp::pathset::components::nodes::node::dict_edgeid_by_nodeid::DictEdgeIdByNodeId;
use crate::tsp::pathset::components::owners::owners_set::OwnersFixedSet;

use std::fs::File;
use std::io::Write;
use std::process::Command;

pub trait DotGraph {
    fn to_dot(&self) -> String;
    fn to_dot_with_param(&self, param : String) -> String;
}

impl PathGraph {

    pub fn to_png(&self, name : String, path : Option<String>) -> std::io::Result<()> {
        let mut path_txt = "./test_visual/graphs".to_string();
        if path.is_some() {
            path_txt = path.unwrap();
        }
        let current_path = std::env::current_dir().unwrap();
        let path_txt = current_path.display().to_string() + &path_txt.replace("./", "/");

        let input_file = format!("{path}/{name}.dot",path=path_txt, name=name);
        let output_file = format!("{path}/{name}.png",path=path_txt, name=name);
        println!("{}", input_file);
        println!("WRITE IN FILE");

        let data = &self.to_dot();
        let mut f = File::create(&input_file).expect("Unable to create file");
        f.write_all(data.as_bytes()).expect("Unable to write data");

        println!("## Executing... dot to png");
            //dot -Tpng ./hola.dot -o hola.png
        let mut print_to_png = Command::new("/usr/local/bin/dot");
        print_to_png.args(vec!["-Tpng",&input_file,"-o",&output_file]);
        print_to_png.output().expect("failed to execute process");
        println!("#-> Dot to png... [OK]");
        //println!("[status: {:#?}]", &output.stdout.take().unwrap());

        Ok(())
    }

    pub(super) fn _get_list_edges(&self) -> Vec<(NodeId,NodeId)> {
        let mut list_result : Vec<(NodeId,NodeId)> = Vec::new();
        let list_lines = self.table_lines.dict().to_list();

        for (_step, nodes) in list_lines {
            for node_id in nodes {
                let action_id = node_id.action_id();
                let node = self.table_nodes_by_action.get_node(&action_id, &node_id);
                if node.is_some() {
                    let node = node.unwrap();
                    let origin_id = node_id.clone();
                    for son_node_id in node.sons_list() {
                        let destine_id = son_node_id.clone();
                        let edge_tuple = (origin_id.clone(), destine_id);

                        list_result.push(edge_tuple);
                    }
                }
            }
        }

        return list_result ;
    }


    pub(super) fn _get_list_actions_id_by_step(&self) -> HashMap<Step, Vec<ActionId>> {
        let mut dict_lines : HashMap<Step, Vec<ActionId>> = HashMap::new();
        let list_lines = self.table_lines.dict().to_list();

        for (step, nodes) in list_lines {
            let mut list_actions_id :  Vec<ActionId> = Vec::new();
            for node in nodes {
                list_actions_id.push(node.action_id())
            }
            dict_lines.insert(step,list_actions_id);
        }

        return dict_lines;
    }

    pub(super) fn _to_dot_action(&self, action_id : ActionId) -> String {
        let mut dot_txt = format!("subgraph cluster_act_{action_id} {{\n",action_id=&action_id);
        dot_txt += &format!("label = \"Action: {action_id}\" \n",action_id=&action_id);

        let table_nodes = self.table_nodes_by_action.get(&action_id).unwrap();
        let list_nodes = table_nodes.to_list();

        for (_node_id, node) in list_nodes {
            dot_txt += &node.to_dot();
        }
        dot_txt += "}";

        return dot_txt;
    }
}

impl DotGraph for NodeId {
    fn to_dot_with_param(&self, param : String) -> String { 
        return format!("{}{}{}",self.action_id(),param, self.action_parent_id());
    }

    fn to_dot(&self) -> String { "".to_string() }
}

impl DotGraph for PathGraph {
    fn to_dot_with_param(&self, _param : String) -> String { 
        "".to_string()
    }

    fn to_dot(&self) -> String {
        let mut dot_content = "".to_string();

        for (step, list_actions) in self._get_list_actions_id_by_step() {

            let mut def_nodes = "".to_string();
            for action_id in list_actions {
                let action_dot = self._to_dot_action(action_id);
                def_nodes = def_nodes + &action_dot + "\n";
            }

            let content = format!(r#"
            subgraph cluster_{step} {{
            style=filled;
            color=lightgrey;
            node [style=filled,color=white];
            {def_nodes}
            fontsize="10"
            label = "Line {step}";
            }}"#, step = step, def_nodes=def_nodes);

            dot_content += &content;
        }
       
        /*dot_content += &"\n".to_string(); 
        for edge in self.table_edges.dict().to_list_values() {
            dot_content += &edge.to_dot(); 
        }*/

        dot_content += &"\n".to_string(); 
        for (origin_id, destine_id) in self._get_list_edges(){
            dot_content += &edge_tuple_to_dot(&origin_id, &destine_id);
        }


        let mut dot_txt = "digraph G {\n".to_string();
        dot_txt += "subgraph cluster_info {\n";
        dot_txt += &dot_content;
        dot_txt += "}\n";
        dot_txt += "}";
        
        return dot_txt;
    }

}

impl DotGraph for NodesIdSet {
    fn to_dot_with_param(&self, _param : String) -> String { 
        "".to_string()
    }

    fn to_dot(&self) -> String {
        let mut dot_content = String::new();
        
        for node_id in self.iter(){
            dot_content += &node_id.to_dot_with_param(".".to_string());
        }
        
        return dot_content;
    }
}



impl DotGraph for OwnersByStep {
    fn to_dot_with_param(&self, _param : String) -> String { 
        "".to_string()
    }

    fn to_dot(&self) -> String {
        let mut owners_txt = String::new();
        
        for step in 0..self.max_step()+1 {
            let owners = self.get_step_owners(step);
            let mut list_owners: String = "".to_string();
            if owners.is_some() {
                list_owners = owners.unwrap().to_dot();
            }

            owners_txt += &format!("<BR /><FONT POINT-SIZE=\"8\"> Km: {}: {}",step,list_owners);
            owners_txt += "</FONT>";
        }

        return owners_txt;
    }
}

impl DotGraph for OwnersFixedSet {
    fn to_dot_with_param(&self, _param : String) -> String { 
        "".to_string()
    }

    fn to_dot(&self) -> String {
        let mut dot_content = String::new();
        
        /*for key in self.get_set() {
            dot_content += &format!("{},",key);
        }*/
        todo!("Not implement for binaryset");

        return dot_content;
    }
}

impl DotGraph for DictEdgeIdByNodeId {
    fn to_dot_with_param(&self, _param : String) -> String { 
        "".to_string()
    }

    fn to_dot(&self) -> String {
        let mut dot_content = String::new();
        
        for node_id in self.to_list_keys() {
            dot_content += &node_id.to_dot_with_param(".".to_string());
            dot_content += ","
        }
        //let position_last_coma = dot_content.len();
        //dot_content.remove(position_last_coma);
        return dot_content;
    }
}

impl DotGraph for Node {
    fn to_dot_with_param(&self, _param : String) -> String { 
        "".to_string()
    }

    fn to_dot(&self) -> String {
        let color = self.color();
        //let sons_nodes_txt = "".to_string();
        let sons_nodes_txt = self.sons().to_dot();
        //let parents_nodes_txt = "".to_string();
        let parents_nodes_txt = self.parents().to_dot();
        //let owners_html = "".to_string();
        let owners_html = self.owners().to_dot();

        let id_txt = self.id().to_dot_with_param("_".to_string());
        let id_key = self.id().key();
        let node_id_txt = self.id().to_dot_with_param(".".to_string());
        let name = format!("step_{id_txt}", id_txt = id_txt);
        let mut node_label_html = format!(r#"<{color}<BR /><FONT POINT-SIZE="8">ID: {node_id_txt} KEY: {key}</FONT>"#, color=color, node_id_txt=node_id_txt, key=id_key);
        node_label_html += &format!(r#"<BR /><FONT POINT-SIZE="8">Parents: {parents_nodes_txt}</FONT>"#,parents_nodes_txt=parents_nodes_txt);
        node_label_html += &format!(r#"<BR /><FONT POINT-SIZE="8">Sons: {sons_nodes_txt}</FONT>"#,sons_nodes_txt=sons_nodes_txt);
        node_label_html += &owners_html;
        node_label_html += ">";
        let node_definition =  format!("{name} [label={node_label_html}]\n", name=name, node_label_html=node_label_html);

        return node_definition;
    }
}

impl DotGraph for Edge {
    fn to_dot_with_param(&self, _param : String) -> String { 
        "".to_string()
    }

    fn to_dot(&self) -> String {
        let origin_id = self.id().origin_id();
        let destine_id = self.id().destine_id();

        let id_origin_txt = origin_id.to_dot_with_param("_".to_string());
        let id_destine_txt = destine_id.to_dot_with_param("_".to_string());

        let node_origin = format!(r#"step_{id_origin_txt}"#,id_origin_txt=id_origin_txt);
        let node_destine = format!(r#"step_{id_destine_txt}"#,id_destine_txt=id_destine_txt);
    
        let edge_txt = format!("{node_origin} -> {node_destine}; \n",node_origin=node_origin,node_destine=node_destine);

        return edge_txt;
    }
    /*
    function node_edge_to_dot(graph :: Graph, edge :: Edge) :: String
    id_origin_txt = NodeIdentity.to_string(edge.id.origin_id,"_")
    id_destino_txt = NodeIdentity.to_string(edge.id.destine_id,"_")

    node_origin = "step_$(id_origin_txt)"
    node_destine = "step_$(id_destino_txt)"

    edge_txt = "$node_origin -> $node_destine;"

    return "$edge_txt \n"
end
    */
}


fn edge_tuple_to_dot(origin_id: &NodeId, destine_id: &NodeId ) -> String {
    let id_origin_txt = origin_id.to_dot_with_param("_".to_string());
    let id_destine_txt = destine_id.to_dot_with_param("_".to_string());

    let node_origin = format!(r#"step_{id_origin_txt}"#,id_origin_txt=id_origin_txt);
    let node_destine = format!(r#"step_{id_destine_txt}"#,id_destine_txt=id_destine_txt);

    let edge_txt = format!("{node_origin} -> {node_destine}; \n",node_origin=node_origin,node_destine=node_destine);

    return edge_txt;
}