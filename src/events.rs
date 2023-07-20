pub enum TodoEvent {
	SetTitle(String),
	Delete(usize),
	Complete(usize),
	Create,
}