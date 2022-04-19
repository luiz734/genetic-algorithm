
pub mod tsp{
   use rand::{Rng, seq::SliceRandom, distributions::{Distribution, Uniform}};
   use num::pow;
   use std::sync::mpsc;
   use std::fs;

   #[derive(Debug)]
   pub struct Data {
      pub current_gen: usize,
      pub order: Vec<usize>,
      pub fitness: f32,
   }
   impl Data {
      pub fn new(current_gen: usize, order: Vec<usize>, fitness: f32) -> Data {
         Data {
               current_gen,
               order,
               fitness,
         }
      } 
   }
   #[derive(Debug, Clone)]
   pub struct Point {
      pub x: f32,
      pub y: f32
   }
   impl Point {
      fn new(x: f32, y: f32) -> Point {
         Point {
            x,
            y
         }
      }
      fn create_random() -> Point {
         let mut rng = rand::thread_rng(); 
         Point {
               x: rng.gen::<f32>() * 100.,
               y: rng.gen::<f32>() * 100.
         }
      }
      pub fn distance(&self, other: &Point) -> f32{
         (pow(self.x - other.x, 2) + pow(self.y - other.y, 2)).sqrt()
      }
   }
   #[derive(Debug)]
   pub struct Tsp {
      n_cities: usize,
      populazion_size: usize,
      cities: Vec<Point>,
      distance_matrix: Vec<Vec<f32>>,
      population: Vec<Individual>,
      mutation_rate: f32,
   }
   impl Tsp {
      pub fn new(n_cities: usize, populazion_size: usize, mutation_rate: f32) -> Tsp {
         Tsp {
            n_cities,
            populazion_size,
            cities: Vec::new(),
            distance_matrix: Vec::new(),
            population: Vec::new(),
            mutation_rate,
         }
      }
      pub fn generate_cities(&mut self, t_tsp: mpsc::Sender<Vec<Point>>) {
         for _i in 0..self.n_cities {
            self.cities.push(Point::create_random());
         }
         t_tsp.send(self.cities.clone()).unwrap();
         self.generate_distance_matrix();
      }
      pub fn load_cities(&mut self, path: &str, t_tsp: mpsc::Sender<Vec<Point>>) {
         let content = match fs::read_to_string(path) {
            Ok(f) => f.replace("\r\n", " ").replace("\n", " "),
            Err(_) => panic!("Coundn't open file")
         };
         // convert raw file as str to vec<f32>
         let content:Vec<f32> = content
            .split(" ")
            .map(|x|x.parse::<f32>().unwrap())
            .collect();
         // let (mut max_x, mut max_y, mut i) = (0., 0., 0);
         // while i+1 < content.len() {
         //    max_x = if content[i] > max_x {content[i]} else {max_x};
         //    max_y = if content[i+1] > max_y {content[i+1]} else {max_y};
         //    i += 2;
         // }
         // // create points vec mappint (x, y) from (0..n, 0..m) to (0..100, 0..100)
         let mut i = 0;
         while i+1 < content.len() {
            self.cities.push(Point::new(
               content[i], /// max_x * 100.,
               content[i+1])); // / max_y * 100.));
            i += 2;
         }      
         t_tsp.send(self.cities.clone()).unwrap();
         self.generate_distance_matrix();
         println!("{:#?}", self.cities);
      }
      fn generate_distance_matrix(&mut self) {
         dbg!(self.n_cities);
         for i in 0..(self.n_cities) {
            self.distance_matrix.push(Vec::new());
            for j in 0..(self.n_cities) {
               let distance = self.cities[i].distance(&self.cities[j]);
               self.distance_matrix[i].push(distance);
            }
         }         
      }
      pub fn generate_population(&mut self) {
         for _i in 0..self.populazion_size {
            let mut individual = Individual::new();
            individual.generate_and_shuffle_cities(self.n_cities);
            individual.calc_fitness(&self);
            self.population.push(individual);
         }
      }
      // TODO: implement optimal selection
      pub fn weighted_index(&mut self) -> usize {
         let weights: Vec<f32> = self.population.iter().map(|idv|1./(pow(idv.fitness, 8)+1.)).collect();
         let total: f32 = weights.iter().sum();
         let weights: Vec<f32> = weights.iter().map(|x|x/total).collect();
         // let total: f32 = weights.iter().sum();
         
         let mut rng = rand::thread_rng(); 
         let random_value = rng.gen::<f32>(); 
         let mut current_value = 0.;
         for i in 0..weights.len() {
            current_value += weights.get(i).unwrap();
            if random_value < current_value {
               return i
            }
         }
         weights.len() - 1
      }
      pub fn weighted_random_parents(&mut self) -> (Individual, Individual){
         let index_a = self.weighted_index(); 
         let mut index_b = self.weighted_index(); 
         while index_a == index_b {
            index_b = self.weighted_index(); 
         }
         
         // vec![self.population[index_a].clone(), self.population[index_b].clone()]
         (self.population[index_a].clone(), self.population[index_b].clone())
      }
      pub fn mutate(&self, child: &mut Individual) {
         let mut rng = rand::thread_rng(); 
         let choices = Uniform::from(1..self.n_cities);
         let random_index_a = choices.sample(&mut rng);
         let random_index_b = choices.sample(&mut rng);
         child.swap_cities(random_index_a, random_index_b);
         child.calc_fitness(&self);
      }
      pub fn find_best_individual(&self) -> Individual {
         let mut best = self.population[0].clone();
         for i in 1..self.population.len() {
            let current_individual = self.population[i].clone();
            let current_fitness = current_individual.fitness;
            if current_fitness < best.fitness {
               best = current_individual;
            } 
         }
         best
      }
      pub fn reproduce(&self, parent_a: &Individual, parent_b: &Individual) -> Individual {
         let mut rng = rand::thread_rng(); 
         let mut choices = Uniform::from(0..self.n_cities - 1);
         let rand_start = choices.sample(&mut rng);
         choices = Uniform::from(rand_start + 1..self.n_cities);
         let rand_end = choices.sample(&mut rng);
         // choices = Uniform::from(rand_start..rand_end);
         // let index = choices.sample(&mut rng);
         // FIX this
         // let mut child = vec![999; self.n_cities];
         let mut child = vec![999; rand_end - rand_start]; 
         child.clone_from_slice(&parent_a.cities[rand_start..rand_end]);
         for i in 0..self.n_cities {
            if !child.contains(&parent_b.cities.get(i).unwrap()) {
               child.push(parent_b.cities.get(i).unwrap().clone());
            }
         }
         let mut child = Individual::new_from_cities(child);
         child.calc_fitness(&self);
         child
      }
      pub fn run(&mut self, tx: mpsc::Sender<Data>) {
         let mut generations: usize = 0;
         let mut best_individual = self.population[0].clone();
         best_individual.fitness = std::f32::INFINITY;
         loop {
            let mut next_population:Vec<Individual> = Vec::new();
            for _i in 0..self.populazion_size {
               let (parent_a, parent_b) = self.weighted_random_parents();
               let mut child = self.reproduce(&parent_a, &parent_b);
               let mut rng = rand::thread_rng(); 
               let random_value = rng.gen::<f32>(); 
               
               if random_value < self.mutation_rate {
                  self.mutate(&mut child);
               }
                 
               next_population.push(child);
            }
            let best_in_gen = self.find_best_individual();
            if generations % 250 == 0 {
               println!("Gen: {}", generations);
            }
            if best_in_gen.fitness < best_individual.fitness {
               best_individual = best_in_gen;
               tx.send(Data::new(generations, best_individual.cities, best_individual.fitness)).unwrap();
            }
            generations += 1;
            self.population = next_population;
         }
      }
   }
   
   #[derive(Debug, Clone)]
   pub struct Individual {
      cities: Vec<usize>,
      fitness: f32,
   }
   impl Individual {
      fn new() -> Individual {
         Individual {
            cities: Vec::new(),
            fitness: 0.,
         }
      }
      fn new_from_cities(cities: Vec<usize>) -> Individual {
         Individual {
            cities,
            fitness: 0.,
         }
      }
      fn generate_and_shuffle_cities(&mut self, n_cities: usize) {
         let mut rng = rand::thread_rng(); 
         self.cities = (0..n_cities).map(|x|x).collect();
         self.cities.shuffle(&mut rng);
      }
      fn calc_fitness(&mut self, tsp: &Tsp) {
         let mut sum = 0.;
         for i in 0..self.cities.len() {
               let city_a = self.cities.get(i%self.cities.len()).unwrap(); 
               let city_b = self.cities.get((i+1)%self.cities.len()).unwrap(); 
               sum += tsp.distance_matrix[*city_a][*city_b];
         } 
         self.fitness = sum;
      }
      fn swap_cities(&mut self, index_a: usize, index_b:usize) {
         let tmp = self.cities[index_a];
         self.cities[index_a] = self.cities[index_b];
         self.cities[index_b] = tmp;
      }
   }
}