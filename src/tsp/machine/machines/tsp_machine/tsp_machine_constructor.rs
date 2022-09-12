use crate::tsp::machine::machines::tsp_machine::TSPMachine;
use crate::tsp::utils::alias::{Color, Km};
use crate::tsp::machine::components::timeline::Timeline;
use crate::tsp::actions::database_actions::DatabaseActions;
use crate::tsp::machine::components::graf::Grafo;

impl TSPMachine {
    pub fn new(graf : Grafo, km_b : Km, color_origin : Color) -> Self {
        let n = graf.get_n();
        let b_max = km_b;
        let actual_km = 0 as Km;
        let timeline = Timeline::new();
        let km_solution_recived = None;

        let db = DatabaseActions::new(n, b_max, color_origin);
        let mut machine = Self {n, actual_km, km_b, km_solution_recived, color_origin, graf, timeline, db};

        machine._init();

        return machine;
    }

    fn _init(&mut self){
        self.timeline.put_init(&self.n, &self.color_origin);
        let color_origin = self.color_origin;
        self.send_destines(&color_origin);
        self.actual_km += 1 as Km;
    }
}
