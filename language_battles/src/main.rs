// Started 12/09/22



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// nightly features
//#![feature(box_syntax)]



mod fns;



use std::{fs, error::Error};



fn main() -> Result<(), Box<dyn Error>> {

    // get input
    let mut input_path = fns::get_program_dir();
    input_path.push("input/Language Battles Input.txt");
    let input = fs::read_to_string(input_path)?;
    let input = fns::split_string_lines(&input);

    // get langs
    let mut count = 0;
    let langs = input[0].split(' ')
        .filter(|s| {
            let is_lang = !s.is_empty();
            if is_lang {count += 1;}
            is_lang && count > 1
        })
        .collect::<Vec<&str>>();

    // get points
    let mut all_points_1 = vec!();
    for line in input.iter().skip(1) {
        let points = line.split('|')
            .map(str::trim)
            .collect::<Vec<&str>>();
        let count = points.len();
        let mut i = 0;
        let points = points.into_iter()
            .filter_map(|s| {
                let output = fns::some_if(i > 0 && i < count - 1, || s.parse::<i32>().unwrap());
                i += 1;
                output
            })
            .collect::<Vec<i32>>();
        all_points_1.push(points);
    }

    // get mutlipliers
    let mut multipliers = vec!();
    for mut points in all_points_1.iter_mut() {
        multipliers.push(points.remove(0));
    }

    // swap axes
    let mut all_points_2 = vec![vec!(); langs.len()];
    for points in all_points_1.iter() {
        for (i, &point) in points.iter().enumerate() {
            all_points_2[i].push(point);
        }
    }

    // apply multipliers
    for points in all_points_2.iter_mut() {
        for (i, point) in points.iter_mut().enumerate() {
            *point *= multipliers[i];
        }
    }
    //for (i, points) in all_points_2.iter().enumerate() {
    //    println!("{}: {points:?}", langs[i]);
    //}

    // sum & sort
    let mut lang_totals = all_points_2.into_iter().enumerate()
        .map(|(i, points)| (langs[i], points.iter().sum()))
        .collect::<Vec<(&str, i32)>>();
    lang_totals.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    for lang in lang_totals.iter() {
        print!(
            "{name:name_width$}{total:total_width$}  ",
            name = lang.0.to_string() + ":",
            total = lang.1,
            name_width = 6,
            total_width = 3,
        );
        let mut bar_string = String::new();
        let bar_width = fns::div_round(lang.1, 10);
        for _ in 0..bar_width {bar_string.push('#');}
        println!("{}", bar_string);
    }

    Ok(())
    
}
