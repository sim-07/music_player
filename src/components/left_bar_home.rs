use gtk4::{prelude::*, Box, Button, Label, Orientation, Stack, CssProvider};
use gtk4::gdk::Display;
use std::rc::Rc;

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("assets/styles/left_bar_home.css");

    let display = Display::default().expect("Errore: nessun display disponibile");
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn build_left_bar(stack: Rc<Stack>) -> Box {

    load_css();

    let main_container = Box::new(Orientation::Vertical, 0);

    let label = Label::new(Some("Playlist"));
    label.set_margin_bottom(10);
    let button = Button::with_label("Settings");

    let stack_clone = Rc::clone(&stack);
    button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("settings");
    });

    main_container.append(&label);
    main_container.append(&button);
    main_container.add_css_class("container");

    main_container
}
