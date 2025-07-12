use crate::prelude::*;



pub mod click_fns;
pub mod init;
pub mod render;
pub mod update;



#[derive(Default)]
pub struct CustomGuiData {
	pub image: Option<Texture>,
	pub click_fn: Option<fn(&mut ProgramData) -> Result<()>>,
	pub darken_while_pressed: bool,
}
