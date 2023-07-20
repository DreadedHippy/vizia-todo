mod ui;
mod data;
mod events;
use data::{TodoList, TodoItem};
use ui::ui_builder;
use vizia::prelude::*;

fn main() {
    Application::new(|cx|{

        // Add the stylesheet to the app
        cx.add_stylesheet(include_style!("src/styles/style.css")).expect("Failed to load stylesheet");


        // Build the App Data
        TodoList{new_title: "".to_string(), todos: vec![TodoItem{ title: "Do a thing".to_string(), done: false}]}
        .build(cx);
        
        // App User interface
        ui_builder(cx);
    })
    .title("Todo Vizia")
    .inner_size((300, 300))
    .run();    
}