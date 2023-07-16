use vizia::prelude::*;

use crate::data::AppData;
use crate::events::AppEvent;

pub fn ui_builder(cx: &mut Context) {
	HStack::new(cx, |cx|{
		Button::new(cx, |ex| ex.emit(AppEvent::Decrement), |cx| Label::new(cx, "Decrement"))
    .class("dec");
		Button::new(cx, |ex| ex.emit(AppEvent::Increment), |cx| Label::new(cx, "Increment"))
    .class("inc");
		Label::new(cx, AppData::count)
				.class("count");
	})
	.class("row");

}