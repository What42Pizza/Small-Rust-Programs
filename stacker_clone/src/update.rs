use std::{error::Error, time::Duration};
use sdl2::{EventPump, event::Event, keyboard::Keycode};

use crate::structs::*;



pub fn update (game_data: &mut GameData, event_pump: &mut EventPump, dt: &Duration) -> Result<(), Box<dyn Error>> {
    
    process_events(game_data, event_pump);

    match game_data.mode {

        GameMode::Moving => {
            update_moving_mode(game_data, dt);
        }

        GameMode::Waiting => {
            update_waiting_mode(game_data, dt);
        }

        GameMode::Flashing => {
            update_flashing_mode(game_data, dt);
        }

        GameMode::Won => {
            update_won_mode(game_data, dt);
        }

        GameMode::Lost => {
            update_lost_mode(game_data, dt);
        }

    }

    Ok(())

}





fn update_moving_mode (game_data: &mut GameData, dt: &Duration) {
    game_data.time_til_move -= dt.as_secs_f64();
    if game_data.time_til_move < 0.0 {
        game_data.time_til_move += game_data.settings.time_between_moves;
        move_paddle(game_data);
    }
}



fn update_waiting_mode (game_data: &mut GameData, dt: &Duration) {
    game_data.wait_time_left -= dt.as_secs_f64();
    if game_data.wait_time_left <= 0. {
        game_data.mode = GameMode::Moving;
    }
}



fn update_flashing_mode (game_data: &mut GameData, dt: &Duration) {
    game_data.wait_time_left -= dt.as_secs_f64();
    if game_data.wait_time_left <= 0. {
        game_data.mode = GameMode::Moving;
    }
}



fn update_won_mode (game_data: &mut GameData, dt: &Duration) {

    game_data.wait_time_left -= dt.as_secs_f64();
    if game_data.wait_time_left <= 0. {
        game_data.exit = true;
    }

    if !game_data.printed_end_message {
        println!("\n\n\nYou won!\n\n\n");
        game_data.printed_end_message = true;
    }

}



fn update_lost_mode (game_data: &mut GameData, dt: &Duration) {

    game_data.wait_time_left -= dt.as_secs_f64();
    if game_data.wait_time_left <= 0. {
        game_data.exit = true;
    }

    if !game_data.printed_end_message {
        println!("\n\n\nYou lost.\n\n\n");
        game_data.printed_end_message = true;
    }

}





fn move_paddle (game_data: &mut GameData) {

    // bounce
    if game_data.padder_dir > 0 {
        if game_data.paddle.x + game_data.paddle.length >= game_data.grid_size.x {
            game_data.padder_dir = -1;
        }
    } else if game_data.paddle.x == 0 {
        game_data.padder_dir = 1;
    }

    // place
    if game_data.space_pressed {
        place_paddle(game_data);
        return;
    }

    // move
    game_data.paddle.x = (game_data.paddle.x as i32 + game_data.padder_dir) as u32;

}



fn place_paddle (game_data: &mut GameData) {
    let y = game_data.paddle.y;

    let mut first_valid_x = -1;
    let mut last_valid_x = -1;
    for i in 0..game_data.paddle.length {
        let x = game_data.paddle.x + i;
        let block_placed = place_block(x, y, game_data);
        if block_placed {
            last_valid_x = x as i32;
            if first_valid_x == -1 {
                first_valid_x = x as i32;
            }
        }
    }
    let new_paddle_length = if first_valid_x != -1 {last_valid_x - first_valid_x + 1} else {0} as u32;

    let no_missed_blocks = new_paddle_length == game_data.paddle.length;
    if no_missed_blocks {
        game_data.mode = GameMode::Waiting;
        game_data.wait_time_left = 0.5;
    } else {
        game_data.mode = GameMode::Flashing;
        game_data.wait_time_left = 2.;
        game_data.flashing_paddle = Some(FlashingPaddle::from_paddle(&game_data.paddle, game_data));
    }

    if new_paddle_length == 0 {
        game_data.wait_time_left = 3.;
        game_data.mode = GameMode::Lost;
        return;
    }

    game_data.paddle.length = new_paddle_length;
    if first_valid_x != -1 {
        game_data.paddle.x = first_valid_x as u32;
    }

    if game_data.paddle.y > 0 {
        game_data.paddle.y -= 1;
    } else {
        game_data.wait_time_left = 3.;
        game_data.mode = GameMode::Won;
    }

}



fn place_block (x: u32, y: u32, game_data: &mut GameData) -> bool {
    let can_place = can_place_block(x, y, game_data);
    if can_place {
        game_data.grid[(x + y * game_data.grid_size.x) as usize] = true;
    }
    can_place
}

fn can_place_block (x: u32, y: u32, game_data: &mut GameData) -> bool {
    if y == game_data.grid_size.y - 1 {return true;}
    game_data.grid[(x + (y + 1) * game_data.grid_size.x) as usize] // return whether there is a block below pos
}





fn process_events (game_data: &mut GameData, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        process_event(&event, game_data);
    }
}

fn process_event (event: &Event, game_data: &mut GameData) {
    match event {

        Event::Quit {..} => {
            game_data.exit = true;
        }

        Event::KeyDown {timestamp: _, window_id: _, keycode: Some(keycode), scancode: _, keymod: _, repeat: _} => {
            if let Keycode::Escape = keycode {
                game_data.exit = true;
                return;
            }
            process_event_keydown (keycode, game_data);
        }

        Event::KeyUp {timestamp: _, window_id: _, keycode: Some(keycode), scancode: _, keymod: _, repeat: _} => {
            process_event_keyup (keycode, game_data);
        }

        _ => {
        }
    }
}



fn process_event_keydown (keycode: &Keycode, game_data: &mut GameData) {
    if keycode == &Keycode::Space {
        game_data.space_pressed = true;
    }
}

fn process_event_keyup (keycode: &Keycode, game_data: &mut GameData) {
    if keycode == &Keycode::Space {
        game_data.space_pressed = false;
    }
}
