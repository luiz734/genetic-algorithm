mod genetic_algorithm;
use genetic_algorithm::TSP;

fn main() {
    let mut tsp = TSP::tsp::new(8, 10);
    tsp.generate_cities();
    tsp.generate_distance_matrix();
    println!("{:#?}", tsp);

}


