use std::{error::Error, time::Duration};
use sdl2::{EventPump, event::Event, keyboard::{Keycode, self}};

use crate::{structs::*, fns};



pub fn update (game_data: &mut GameData, event_pump: &mut EventPump, dt: &Duration) -> Result<(), Box<dyn Error>> {
    
    process_events(game_data, event_pump)?;
    update_snake(game_data, dt);

    reset_pressed_keys (game_data);

    Ok(())

}



fn update_snake (game_data: &mut GameData, dt: &Duration) {
    let mut snake = &mut game_data.snake;

    let pressed_keys = &game_data.pressed_keys;
    loop {
        if pressed_keys[3] && !snake.direction.is_left()  {snake.direction = Direction::Right; break;}
        if pressed_keys[2] && !snake.direction.is_right() {snake.direction = Direction::Left;  break;}
        if pressed_keys[1] && !snake.direction.is_up()    {snake.direction = Direction::Down;  break;}
        if pressed_keys[0] && !snake.direction.is_down()  {snake.direction = Direction::Up;    break;}
        break;
    }

    snake.time_til_next_move -= dt.as_secs_f64();
    if snake.time_til_next_move > 0. {return;}
    snake.time_til_next_move += game_data.time_between_moves;

    let new_point = fns::shift_point(&snake.head_pos, &snake.direction);
    let map_index = fns::point_to_index(&new_point, &game_data.map_size);

    // wall collision
    if !fns::point_is_in_map(&new_point, &game_data.map_size) {
        game_data.exit = true;
        return;
    }

    // move
    if !snake.just_ate {
        let removed_point = snake.positions.remove(0);
        let removed_map_index = (removed_point.x + removed_point.y * game_data.map_size.x) as usize;
        game_data.map_contents[removed_map_index] = MapTile::None;
    }
    snake.positions.push(new_point);

    // snake collision
    if game_data.map_contents[map_index].is_snake() {
        game_data.exit = true;
        return;
    }


    // place new head in map
    let eating = game_data.map_contents[map_index].is_apple();
    game_data.map_contents[map_index] = MapTile::Snake;
    
    // eat
    if eating {
        snake.just_ate = true;
        fns::move_apple(game_data);
        println!("{}", game_data.snake.positions.len());
    }
    game_data.snake.just_ate = eating;

    game_data.snake.head_pos = new_point;

}





fn reset_pressed_keys (game_data: &mut GameData) {
    let mut pressed_keys = &mut game_data.pressed_keys;
    for key in pressed_keys {
        *key = false;
    }
}





fn process_events (game_data: &mut GameData, event_pump: &mut EventPump) -> Result<(), Box<dyn Error>> {
    for event in event_pump.poll_iter() {
        process_event(&event, game_data)?;
    }
    Ok(())
}

fn process_event (event: &Event, game_data: &mut GameData) -> Result<(), Box<dyn Error>> {
    match event {

        Event::Quit {..} => {
            game_data.exit = true;
            Ok(())
        }

        Event::KeyDown {timestamp, window_id, keycode, scancode, keymod, repeat} => {
            if let Some(keycode) = keycode {
                if let Keycode::Escape = keycode {
                    game_data.exit = true;
                    return Ok(());
                }
                process_event_keydown (keycode, game_data);
            }
            Ok(())
        }

        Event::KeyUp {timestamp, window_id, keycode, scancode, keymod, repeat} => {
            if let Some(keycode) = keycode {
                process_event_keydown (keycode, game_data);
            }
            Ok(())
        }

        _ => {
            Ok(())
        }
    }
}



fn process_event_keydown (keycode: &Keycode, game_data: &mut GameData) {
    match keycode {
        Keycode::Up    => {game_data.pressed_keys[0] = true;}
        Keycode::Down  => {game_data.pressed_keys[1] = true;}
        Keycode::Left  => {game_data.pressed_keys[2] = true;}
        Keycode::Right => {game_data.pressed_keys[3] = true;}
        _ => {}
    }
}

fn process_event_keyup (keycode: &Keycode, game_data: &mut GameData) {
    match keycode {
        Keycode::Up    => {game_data.pressed_keys[0] = false;},
        Keycode::Down  => {game_data.pressed_keys[1] = false;},
        Keycode::Left  => {game_data.pressed_keys[2] = false;},
        Keycode::Right => {game_data.pressed_keys[3] = false;},
        _ => {}
    }
}
