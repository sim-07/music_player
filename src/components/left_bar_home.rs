use gtk4::{prelude::*, Builder, Box, Button, Label, Orientation, Stack, CssProvider};
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

    let ui_src = include_str!("../ui/left_bar_home.ui");
    let builder = Builder::from_string(ui_src);

    let container: Box = builder
        .object("left_bar_container")
        .expect("Non trovato: left_bar_container");

    let button: Button = builder
        .object("left_bar_button")
        .expect("Non trovato: left_bar_button");

    let stack_clone = Rc::clone(&stack);
    button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("settings");
    });

    container.add_css_class("container");

    container
}
