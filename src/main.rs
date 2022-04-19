mod genetic_algorithm;
mod canvas;
use genetic_algorithm::tsp;
use canvas::Canvas;
use std::thread;
use std::sync::mpsc;
use clap::Parser;

/// A genetic algoritm for the Travelling Salesman Problem
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// number of cities (points) 
    #[clap(short, long)]
    cities: usize,

    /// population size
    #[clap(short, long, default_value_t = 200)]
    population: usize,

    /// mutation rate
    #[clap(short, long, default_value_t = 0.05)]
    mutation: f32,

    /// file with cities separeted by spaces or newlines (xx.xx yy.xx)
    #[clap(short, long)]
    input_file: Option<String>,

    /// generate output file
    #[clap(short, long)]
    output_file: bool,
 
}


fn main() {
    let args = Args::parse();
    let (s_output, r_output) = mpsc::channel();
    let (s_tsp, r_tsp) = mpsc::channel();

    thread::spawn(move || {
        let mut tsp = tsp::Tsp::new(args.cities, args.population, args.mutation);

        match args.input_file {
            Some(path) => tsp.load_cities(path.as_str(), s_tsp),
            None => tsp.generate_cities(s_tsp)
        }
        tsp.generate_population();
        tsp.run(s_output);
    });

    let cities = r_tsp.recv().unwrap();
    let mut canvas = Canvas::new(800, 600, "Genetic Algorithm - TSP", 8., 6.);
    canvas.setup(cities);

    // main loop
    canvas.draw(r_output);
}


