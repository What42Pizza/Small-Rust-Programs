// started 24/12/15



#![feature(random)]



use std::{ops::Rem, random::random, time::Instant};
use glam::{Vec2, Vec3, Vec4};



fn main() {
	let start = Instant::now();
	println!("Starting tests...");
	let nanos = start.elapsed().as_nanos() as u64;
	for _ in 0..nanos%256 {random::<u8>();}
	
	//test_encoder_vec3("emmet", emmet_vec3_to_float, emmet_float_to_vec3);
	//test_encoder_vec3("what42", what42_vec3_to_float, what42_float_to_vec3);
	//test_encoder_vec3("what42_2", what42_2_vec3_to_float, what42_2_float_to_vec3);
	
	test_encoder_vec2("what42", what42_vec2_to_float, what42_float_to_vec2);
	test_encoder_vec2("what42", continuum_vec2_to_float, continuum_float_to_vec2);
	
}



pub fn test_encoder_vec4(name: &'static str, to_float: fn(Vec4) -> f32, to_vec4: fn(f32) -> Vec4) {
	let mut total_error = 0f32;
	let mut max_error = 0f32;
	for _ in 0..1000000 {
		let original = Vec4::new(random_f32(), random_f32(), random_f32(), random_f32());
		let encoded = to_float(original);
		let decoded = to_vec4(encoded);
		let difference = decoded - original;
		total_error += difference.x.abs() + difference.y.abs() + difference.z.abs() + difference.w.abs();
		max_error = max_error.max(difference.x.abs()).max(difference.y.abs()).max(difference.z.abs()).max(difference.w.abs());
	}
	println!("Encoder '{name}' average error: {}, max error: {}", total_error / 3000000.0, max_error);
}



pub fn test_encoder_vec3(name: &'static str, to_float: fn(Vec3) -> f32, to_vec3: fn(f32) -> Vec3) {
	let mut total_error = 0f32;
	let mut max_error = 0f32;
	for _ in 0..1000000 {
		let original = Vec3::new(random_f32(), random_f32(), random_f32());
		let encoded = to_float(original);
		let decoded = to_vec3(encoded);
		let difference = decoded - original;
		total_error += difference.x.abs() + difference.y.abs() + difference.z.abs();
		max_error = max_error.max(difference.x.abs()).max(difference.y.abs()).max(difference.z.abs());
	}
	println!("Encoder '{name}' average error: {}, max error: {}", total_error / 3000000.0, max_error);
}



pub fn test_encoder_vec2(name: &'static str, to_float: fn(Vec2) -> f32, to_vec2: fn(f32) -> Vec2) {
	let mut total_error = 0f32;
	let mut max_error = 0f32;
	for _ in 0..1000000 {
		let original = Vec2::new(random_f32(), random_f32());
		let encoded = to_float(original);
		if encoded < 0.0 || encoded > 1.0 {panic!("Error in {name}: encoded value from {original:?} was {encoded}");}
		let decoded = to_vec2(encoded);
		let difference = decoded - original;
		total_error += difference.x.abs() + difference.y.abs();
		max_error = max_error.max(difference.x.abs()).max(difference.y.abs());
	}
	println!("Encoder '{name}' average error: {}, max error: {}", total_error / 3000000.0, max_error);
}



pub fn random_f32() -> f32 {
	(random::<u32>() as f64 / u32::MAX as f64) as f32
}



// from: https://aras-p.info/blog/2009/07/30/encoding-floats-to-rgba-the-final/

const C_PRECISION: f32 = 128.0;
const C_PRECISIONP1: f32 = C_PRECISION + 1.0;

pub fn emmet_vec3_to_float(mut color: Vec3) -> f32 {
	color = Vec3::clamp(color, Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
	return (color.x * C_PRECISION + 0.5).floor()
		+ ((color.y * C_PRECISION + 0.5) * C_PRECISIONP1).floor()
		+ ((color.z * C_PRECISION + 0.5) * C_PRECISIONP1 * C_PRECISIONP1).floor();
}

pub fn emmet_float_to_vec3(value: f32) -> Vec3 {
	let mut color = Vec3::ZERO;
	color.x = value.rem(C_PRECISIONP1) / C_PRECISION;
	color.y = (value / C_PRECISIONP1).floor().rem(C_PRECISIONP1) / C_PRECISION;
	color.z = (value / (C_PRECISIONP1 * C_PRECISIONP1)).floor() / C_PRECISION;
	return color;
}



pub fn what42_vec3_to_float(mut v: Vec3) -> f32 {
	v *= 127.0; // map to 0-127
	v = Vec3::floor(v + 0.5);
	v.y *= 128.0; // shift 7 bits
	v.z *= 128.0 * 128.0; // shift 14 bits
	let output = v.x + v.y + v.z;
	output / 0x1FFFFF as f32 // map to 0-1
}

pub fn what42_float_to_vec3(mut v: f32) -> Vec3 {
	v *= 0x1FFFFF as f32; // map to 0 - 2^21-1
	let x = v.rem(128.0); // take lowest 7 bits
	v = (v / 128.0).floor(); // shift 7 bits
	let y = v.rem(128.0); // take lowest 7 bits
	v = (v / 128.0).floor(); // shift 7 bits
	let z = v; // take last 7 bits
	Vec3::new(x, y, z) / 127.0 // map to 0-1
}



pub fn what42_2_vec3_to_float(mut v: Vec3) -> f32 {
	v = Vec3::floor(v * 127.0 + 0.5);
	return Vec3::dot(v, Vec3::new(1.0, 128.0, 16384.0)) / 2097151.0;
}

pub fn what42_2_float_to_vec3(mut v: f32) -> Vec3 {
	v *= 16383.99219;
	let x = v;
	v = v.floor() / 127.999939;
	let y = v;
	v = v.floor() / 128.0;
	let z = v;
	return Vec3::new(x, y, z).fract();
}

/*

glsl version:

float what42_2_vec3_to_float(vec3 v) {
    v = floor(v * 127.0 + 0.5);
	return dot(v, vec3(1.0, 128.0, 16384.0)) / 2097151.0;
}

vec3 what42_2_float_to_vec3(float v) {
    v *= 16383.99;
    float x = v;
    v = floor(v) / 127.9999;
    float y = v;
    v = floor(v) / 128.0;
    return vec3(x, y, v);
}

*/



pub fn what42_vec2_to_float(mut v: Vec2) -> f32 {
	v = Vec2::floor(v * 2047.0 + 0.5);
	return Vec2::dot(v, Vec2::new(1.0, 2048.0)) / 0x3FFFFF as f32;
}

pub fn what42_float_to_vec2(mut v: f32) -> Vec2 {
	v *= 0x3FFFFF as f32; // map to 0 - 2^22-1
	let x = v.rem(2048.0); // take lowest 11 bits
	v = (v / 2048.0).floor(); // shift 11 bits
	let y = v; // take last 11 bits
	Vec2::new(x, y) / 2047.0 // map to 0-1
}



pub fn continuum_vec2_to_float(v: Vec2) -> f32 {
	let constant1 = Vec2::new(1.0, 256.0) / 65535.0;
	return Vec2::dot(Vec2::floor(v * 255.0), constant1);
}

pub fn continuum_float_to_vec2(v: f32) -> Vec2 {
	let constant1 = 65535.0 / Vec2::new(256.0, 65536.0);
	let constant2 = 256.0 / 255.0;
	return Vec2::fract(v * constant1) * constant2;
}
