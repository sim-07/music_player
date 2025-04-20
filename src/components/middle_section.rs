use gtk4::{prelude::*, Box, Builder};
pub fn build_middle_section() -> Box {

    let ui_src = include_str!("../ui/middle_section.ui");
    let builder = Builder::from_string(ui_src);

    let container: Box = builder
        .object("middle_section_container")
        .expect("Non trovato: middle_section_container");

    container
}
