pub fn show_fatal_error(message: impl AsRef<str>) -> ! {
	let message = message.as_ref();
	rfd::MessageDialog::new()
		.set_title("Program Crashed")
		.set_description(message)
		.set_buttons(rfd::MessageButtons::Ok)
		.set_level(rfd::MessageLevel::Error)
		.show();
	panic!("{message}");
}
