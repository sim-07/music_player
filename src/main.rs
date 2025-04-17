use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Stack, Label, Button, Box, Orientation};
use std::rc::Rc;
use std::cell::RefCell;

mod pages;
mod components;

fn main() {
    let app = Application::builder()
        .application_id("com.example.musicplayer")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Music Player")
            .default_width(1024)
            .default_height(768)
            .build();

        let stack = Stack::new();
        let stack_rc = Rc::new(stack);

        let home_page = pages::home::build(Rc::clone(&stack_rc));
        let settings_page = pages::settings::build(Rc::clone(&stack_rc));

        stack_rc.add_named(&home_page, Some("home"));
        stack_rc.add_named(&settings_page, Some("settings"));

        // Pagina iniziale
        stack_rc.set_visible_child_name("home");

        window.set_child(Some(&*stack_rc));
        window.show();
    });

    app.run();
}
