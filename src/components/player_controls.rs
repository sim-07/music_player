use glib::clone;
use gtk4::gdk::Display;
use gtk4::{prelude::*, Box, Button, CssProvider, Label, Orientation, Picture, StyleContext};

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

    let container = Box::new(Orientation::Horizontal, 10);

    let play_icon = Picture::for_filename("assets/icons/play.svg");
    let pause_icon = Picture::for_filename("assets/icons/pause.svg");
    let skip_previous_icon = Picture::for_filename("assets/icons/skip_previous.svg");
    let skip_next_icon = Picture::for_filename("assets/icons/skip_next.svg");

    let skip_previuous_button = Button::new();
    let skip_previuous_box = Box::new(Orientation::Horizontal, 5);
    skip_previuous_box.append(&skip_previous_icon);
    skip_previuous_button.set_child(Some(&skip_previuous_box));
    skip_previuous_button.add_css_class("button");

    let play_button = Button::new();
    let play_box = Box::new(Orientation::Horizontal, 5);
    play_box.append(&play_icon);
    play_button.set_child(Some(&play_box));
    play_button.add_css_class("button");

    let pause_button = Button::new();
    let pause_box = Box::new(Orientation::Horizontal, 5);
    pause_box.append(&pause_icon);
    pause_button.set_child(Some(&pause_box));
    pause_button.add_css_class("button");

    pause_button.set_visible(false);

    let skip_next_button = Button::new();
    let skip_next_box = Box::new(Orientation::Horizontal, 5);
    skip_next_box.append(&skip_next_icon);
    skip_next_button.set_child(Some(&skip_next_box));
    skip_next_button.add_css_class("button");

    // let pb = play_button.clone();
    // let pab = pause_button.clone();

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

    container.append(&skip_previuous_button);
    container.append(&play_button);
    container.append(&pause_button);
    container.append(&skip_next_button);

    PlayerControls {
        container,
        play_button,
        pause_button,
    }
}
