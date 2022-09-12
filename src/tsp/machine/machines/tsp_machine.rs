use crate::tsp::utils::alias::{Km, Color};
use crate::tsp::machine::components::graf::Grafo;
use crate::tsp::machine::components::timeline::Timeline;
use crate::tsp::actions::database_actions::DatabaseActions;

use std::fmt::Debug;

#[derive(Debug)]
pub struct TSPMachine {
    n : Color,
    actual_km : Km,
    km_b : Km,
    km_solution_recived: Option<Km>,
    color_origin : Color,
    graf : Grafo,
    timeline : Timeline,
    db : DatabaseActions
}

pub mod tsp_machine_constructor;
pub mod tsp_machine_send_destines;
pub mod tsp_machine_execute;
pub mod tsp_machine_solution;