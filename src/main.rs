use gtk4::{prelude::*, Builder};
use gtk4::{Application, ApplicationWindow};
use std::rc::Rc;

mod pages;
mod components;
mod scripts;

fn main() {
    let app = Application::builder()
        .application_id("com.example.musicplayer")
        .build();

        app.connect_activate(|app| {
            let builder = Builder::from_file("src/ui/main.ui");
    
            let window: ApplicationWindow = builder
                .object("main_window")
                .expect("Impossibile trovare 'main_window' nel file UI");
    
            window.set_application(Some(app));
    
            let stack: gtk4::Stack = builder
                .object("main_stack")
                .expect("Impossibile trovare 'main_stack' nel file UI");
    
            let stack_rc = Rc::new(stack);
            
            let home_page = pages::home::build(Rc::clone(&stack_rc), &window);
            let settings_page = pages::settings::build(Rc::clone(&stack_rc));
    
            stack_rc.add_named(&home_page, Some("home"));
            stack_rc.add_named(&settings_page, Some("settings"));
    
            // Pagina iniziale
            stack_rc.set_visible_child_name("home");
    
            window.show();
        });

    app.run();
}
