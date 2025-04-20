use glib::clone;
use gtk4::gdk::Display;
use gtk4::subclass::window;
use gtk4::{
    prelude::*, ApplicationWindow, Box, Builder, Button, CssProvider, Dialog, Entry, Label,
    Orientation, Picture, Stack,
};
use lazy_static::lazy_static;
use std::rc::Rc;
use std::sync::Mutex;

use crate::scripts::create_playlist::create_playlist;

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

// fn set_playlist_name(name: &str) {
//     let mut guard = PLAYLIST_NAME.lock().unwrap();
//     *guard = name.to_owned();
// }

// // fn get_playlist_name() -> String {
// //     PLAYLIST_NAME.lock().unwrap().clone()
// // }

lazy_static! {
    static ref PLAYLIST_NAME: Mutex<String> = Mutex::new(String::new());
}

fn show_input_dialog(parent: &ApplicationWindow) {
    let dialog = Dialog::builder()
        .transient_for(parent)
        .modal(true)
        .title("Create playlist")
        .build();

    let content_area = dialog.content_area();
    let label = Label::new(Some("Playlist name:"));
    let entry = Entry::new();

    content_area.append(&label);
    content_area.append(&entry);

    dialog.add_button("Annulla", gtk4::ResponseType::Cancel);
    dialog.add_button("OK", gtk4::ResponseType::Ok);

    let entry_weak = entry.downgrade();
    dialog.connect_response(move |dialog, response| {
        if response == gtk4::ResponseType::Ok {
            if let Some(entry) = entry_weak.upgrade() {
                let text = entry.text().to_string();
                let mut guard = PLAYLIST_NAME.lock().unwrap();
                *guard = text.to_owned();
                println!("Hai inserito: {}", text);
            }
        }
        dialog.close();
    });

    dialog.show();
}

pub fn build_left_bar(stack: Rc<Stack>, window: &ApplicationWindow) -> Box {
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

    let window_weak = window.downgrade();
    create_pl_button.connect_clicked(move |_| {
        if let Some(window) = window_weak.upgrade() {
            show_input_dialog(&window);
        }
    });

    container.add_css_class("container");

    container
}
