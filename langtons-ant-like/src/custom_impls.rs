use crate::prelude::*;



pub trait GuiColor {
	fn as_notan_color(&self) -> notan::prelude::Color;
}

impl GuiColor for crate::gui_mod::data::Color {
	fn as_notan_color(&self) -> notan::prelude::Color {
		notan::prelude::Color::new(self.r, self.g, self.b, self.a)
	}
}



pub trait TupleF32F32 {
	fn to_i32(self) -> (i32, i32);
}

impl TupleF32F32 for (f32, f32) {
	fn to_i32(self) -> (i32, i32) {
		(self.0 as i32, self.1 as i32)
	}
}



pub trait TupleI32I32 {
	fn to_f32(self) -> (f32, f32);
}

impl TupleI32I32 for (i32, i32) {
	fn to_f32(self) -> (f32, f32) {
		(self.0 as f32, self.1 as f32)
	}
}



pub trait TupleU32U32 {
	fn to_f32(self) -> (f32, f32);
}

impl TupleU32U32 for (u32, u32) {
	fn to_f32(self) -> (f32, f32) {
		(self.0 as f32, self.1 as f32)
	}
}
