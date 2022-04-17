mod genetic_algorithm;
use genetic_algorithm::tsp;
use std::thread;
use std::sync::mpsc;
use sfml::system::Vector2f;
use sfml::window::{VideoMode, Style, Event};
use sfml::graphics::{RenderStates,PrimitiveType, Vertex, RenderWindow, RenderTarget, CircleShape, Color, Shape};
/*
clear & LD_LIBRARY_PATH=/usr/local/lib cargo r
*/

fn main() {
    let (tx, rx) = mpsc::channel();
    let (t_tsp, r_tsp) = mpsc::channel();
    thread::spawn(move || {
        let mut tsp = tsp::Tsp::new(20, 100, 0.05);
        tsp.generate_cities(t_tsp);
        tsp.generate_distance_matrix();
        tsp.generate_population();
        tsp.run(tx);
    });
    let cities = r_tsp.recv().unwrap();
    let mut ui_points :Vec<Vertex> = Vec::new();
    let (scale_x, scale_y) = (8., 6.);
    for p in cities {
        let radius = 5.;
        let position = Vector2f::new(
            scale_x * p.x - (radius / 2.),
            scale_y * p.y - (radius / 2.));

        let vertex = Vertex::new(position, Color::WHITE, Vector2f::new(0.,0.));
       ui_points.push(vertex);
    }

    // Create the window of the application
    let mut window = RenderWindow::new(

        VideoMode::new(800, 600, 32),
        "SFML Example",
        Style::CLOSE,
        &Default::default());
    window.set_framerate_limit(60);

    let mut circle = CircleShape::new(20., 32);
    circle.set_fill_color(Color::BLUE);
    while window.is_open() {
        // Event processing
        while let Some(event) = window.poll_event() {
            // Request closing for the window
            if event == Event::Closed {
                window.close();
            }
        }
                
        window.clear(Color::BLACK);


        let mut vertexes: Vec<Vertex> = Vec::new();
        match rx.recv() {
            Ok(data) => {
                // println!("Gen: {} {:?} {}", data.current_gen, data.order, data.fitness);
                println!("{:#?}", data);
                for city in data.order {
                    vertexes.push(ui_points[city].clone());
                }
                if vertexes.len() > 0 {
                    vertexes.push(vertexes[0].clone());
                }
            }
            Err(_) => { println!("Wait");}
        }
        window.draw_primitives(ui_points.as_slice(), PrimitiveType::POINTS, &RenderStates::DEFAULT); 
        window.draw_primitives(vertexes.as_slice(), PrimitiveType::LINE_STRIP, &RenderStates::DEFAULT); 
        window.display();
    
        // Activate the window for OpenGL rendering
        window.set_active(true);
    
        // End the current frame and display its contents on screen
    }

}


