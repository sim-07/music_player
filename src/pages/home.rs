use gtk4::{prelude::*, Align, Box, Orientation, Stack};
use std::rc::Rc;

use crate::components::left_bar_home::build_left_bar;
use crate::components::middle_section::build_middle_section;
use crate::components::player_controls::build_player_controls;

pub fn build(stack: Rc<Stack>) -> Box {

    let main_container = Box::new(Orientation::Vertical, 0);
    main_container.set_vexpand(true);

    let horizontal_layout = Box::new(Orientation::Horizontal, 0);

    let left_bar = build_left_bar(Rc::clone(&stack));
    left_bar.set_width_request(300);

    let middle_section = build_middle_section();
    middle_section.set_hexpand(true);

    horizontal_layout.append(&left_bar);
    horizontal_layout.append(&middle_section);
    horizontal_layout.set_vexpand(true);

    let controls = build_player_controls();
    controls.container.set_halign(Align::Center);
    controls.container.set_valign(Align::End);
    controls.container.set_margin_bottom(10);

    main_container.set_margin_top(20);
    main_container.append(&horizontal_layout);
    main_container.append(&controls.container);

    main_container
}
