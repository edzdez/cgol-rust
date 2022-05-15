use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, DrawingArea, Inhibit, Orientation};
use gtk4::cairo::Context;

use crate::game::{Game, GameStatus, NUM_CELLS};

#[derive(Copy, Clone, Debug)]
enum Signal {
    Start,
    Stop,
    Step,
    Randomize,
    Clear,
}

fn build_menu(tx: Sender<Signal>) -> gtk::Box {
    let menu_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .margin_top(40)
        .width_request(210)
        .spacing(20)
        .build();

    let start_button = gtk::Button::with_label("Start");
    let step_button = gtk::Button::with_label("Step");
    let randomize_button = gtk::Button::with_label("Randomize");
    let clear_button = gtk::Button::with_label("Clear");

    menu_box.append(&start_button);
    menu_box.append(&step_button);
    menu_box.append(&randomize_button);
    menu_box.append(&clear_button);


    let tx1 = tx.clone();
    start_button.connect_clicked(move |button| {
        if let Some(label) = button.label() {
            if label == "Start" {
                button.set_label("Stop");
                let _ = tx1.send(Signal::Start);
            } else {
                button.set_label("Start");
                let _ = tx1.send(Signal::Stop);
            }
        }
    });


    let tx2 = tx.clone();
    step_button.connect_clicked(move |_| {
        let _ = tx2.send(Signal::Step);
    });

    let tx3 = tx.clone();
    randomize_button.connect_clicked(move |_| {
        let _ = tx3.send(Signal::Randomize);
    });

    clear_button.connect_clicked(move |_| {
        let _ = tx.send(Signal::Clear);
    });

    menu_box
}

fn process_signals(signal: Signal, game: &mut Game) {
    match signal {
        Signal::Start => {
            game.game_status = GameStatus::Started;
        }
        Signal::Stop => {
            game.game_status = GameStatus::Stopped;
        }
        Signal::Step => {
            game.step();
        }
        Signal::Randomize => {
            game.randomize();
        }
        Signal::Clear => {
            game.clear();
        }
    }
}

fn main_loop(game_mutex: Arc<Mutex<Game>>) -> Sender<Signal> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || while let Ok(signal) = rx.recv() {
        let mut game = game_mutex.lock().unwrap();

        process_signals(signal, &mut game);

        while let GameStatus::Started = game.game_status {
            game.step();

            drop(game);

            thread::sleep(Duration::from_millis(200));

            game = game_mutex.lock().unwrap();

            if let Ok(signal) = rx.try_recv() {
                process_signals(signal, &mut game)
            }
        }
    });

    tx
}

pub fn build_ui(app: &Application) {
    let game = Arc::new(Mutex::new(Game::new(NUM_CELLS)));

    {
        let mut game = game.lock().unwrap();
        game.randomize();
    }

    let tx = main_loop(game.clone());

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Conway's Game of Life")
        .default_width(940)
        .default_height(730)
        .resizable(false)
        .build();

    let main_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .spacing(10)
        .build();

    let drawing_area = gtk::DrawingArea::builder()
        .content_width(700)
        .content_height(700)
        .build();

    let game = game.clone();
    drawing_area.set_draw_func(move |drawing_area, ctx, w, h| {
        draw(drawing_area, ctx, game.clone(), w, h);
    });

    main_box.append(&drawing_area);
    main_box.append(&build_menu(tx));

    window.set_child(Some(&main_box));
    window.show();

    gtk::glib::timeout_add_local(Duration::from_millis(200), move || {
        drawing_area.queue_draw();

        Continue(true)
    });
}

fn draw(_: &DrawingArea, ctx: &Context, game: Arc<Mutex<Game>>, w: i32, h: i32) -> Inhibit {
    ctx.scale(w as f64, h as f64);

    let game = game.lock().unwrap();
    let cell_size = 1.0 / NUM_CELLS as f64;

    for (y, row) in game.grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell {
                ctx.set_source_rgb(1.0, 1.0, 1.0);
                ctx.rectangle((x as f64) * cell_size, (y as f64) * cell_size, cell_size, cell_size);
                let _ = ctx.fill();
            } else {
                ctx.set_source_rgb(0.2, 0.2, 0.2);
                ctx.rectangle((x as f64) * cell_size, (y as f64) * cell_size, cell_size, cell_size);
                let _ = ctx.fill();
            }
        }
    }
    let _ = ctx.stroke();

    Inhibit(false)
}