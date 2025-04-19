use glib::clone;
use gtk4::{prelude::*, Builder, Box, Button, CssProvider, Picture};
use gtk4::gdk::Display;
use std::rc::Rc;

pub struct PlayerControls {
    pub container: Box,
    pub play_button: Button,
    pub pause_button: Button,
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("assets/styles/player_controls.css");

    let display = Display::default().expect("Errore: nessun display disponibile");
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn build_player_controls() -> PlayerControls {
    load_css();

    let ui_src = include_str!("../ui/player_controls.ui");
    let builder = Builder::from_string(ui_src);

    let container: Box = builder
        .object("player_controls_container")
        .expect("Non trovato: player_controls_container");

    let play_button: Button = builder
        .object("play_button")
        .expect("Non trovato: play_button");

    let pause_button: Button = builder
        .object("pause_button")
        .expect("Non trovato: pause_button");

    let skip_previous_button: Button = builder
        .object("skip_previous_button")
        .expect("Non trovato: skip_previous_button");

    let skip_next_button: Button = builder
        .object("skip_next_button")
        .expect("Non trovato: skip_next_button");

    let play_icon = Picture::for_filename("assets/icons/play.svg");
    let pause_icon = Picture::for_filename("assets/icons/pause.svg");
    let skip_previous_icon = Picture::for_filename("assets/icons/skip_previous.svg");
    let skip_next_icon = Picture::for_filename("assets/icons/skip_next.svg");

    let play_box = Box::new(gtk4::Orientation::Horizontal, 5);
    play_box.append(&play_icon);
    play_button.set_child(Some(&play_box));
    play_button.add_css_class("button");

    let pause_box = Box::new(gtk4::Orientation::Horizontal, 5);
    pause_box.append(&pause_icon);
    pause_button.set_child(Some(&pause_box));
    pause_button.set_visible(false);
    pause_button.add_css_class("button");

    let skip_previous_box = Box::new(gtk4::Orientation::Horizontal, 5);
    skip_previous_box.append(&skip_previous_icon);
    skip_previous_button.set_child(Some(&skip_previous_box));
    skip_previous_button.add_css_class("button");

    let skip_next_box = Box::new(gtk4::Orientation::Horizontal, 5);
    skip_next_box.append(&skip_next_icon);
    skip_next_button.set_child(Some(&skip_next_box));
    skip_next_button.add_css_class("button");

    play_button.connect_clicked(clone!(
        #[strong]
        play_button,
        #[strong]
        pause_button,
        move |_| {
            play_button.set_visible(false);
            pause_button.set_visible(true);
        },
    ));

    pause_button.connect_clicked(clone!(
        #[strong]
        play_button,
        #[strong]
        pause_button,
        move |_| {
            play_button.set_visible(true);
            pause_button.set_visible(false);
        },
    ));

    PlayerControls {
        container,
        play_button,
        pause_button,
    }
}
