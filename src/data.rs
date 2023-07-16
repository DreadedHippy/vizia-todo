use vizia::prelude::*;
use crate::events::AppEvent;
#[derive(Lens)]
pub struct AppData {
	pub count: i32,
}

impl Model for AppData {
	fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
		event.map(|app_event, meta| match app_event {
				AppEvent::Decrement => self.count -= 1,
				AppEvent::Increment => self.count += 1,
		});
}
}