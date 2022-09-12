#[cfg(test)]
pub mod actions {

    mod database_actions_test;
    mod action_test;

    mod db_controller_test;
}

#[cfg(test)]
pub mod machine {
    pub mod components {
        mod graf_test;
    }

    pub mod machines {
        pub mod hal_machine_test;
        pub mod tsp_machine_test;
    }
}

#[cfg(test)]
pub mod utils {
    mod generator_ids_test;
    mod generator_node_key_test;

    mod inmutable_dict_test;

    mod mutable_test;
}

#[cfg(test)]
pub mod path_graph {
    pub mod path_graph_test;
    pub mod path_graph_delete_test;
    pub mod path_graph_join_test;

    pub mod path_graph_multijoin_test;

    pub mod test_utils;

    pub mod components {
        pub mod nodes {
            pub mod node_test;
            pub mod node_id_test;
        }

        pub mod edges {
            pub mod edge_test;
        }

        pub mod owners {
            pub mod owners_test;
        }
    }
}