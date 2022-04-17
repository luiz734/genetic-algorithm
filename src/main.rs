mod genetic_algorithm;
use genetic_algorithm::tsp;
use std::thread;
use std::time;
use std::sync::mpsc;
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, Style, Window, Event};
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape};
/*
clear & LD_LIBRARY_PATH=/usr/local/lib cargo r
*/
#[derive(Debug)]
struct Data {
    current_gen: usize,
    points: Vec<tsp::Point>,
    cities: Vec<usize>,
}
impl Data {
    pub fn new(current_gen: usize, cities: Vec<usize>) -> Data {
        Data {
            current_gen,
            points: Vec::new(),
            cities,
        }
    } 
}

fn main() {
    // let mut tsp = tsp::Tsp::new(20, 100, 0.05);
    // tsp.generate_cities();
    // tsp.generate_distance_matrix();
    // tsp.generate_population();
    // tsp.run();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut count: usize = 0;
        loop {
            let data = Data::new(12, vec![1, 2, 3]);
            count += 1;
            tx.send(data).unwrap();
        }
    });
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
        window.draw(&circle);
        window.display();
    
        // Activate the window for OpenGL rendering
        window.set_active(true);
    
        // OpenGL drawing commands go here...
    
        // End the current frame and display its contents on screen
        window.display();
    }
    //  loop {  
    //     let received = rx.recv().unwrap();
    //     println!("Got {:#?}", received);
    // }
}


