use crate::*;
use smart_read::*;



pub struct TileInput {
	pub width: usize,
	pub height: usize,
}

impl TryRead for TileInput {
	type Output = (usize, usize, TileAction);
	type Default = ();
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		loop {
			
			let input = read_stdin()?;
			let input = input.split(' ').collect::<Vec<_>>();
			if input.len() < 3 {
				println!("Please enter two integers and an action separated by spaces");
				println!("Actions: uncover, flag, unflag");
				continue;
			}
			
			let Ok(x) = input[0].parse::<usize>() else {
				println!("The first component must be a positive integer");
				continue;
			};
			if x >= self.width {
				println!("The x value must be lower than {}", self.width);
				continue;
			}
			let Ok(y) = input[1].parse::<usize>() else {
				println!("The second component must be a positive integer");
				continue;
			};
			if y >= self.height {
				println!("The y value must be lower than {}", self.height);
				continue;
			}
			let Some(action) = TileAction::from_str(input[2]) else {
				println!("The third component must be 'uncover' or 'u', 'flag' or 'f', or 'unflag");
				continue;
			};
			
			return Ok((x, y, action));
			
		}
	}
}

pub struct StartingTileInput {
	pub width: usize,
	pub height: usize,
}

impl TryRead for StartingTileInput {
	type Output = (usize, usize);
	type Default = ();
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		loop {
			
			let input = read_stdin()?;
			let input = input.split(' ').collect::<Vec<_>>();
			if input.len() < 3 {
				println!("Please enter two integers separated by spaces");
				continue;
			}
			
			let Ok(x) = input[0].parse::<usize>() else {
				println!("The first component must be a positive integer");
				continue;
			};
			if x >= self.width {
				println!("The x value must be lower than {}", self.width);
				continue;
			}
			let Ok(y) = input[1].parse::<usize>() else {
				println!("The second component must be a positive integer");
				continue;
			};
			if y >= self.height {
				println!("The y value must be lower than {}", self.height);
				continue;
			}
			
			return Ok((x, y));
			
		}
	}
}
