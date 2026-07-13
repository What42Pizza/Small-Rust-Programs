#![allow(unused)]
#![warn(unused_must_use)]

#![feature(exit_status_error)]



pub use std::{path::Path, process::Command, str::FromStr, time::Duration, thread, time::Instant};
pub use anyhow::*;



pub mod utils;
pub use utils::*;



pub struct ProgramSettings {
	run_type: RunType,
	seconds_per_run: u64,
	swap_usage_needed: u64,
	stability_check_min_duration: u64,
	stability_check_max_duration: u64,
	stability_check_interval: u64,
	stability_check_look_ahead: u64,
	excess_ram_needed: u64,
}

impl Default for ProgramSettings {
	fn default() -> Self {
		Self {
			run_type: RunType::Looped,
			seconds_per_run: 30,
			swap_usage_needed: 1024 * 1024 * 1024,
			stability_check_min_duration: 10,
			stability_check_max_duration: 30,
			stability_check_interval: 200,
			stability_check_look_ahead: 15,
			excess_ram_needed: 1024 * 1024 * 1024,
		}
	}
}

pub enum RunType {
	Looped,
	Once,
	Help,
}

fn main() -> Result<()> {
	
	let mut settings = ProgramSettings::default();
	
	let mut args = std::env::args().skip(1);
	loop {
		let Some(arg) = args.next() else { break; };
		match &*arg {
			"--once"                      => settings.run_type = RunType::Once,
			"--help" | "-h"               => settings.run_type = RunType::Help,
			"--seconds-per-run"              => { settings.seconds_per_run              = take_arg::<u64>(&mut args, "--seconds-per-run")?; }
			"--swap-usage-needed"            => { settings.swap_usage_needed            = take_arg::<u64>(&mut args, "--swap-usage-needed")? * 1024 * 1024; }
			"--stability-check-min-duration" => { settings.stability_check_min_duration = take_arg::<u64>(&mut args, "--stability-check-min-duration")?; }
			"--stability-check-max-duration" => { settings.stability_check_max_duration = take_arg::<u64>(&mut args, "--stability-check-max-duration")?; }
			"--stability-check-interval"     => { settings.stability_check_interval     = take_arg::<u64>(&mut args, "--stability-check-interval")?; }
			"--stability-check-look-ahead"   => { settings.stability_check_look_ahead   = take_arg::<u64>(&mut args, "--stability-check-look-ahead")?; }
			"--excess-ram-needed"            => { settings.excess_ram_needed            = take_arg::<u64>(&mut args, "--excess-ram-needed")? * 1024 * 1024; }
			_ => eprintln!("Warning: unknown argument '{arg}'"),
		}
	}
	
	match settings.run_type {
		RunType::Looped => {
			
			println!("Running autoswapoff in loop");
			run_loop(&settings);
			
		}
		RunType::Once => {
			
			println!("Running autoswapoff once");
			let result = run_once(&settings);
			if let Err(err) = result {
				eprintln!("Error encountered while performing operation: {err}");
			}
			
		}
		RunType::Help => {
			
			println!("Arguments:");
			println!("    --once                                      Runs the check and operations only once instead of looping.");
			println!("    --help | -h                                 Prints this help screen.");
			println!("    --seconds-per-run <SECS>                    Sets how frequently this should check the current swap usage. Unit is seconds, default is 30.");
			println!("    --swap-usage-needed <AMOUNT_MB>             This will not run swapoff/swapon unless the swap usage exceeds this amount. Unit is megabytes, default is 1024.");
			println!("    --stability-check-min-duration <DUR_SEC>    Once the 'swap usage' check passes, it starts tracking the ram usage for at least this long to make sure it isn't still being filled up. Unit is seconds, default is 10.");
			println!("    --stability-check-max-duration <DUR_SEC>    If the stability check lasts longer than this then the operation is aborted and the program will wait 3x this duration before doing another 'swap usage' check. Unit is seconds, default is 30.");
			println!("    --stability-check-interval <DUR_MS>         Sets how frequently the ram usage is checked during the stability check. Unit is milliseconds, default is 200.");
			println!("    --stability-check-look-ahead <DUR_SEC>      As the ram usage is tracked, an estimate is made for how full the ram will likely be several seconds later (by simply fitting a line to the tracked data), this sets how far ahead it estimates. Unit is seconds, default is 15.");
			println!("    --excess-ram-needed <AMOUNT_MB>             This will not run swapoff/swapon unless the available ram exceeds the amount of data currently stored in swap by at least this amount. This applies for both the current ram usage and predicted ram usage. Unit is megabytes, default is 1024.");
			
		}
	}
	
	Ok(())
}



pub fn run_loop(settings: &ProgramSettings) -> ! {
	loop {
		
		match run_once(settings) {
			Result::Ok(stabilization_period_exceeded) => {
				if stabilization_period_exceeded {
					thread::sleep(Duration::from_secs(settings.stability_check_max_duration * 3 - settings.seconds_per_run));
				}
			}
			Result::Err(err) => {
				eprintln!("Error encountered while performing operation: {err}");
			}
		}
		
		thread::sleep(Duration::from_secs(settings.seconds_per_run));
		
	}
}



