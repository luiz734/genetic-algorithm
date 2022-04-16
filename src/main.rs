mod genetic_algorithm;
use genetic_algorithm::tsp;

fn main() {
    let mut tsp = tsp::Tsp::new(30, 200, 0.05);
    tsp.generate_cities();
    tsp.generate_distance_matrix();
    tsp.generate_population();
    tsp.run();
    println!("{:#?}", tsp);

}


