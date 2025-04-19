use gtk4::gdk::Display;
use gtk4::{prelude::*, Box, Builder, Button, CssProvider, Label, Orientation, Picture, Stack};
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

    let settings_button: Button = builder
        .object("settings_button")
        .expect("Non trovato: settings_button");

    let create_pl_button: Button = builder
        .object("create_pl_button")
        .expect("Non trovato: create_pl_button");

    let settings_icon: Picture = Picture::for_filename("assets/icons/settings.svg");
    let settings_box = Box::new(gtk4::Orientation::Horizontal, 5);
    settings_box.append(&settings_icon);
    settings_button.set_child(Some(&settings_box));

    let create_pl_icon: Picture = Picture::for_filename("assets/icons/add.svg");
    let create_pl_box = Box::new(gtk4::Orientation::Horizontal, 5);
    create_pl_box.append(&create_pl_icon);
    create_pl_button.set_child(Some(&create_pl_box));

    let stack_clone = Rc::clone(&stack);
    settings_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("settings");
    });

    create_pl_button.connect_clicked(move |_| {
        println!("Creazione playlist...");
    });

    container.add_css_class("container");

    container
}
