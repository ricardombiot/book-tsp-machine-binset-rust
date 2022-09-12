

use crate::tsp::machine::components::graf::Grafo;
use crate::tsp::machine::machines::tsp_machine::TSPMachine;
use crate::tsp::utils::alias::{Color, Km, Weight};

use crate::tsp::pathset::readers::path_reader::PathSolutionReader;

pub fn gen_example() -> Grafo {
    let mut g = Grafo::new(5);

    g.add_bidirectional( 0, 1, 17);
    g.add_bidirectional( 0, 2, 18);
    g.add_bidirectional( 0, 3, 20);
    g.add_bidirectional( 0, 4, 30);

    g.add_bidirectional( 1, 2, 5);
    g.add_bidirectional( 1, 3, 11);
    g.add_bidirectional( 1, 4, 16);

    g.add_bidirectional( 2, 3, 9);
    g.add_bidirectional( 2, 4, 17);

    g.add_bidirectional( 3, 4, 11);

    return g;
}


#[test]
pub fn test_tsp_machine_example(){
    let n = 4 as Color;
    let km_b = 100 as Km;
    let g = gen_example();

    //println!("{:#?}", g);
    println!("Execute test_tsp_machine_example...");
    let color_origin = 0;
    let mut machine = TSPMachine::new(g, km_b, color_origin);
    machine.execute();

    assert_eq!(machine.get_actual_km(),70);

   
    let graph = machine.get_one_solution_graph();
    assert!(graph.is_some());
    //println!("{:#?}", machine);
    let graph = graph.unwrap();


    let path = PathSolutionReader::read(n, km_b, &graph);
    println!("Solution Path: {:?}",path.route());
      /**/
}

#[test]
pub fn test_tsp_machine_complete_random(){
    let n = 7 as Color;
    let peso_min = 4 as Weight;
    let peso_max = 10 as Weight;
    let mut g = Grafo::gen_dircomplete_rnd(n, peso_min, peso_max);

    // Build min optimal cycle...
    let mut origin = 0;
    for destine in 1..n {
        g.add(origin, destine, 5 as Weight);
        origin = destine;
    }
    let min_optimal = (5*(n as Km)) as Km;

    let km_b = (peso_max*(n as Km)) as Km;
    println!("Execute test_tsp_machine_example...");
    let color_origin = 0;
    let mut machine = TSPMachine::new(g, km_b, color_origin);
    machine.execute();

    let optimal = machine.get_actual_km();
    assert!(optimal <= min_optimal);

    let graph = machine.get_one_solution_graph();
    assert!(graph.is_some());
    //println!("{:#?}", machine);
    let graph = graph.unwrap();
    let path = PathSolutionReader::read(n, km_b, &graph);
    println!("Solution Path: {:?}",path.route());

}

/*

#[test]
pub fn test_hal_machine_dode(){
    let n = 20 as Color;
    let b_max = n as Km;
    let g = Grafo::gen_dodecaedro();

    //println!("{:#?}", g);

    let color_origin = 0;
    let mut machine = HamiltonianMachine::new(g, color_origin);

    machine.execute();

    let graph = machine.get_one_solution_graph();
    assert!(graph.is_some());
    //println!("{:#?}", machine);
    let graph = graph.unwrap();

    let path = PathSolutionReader::read(n, b_max, &graph);

    println!("Solution Path: {:?}",path.route());
}

 */