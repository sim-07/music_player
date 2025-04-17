use gtk4::{prelude::*, Align};
use gtk4::{Box, Button, Label, Orientation, Stack};
use std::rc::Rc;

use crate::components::player_controls::{self, build_player_controls};

pub fn build(stack: Rc<Stack>) -> Box {
    let main_container = Box::new(Orientation::Vertical, 0);

    let content_box = Box::new(Orientation::Vertical, 10);
    content_box.set_vexpand(true);

    let label = Label::new(Some("Benvenuto alla Home Page!"));
    let button = Button::with_label("Vai alle Impostazioni");

    let stack_clone = Rc::clone(&stack);
    button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("settings");
    });

    content_box.append(&label);
    content_box.append(&button);

    let controls = build_player_controls();
    controls.container.set_halign(Align::Center);
    controls.container.set_margin_bottom(10);

    main_container.append(&content_box);
    main_container.append(&controls.container);

    main_container
}
