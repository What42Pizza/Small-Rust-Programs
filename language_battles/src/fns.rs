use std::path::PathBuf;
use num_traits::{ToPrimitive, FromPrimitive};



pub fn get_program_dir() -> PathBuf {
    let mut path = std::env::current_exe()
        .expect("Could not retrieve the path of the current program.");
    path.pop();
    path
}



pub fn split_string_lines (s: &str) -> Vec<String> {
    let mut output = vec!(String::new());
    for char in s.chars() {
        match char {
            '\n' => {
                output.push(String::new());
            }
            '\r' => {},
            _ => output.last_mut().unwrap().push(char),
        }
    }
    output
}



pub fn some_if<T> (condition: bool, some_fn: impl FnOnce() -> T) -> Option<T> {
    if condition {
        Some(some_fn())
    } else {
        None
    }
}



pub fn div_round<T> (a: T, b: T) -> T where T: ToPrimitive + FromPrimitive {
    T::from_f64(a.to_f64().unwrap() / b.to_f64().unwrap() + 0.5).unwrap()
}
