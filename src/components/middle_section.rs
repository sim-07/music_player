use gtk4::{prelude::*, Box, Button, Label, Orientation, Stack, CssProvider};
use std::rc::Rc;

pub fn build_middle_section() -> Box {

    let main_container = Box::new(Orientation::Vertical, 0);

    let label = Label::new(Some("Middle section"));

    main_container.append(&label);

    main_container
}
