use vizia::prelude::*;
use crate::data::TodoList;
use crate::events::TodoEvent;

pub fn ui_builder(cx: &mut Context) {
	VStack::new(cx, |cx| {

		// Todo Input field
		HStack::new(cx, |cx| {
			Label::new(cx, "Title:").width(Pixels(80.0));
			Textbox::new(cx, TodoList::new_title).on_edit(move |cx, text| {
				cx.emit(TodoEvent::SetTitle(text.clone()));
			});
			Button::new(cx, |cx| cx.emit(TodoEvent::Create), |cx| Label::new(cx, "+"));
		});

		// Todo List
		List::new(cx, TodoList::todos, |cx, index, item| {
			HStack::new(cx, |cx| {
				let d_ind = index.clone();
				Checkbox::new(cx, item.map(|todo| todo.done))
				.on_toggle(move |cx| cx.emit(TodoEvent::Complete(d_ind)));
				Label::new(
					cx, item.map(|todo| format!("{}", todo.title)),
				);
				Button::new(cx, move |cx| cx.emit(TodoEvent::Delete(index)), |cx| Label::new(cx, "X"));
			});
		})
		.class("list");
	});
}