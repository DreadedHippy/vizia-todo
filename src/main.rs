mod ui;
mod data;
mod events;
use data::AppData;
use ui::ui_builder;
use vizia::prelude::*;

fn main() {
    Application::new(|cx|{

        // Add the stylesheet to the app
        cx.add_stylesheet(include_style!("src/styles/style.css")).expect("Failed to load stylesheet");

        // Build the data into the application
        AppData{count: 0}.build(cx);
        
        // Contents
        ui_builder(cx);
    })
    .title("Todo Vizia")
    .inner_size((300, 300))
    .run();    
}