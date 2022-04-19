use std::sync::mpsc;
use sfml::system::Vector2f;
use sfml::window::{VideoMode, Style, Event};
use sfml::graphics::{Transformable, RenderStates,PrimitiveType, Vertex, RenderWindow, RenderTarget, CircleShape, Color, Shape};
use crate::genetic_algorithm::tsp;

pub struct Canvas<'a> {
    pub window: RenderWindow,
    pub ui_points: Vec<CircleShape<'a>>,
    scale: Vector2f,
}
impl<'a> Canvas<'a> {
    pub fn new(width: u32, height: u32, title: &str, scale_x: f32, scale_y: f32) -> Canvas {
        Canvas {
            window: RenderWindow::new(
                VideoMode::new(width, height, 32),
                title, 
                Style::CLOSE,
                &Default::default()),
            ui_points: Vec::new(),
            scale: Vector2f::new(scale_x, scale_y),
        }
    }
    pub fn setup(&mut self, cities: Vec<tsp::Point>) {
        self.window.set_framerate_limit(60);
        for p in cities {
        let radius = 5.;
        let position = Vector2f::new(
                self.scale.x * p.x - radius,
                self.scale.y * p.y - radius);

            let mut circle = CircleShape::new(radius, 32); 
            circle.set_fill_color(Color::WHITE);
            circle.set_position(position);
            circle.set_origin(Vector2f::new(radius, radius));
            self.ui_points.push(circle);
        }   
    }
    fn generate_ui_vertexex(&self, data: tsp::Data) -> Vec<Vertex> {
        println!("{:#?}", data);

        let mut vertexes: Vec<Vertex> = Vec::new();
        for city in data.order {
            let position = self.ui_points[city].position();
            let vertex = Vertex::new(position, Color::WHITE, Vector2f::new(0.,0.));
            vertexes.push(vertex);
        }
        if vertexes.len() > 0 {
            vertexes.push(vertexes[0].clone());
        }
        return vertexes;
    }
    pub fn draw(&mut self, r_output: mpsc::Receiver<tsp::Data>) {
        while self.window.is_open() {
            // Event processing
            while let Some(event) = self.window.poll_event() {
                // Request closing for the window
                if event == Event::Closed {
                    self.window.close();
                }
            }

            self.window.clear(Color::BLACK);
            // try draw vertices (connections)
            match r_output.recv() {
                Ok(data) => {
                    let vertexes = self.generate_ui_vertexex(data);
                    self.window.draw_primitives(vertexes.as_slice(), PrimitiveType::LINE_STRIP, &RenderStates::DEFAULT); 
                },
                Err(_) => { println!("Can't draw vertices");}
            };

            // draw points and vertices 
            for p in self.ui_points.iter() {
                self.window.draw_circle_shape(p, &RenderStates::DEFAULT);
            }
        
            self.window.display(); 
        }  
    }
}