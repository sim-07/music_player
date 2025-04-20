use gtk4::subclass::window;
use gtk4::{prelude::*, Align, ApplicationWindow, Box, Builder, Orientation, Stack};
use std::rc::Rc;

use crate::components::left_bar_home::build_left_bar;
use crate::components::middle_section::build_middle_section;
use crate::components::player_controls::build_player_controls;

pub fn build(stack: Rc<Stack>, window: &ApplicationWindow) -> Box {
    let ui_src = include_str!("../ui/home.ui");
    let builder = Builder::from_string(ui_src);

    let main_container: Box = builder
        .object("main_container")
        .expect("Non trovato: main_container");
    main_container.set_vexpand(true);

    let horizontal_layout: Box = builder
        .object("horizontal_layout")
        .expect("Non trovato: horizontal_layout");

    let controls_container: Box = builder
        .object("player_controls_container")
        .expect("Non trovato: player_controls_container");

    let left_bar = build_left_bar(Rc::clone(&stack), &window);
    left_bar.set_width_request(300);

    let middle_section = build_middle_section();
    middle_section.set_hexpand(true);

    let controls = build_player_controls();
    controls.container.set_halign(Align::Center);
    controls.container.set_valign(Align::End);
    controls.container.set_margin_bottom(10);

    horizontal_layout.append(&left_bar);
    horizontal_layout.append(&middle_section);
    controls_container.append(&controls.container);

    main_container
}
