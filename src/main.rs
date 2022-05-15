extern crate core;

mod ui;
mod game;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::Application;

fn main() {
    let application = Application::builder()
        .application_id("com.github.edzdez.conway")
        .build();

    application.connect_activate(ui::build_ui);

    application.run();
}