pub type StabilizationPeriodExceeded = bool;

pub fn run_once(settings: &ProgramSettings) -> Result<StabilizationPeriodExceeded> {
	
	let (swap_used, mem_avail) = get_swap_used_and_mem_avail()?;
	if swap_used < settings.swap_usage_needed { return Ok(false); }
	println!("Detected significant swap usage (current swap used: {swap_used}), starting ram usage tracking...");
	
	let mut mem_avail_list = vec!();
	mem_avail_list.push(mem_avail);
	let tracking_start_time = Instant::now();
	let sleep_dur = Duration::from_millis(settings.stability_check_interval);
	let min_tracking_dur = Duration::from_secs(settings.stability_check_min_duration);
	let max_tracking_dur = Duration::from_secs(settings.stability_check_max_duration);
	let look_ahead_count = settings.stability_check_look_ahead as f64 * 1000.0 / settings.stability_check_interval as f64;
	
	loop {
		thread::sleep(sleep_dur);
		let (swap_used, mem_avail) = get_swap_used_and_mem_avail()?;
		mem_avail_list.push(mem_avail);
		let elapsed = tracking_start_time.elapsed();
		if elapsed >= min_tracking_dur {
			let (m, b) = fit_line(&mem_avail_list); // notice that mem_avail_list has had at least 2 push() calls before this
			let predicted_mem_avail = m * (mem_avail_list.len() as f64 + look_ahead_count) + b;
			if mem_avail > swap_used + settings.excess_ram_needed && predicted_mem_avail > (swap_used + settings.excess_ram_needed) as f64 {
				println!("Detected that ram usage has stabilized, running swapoff/swapon...");
				do_swap_off_on()?;
				println!("Done");
				return Ok(false);
			}
			if elapsed > max_tracking_dur {
				println!("Ram usage has not stabilized within allowed time frame, aborting");
				return Ok(true);
			}
		}
	}
}



pub fn fit_line(samples: &[u64]) -> (f64, f64) {
	debug_assert!(samples.len() >= 2);
	let n = samples.len() as f64;
	
	let mut sum_y = 0.0;
	let mut sum_xy = 0.0;
	
	for (x, &y) in samples.iter().enumerate() {
		let (x, y) = (x as f64, y as f64);
		sum_y += y;
		sum_xy += x * y;
	}
	
	let sum_x = n * (n - 1.0) * 0.5;
	let sum_xx = n * (n - 1.0) * (2.0 * n - 1.0) / 6.0;
	
	let denom = n * sum_xx - sum_x * sum_x;
	
	let m = (n * sum_xy - sum_x * sum_y) / denom;
	let b = (sum_y - m * sum_x) / n;
	(m, b)
}



pub fn get_swap_used_and_mem_avail() -> Result<(u64, u64)> {
	
	let meminfo = fs_read_to_string("/proc/meminfo")?;
	
	let mut mem_avail_line = None;
	let mut swap_total_line = None;
	let mut swap_free_line = None;
	for meminfo_line in meminfo.lines() {
		if meminfo_line.starts_with("MemAvailable") { mem_avail_line  = Some(meminfo_line); }
		if meminfo_line.starts_with("SwapTotal"   ) { swap_total_line = Some(meminfo_line); }
		if meminfo_line.starts_with("SwapFree"    ) { swap_free_line  = Some(meminfo_line); }
	}
	
	fn extract_amount(line: Option<&str>, name: &str) -> Result<u64> {
		let Some(line) = line else { bail!("Failed to find line \"{}\" in \"/proc/meminfo\"", name); };
		let Some(amount_str) = line.split_whitespace().nth(1) else { bail!("Failed to find amount within line \"{}\"", line); };
		amount_str.parse().with_context(|| format!("Failed to parse data in line \"{line}\" (stripped as \"{amount_str}\")"))
	}
	let mem_avail  = extract_amount(mem_avail_line , "MemAvailable")? * 1024;
	let swap_total = extract_amount(swap_total_line, "SwapTotal"   )? * 1024;
	let swap_free  = extract_amount(swap_free_line , "SwapFree"    )? * 1024;
	let swap_used = swap_total - swap_free;
	
	Ok((swap_used, mem_avail))
}



pub fn do_swap_off_on() -> Result<()> {
	
	let swaps = fs_read_to_string("/proc/swaps")?;
	let swap_devices =
		swaps
		.lines()
		.skip(1)
		.filter_map(|line| {
			line.split_whitespace().next()
		})
		.collect::<Vec<_>>();
	if swap_devices.is_empty() {
		eprintln!("Warning: no swap devices were detected! Output read from \"/proc/swaps\":\n```\n{swaps}\n```");
	}
	
	// do swap off
	for device in &swap_devices {
		println!("Running `swapoff {device}`...");
		Command::new("swapoff")
			.arg(device)
			.status()?
			.exit_ok()
			.with_context(|| format!("Failed to run command `swapoff {device}`"))?;
	}
	
	// do swap on
	for device in &swap_devices {
		println!("Running `swapon {device}`...");
		Command::new("swapon")
			.arg(device)
			.status()?
			.exit_ok()
			.with_context(|| format!("Failed to run command `swapon {device}`"))?;
	}
	
	Ok(())
}
