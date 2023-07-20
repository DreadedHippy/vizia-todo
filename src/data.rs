use vizia::prelude::*;
use crate::events::TodoEvent;
#[derive(Lens)]
pub struct  TodoList {
	pub todos: Vec<TodoItem>,
	pub new_title: String
}

#[derive(Clone)]
pub struct TodoItem {
	pub title: String,
	pub done: bool
}

impl Model for TodoList {
	fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
		event.map(|todo_event, _| match todo_event {

			TodoEvent::SetTitle(title) => {
				self.new_title = title.clone();
			}

			TodoEvent::Create => {
				if !self.new_title.is_empty() {
					self.todos.push(TodoItem { title: self.new_title.clone(), done: false });
					self.new_title = "".to_string();
				}
			},

			TodoEvent::Delete(index) => {
				self.todos.remove(index.clone());
			},

			TodoEvent::Complete(index) => {
				self.todos[*index].done = !self.todos[*index].done;
			}

		})
	}

}
