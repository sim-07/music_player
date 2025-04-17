use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, Stack};
use std::rc::Rc;

pub fn build(stack: Rc<Stack>) -> Box {
    let container = Box::new(Orientation::Vertical, 10);
    let label = Label::new(Some("Impostazioni"));
    let button = Button::with_label("Vai alla home page");

    let stack_clone = Rc::clone(&stack);
    button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("home");
    });

    container.append(&label);
    container.append(&button);
    container

    
}
