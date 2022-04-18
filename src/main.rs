mod genetic_algorithm;
use genetic_algorithm::tsp;
use std::env;
use std::thread;
use std::sync::mpsc;
use sfml::system::Vector2f;
use sfml::window::{VideoMode, Style, Event};
use sfml::graphics::{Transformable, RenderStates,PrimitiveType, Vertex, RenderWindow, RenderTarget, CircleShape, Color, Shape};
/*
clear & LD_LIBRARY_PATH=/usr/local/lib cargo r
*/
fn validate_input(args: Vec<String>) -> Option<(usize, usize, f32 )> {

    if args.len() < 3{
        panic!("Wrong number of parameters");
    }
    let n_cities= match args[1].trim().parse::<usize>() {
        Ok(value) => value,
        Err(_) => panic!("Invalid parameter: n_cities")
    };
    let population_size = match args[2].trim().parse::<usize>() {
        Ok(value) => value,
        Err(_) => panic!("Invalid parameter: population_size")
    };
    let mutation = match args.get(3) {
        Some(x) => {
            match x.trim().parse::<f32>() {
                Ok(value) => value,
                Err(_) => panic!("Invalid parameter: mutation_rate")
            }
        },
        None => 0.05 
    };

    Some((n_cities, population_size, mutation))

}
fn main() {
    let (tx, rx) = mpsc::channel();
    let (t_tsp, r_tsp) = mpsc::channel();
    thread::spawn(move || {
        let args:Vec<String> = env::args().collect();
        let (n_cities, population_size, mutation_rate) = validate_input(args).unwrap(); 
        let mut tsp = tsp::Tsp::new(n_cities, population_size, mutation_rate);
        tsp.generate_cities(t_tsp);
        tsp.generate_distance_matrix();
        tsp.generate_population();
        tsp.run(tx);
    });
    let cities = r_tsp.recv().unwrap();
    let mut ui_points :Vec<CircleShape> = Vec::new();
    let (scale_x, scale_y) = (8., 6.);
    for p in cities {
        let radius = 5.;
        let position = Vector2f::new(
            scale_x * p.x - radius,
            scale_y * p.y - radius);

        let mut circle = CircleShape::new(radius, 32); 
        circle.set_fill_color(Color::WHITE);
        circle.set_position(position);
        circle.set_origin(Vector2f::new(radius, radius));
        // let vertex = Vertex::new(position, Color::WHITE, Vector2f::new(0.,0.));
       ui_points.push(circle);
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
                    let position = ui_points[city].position();
                    let vertex = Vertex::new(position, Color::WHITE, Vector2f::new(0.,0.));
                    vertexes.push(vertex);
                }
                if vertexes.len() > 0 {
                    vertexes.push(vertexes[0].clone());
                }
            }
            Err(_) => { println!("Wait");}
        }

        // Draw
        for p in ui_points.iter() {
            window.draw_circle_shape(p, &RenderStates::DEFAULT);
        }
        window.draw_primitives(vertexes.as_slice(), PrimitiveType::LINE_STRIP, &RenderStates::DEFAULT); 
        window.display();
    
        // window.set_active(true);
    
    }

}


