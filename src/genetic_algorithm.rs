
pub mod TSP {
   use rand::Rng;
   use num::pow;

   #[derive(Debug)]
   pub struct Point {
      x: f32,
      y: f32
   }
   impl Point {
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
   pub struct tsp {
      n_cities: usize,
      populazion_size: usize,
      cities: Vec<Point>,
      distance_matrix: Vec<Vec<f32>>,
   }
   impl tsp {
      pub fn new(n_cities: usize, populazion_size: usize) -> tsp {
         tsp {
            n_cities,
            populazion_size,
            cities: Vec::new(),
            distance_matrix: Vec::new(),
         }
      }
      pub fn generate_cities(&mut self) {
         for i in 0..self.n_cities {
            self.cities.push(Point::create_random());
         }
      }
      pub fn generate_distance_matrix(&mut self) {
         dbg!(self.n_cities);
         for i in 0..(self.n_cities) {
            self.distance_matrix.push(Vec::new());
            for j in 0..(self.n_cities) {
               let distance = self.cities[i].distance(&self.cities[j]);
               self.distance_matrix[i].push(distance);
            }
         }         
      }
     
   }
   struct Individual {
      cities: Vec<u8>,
      fitness: f32,
   }
   impl Individual {
      fn calc_fitness(&mut self) {
         let mut sum = 0;
         for i in 0..self.cities.len() {
               let city_a = self.cities.get(i%self.cities.len()).unwrap(); 
               let city_b = self.cities.get((i+1)%self.cities.len()).unwrap(); 
         } 
      }
   }
}