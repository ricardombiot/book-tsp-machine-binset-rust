use crate::tsp::utils::alias::{Color, Weight};
use crate::tsp::machine::components::graf::Grafo;
use rand::prelude::*;

impl Grafo { 
    pub fn gen_complete(n : Color, peso : Weight)-> Self {
        let mut g = Grafo::new(n);

        for origen in 0..n {
            for destino in 0..n {
                if origen != destino {
                    let origin_color : Color = origen.clone() as Color;
                    let destino_color : Color = destino.clone() as Color;
                    let peso : Weight = peso.clone() as Weight;

                    g.add_bidirectional(origin_color, destino_color, peso);
                }
            }
        }

        return g;
    }

    pub fn gen_dircomplete_rnd(n : Color, peso_min : Weight, peso_max : Weight)-> Self {
        let mut g = Grafo::new(n);
        let mut rng = thread_rng();
        //let peso_min = peso_min.clone() as u32;
        //let peso_max = peso_max.clone() as u32;

        for origen in 0..n {
            for destino in 0..n {
                if origen != destino {
                    let origin_color : Color = origen.clone() as Color;
                    let destino_color : Color = destino.clone() as Color;
                    let peso : Weight = rng.gen_range(peso_min..peso_max+1) as Weight;
                    //let peso : Weight = peso.clone() as Weight;

                    g.add(origin_color, destino_color, peso);
                }
            }
        }

        return g;
    }

    pub fn gen_dodecaedro() -> Self {
        let mut g = Grafo::new(20);

        g.add_bidirectional_vec( 0, vec![1, 2, 5], 1);
        g.add_bidirectional_vec( 1, vec![0, 3, 7], 1);
        g.add_bidirectional_vec( 2, vec![0, 4, 13], 1);
        g.add_bidirectional_vec( 3, vec![1, 4, 9], 1);
        g.add_bidirectional_vec( 4, vec![2, 3, 11], 1);
        g.add_bidirectional_vec( 5, vec![0, 6, 14], 1);
        g.add_bidirectional_vec( 6, vec![5, 7, 16], 1);
        g.add_bidirectional_vec( 7, vec![1, 6, 8], 1);
        g.add_bidirectional_vec( 8, vec![7, 9, 17], 1);
        g.add_bidirectional_vec( 9, vec![3, 8, 10], 1);
        g.add_bidirectional_vec( 10, vec![9, 18, 11], 1);
        g.add_bidirectional_vec( 11, vec![4, 12, 10], 1);
        g.add_bidirectional_vec( 12, vec![11, 13, 19], 1);
        g.add_bidirectional_vec( 13, vec![2, 14, 12], 1);
        g.add_bidirectional_vec( 14, vec![5, 15, 13], 1);
        g.add_bidirectional_vec( 15, vec![14, 16, 19], 1);
        g.add_bidirectional_vec( 16, vec![15, 6, 17], 1);
        g.add_bidirectional_vec( 17, vec![16, 18, 8], 1);
        g.add_bidirectional_vec( 18, vec![17, 19, 10], 1);
        g.add_bidirectional_vec( 19, vec![12, 18, 15], 1);

        return g;
    }



    /*
        function dodecaedro() :: Grafo
        n = 20)
        g = g.new(n)

        g.add_bidirectional_vec( 0, vec![1, 2, 5], 1);
        g.add_bidirectional_vec( 1, vec![0, 3, 7], 1);
        g.add_bidirectional_vec( 2, vec![0, 4, 13], 1);
        g.add_bidirectional_vec( 3, vec![1, 4, 9], 1);
        g.add_bidirectional_vec( 4, vec![2, 3, 11], 1);
        g.add_bidirectional_vec( 5, vec![0, 6, 14], 1);
        g.add_bidirectional_vec( 6, vec![5, 7, 16], 1);
        g.add_bidirectional_vec( 7, vec![1, 6, 8], 1);
        g.add_bidirectional_vec( 8, vec![7, 9, 17], 1);
        g.add_bidirectional_vec( 9, vec![3, 8, 10], 1);
        g.add_bidirectional_vec( 10, vec![9, 18, 11], 1);
        g.add_bidirectional_vec( 11, vec![4, 12, 10], 1);
        g.add_bidirectional_vec( 12, vec![11, 13, 19], 1);
        g.add_bidirectional_vec( 13, vec![2, 14, 12], 1);
        g.add_bidirectional_vec( 14, vec![5, 15, 13], 1);
        g.add_bidirectional_vec( 15, vec![14, 16, 19], 1);
        g.add_bidirectional_vec( 16, vec![15, 6, 17], 1);
        g.add_bidirectional_vec( 17, vec![16, 18, 8], 1);
        g.add_bidirectional_vec( 18, vec![17, 19, 10], 1);
        g.add_bidirectional_vec( 19, vec![12, 18, 15], 1);

        return g
    end
    */

}
