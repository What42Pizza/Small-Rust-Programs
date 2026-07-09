#![allow(unused)]
#![warn(unused_must_use)]

#![feature(exit_status_error)]



use std::time::Duration;
pub use std::{path::Path, process::Command};
pub use anyhow::*;



pub mod utils;
pub use utils::*;



fn main() -> Result<()> {
	
	let mut seconds_per_run: u64 = 30;
	let mut min_swap_usage_mb: u64 = 1024;
	let mut excess_ram_needed_mb: u64 = 1024;
	let mut run_type = RunType::Looped;
	enum RunType {
		Looped,
		Once,
		Help,
	}
	
	fn get_arg_u64(args: &mut impl Iterator<Item = String>, arg_name: &str) -> Result<u64> {
		args.next()
			.ok_or_else(|| anyhow!("unexpected end of command arguments"))?
			.parse()
			.with_context(|| anyhow!("failed to parse '{}' argument", arg_name))
	}
	let mut args = std::env::args();
	let mut args = args.skip(1);
	loop {
		let Some(arg) = args.next() else { break; };
		match &*arg {
			
			"--seconds-per-run"   => { seconds_per_run = get_arg_u64(&mut args, "--seconds-per-run")?  ; }
			"--min-swap-usage"    => { min_swap_usage_mb = get_arg_u64(&mut args, "--min-swap-usage")?   ; }
			"--excess-ram-needed" => { excess_ram_needed_mb = get_arg_u64(&mut args, "--excess-ram-needed")?; }
			"--once" => run_type = RunType::Once,
			"--help" | "-h" => run_type = RunType::Help,
			
			_ => eprintln!("Warning: unknown argument '{arg}'"),
		}
	}
	
	let (min_swap_usage, excess_ram_needed) = (min_swap_usage_mb * 1024 * 1024, excess_ram_needed_mb * 1024 * 1024);
	match run_type {
		RunType::Looped => {
			
			run_loop(seconds_per_run, min_swap_usage, excess_ram_needed);
			
		}
		RunType::Once => {
			
			let result = do_check_and_operation(min_swap_usage, excess_ram_needed);
			if let Err(err) = result {
				eprintln!("Warning: encountered error while performing operation: {err}");
			}
			
		}
		RunType::Help => {
			
			println!("Arguments:");
			println!("    --seconds-per-run <SECS>            Specifies how frequently this should do its operation. Defaults to 30.");
			println!("    --min-swap-usage <AMOUNT_MB>        This will not run swapoff/swapon unless there is at least this much in swap. Default to 1024.");
			println!("    --excess-ram-needed <AMOUNT_MB>     This will not run swapoff/swapon unless the amount of free ram is enough to hold all the stored data in swap plus this amount. Defaults to 1024.");
			println!("    --once                              Runs the check and operations only once instead of looping.");
			println!("    --help | -h                         Prints this help screen.");
			
		}
	}
	
	Ok(())
}



pub fn run_loop(seconds_per_run: u64, min_swap_usage: u64, excess_ram_needed: u64) -> ! {
	println!("Running autoswapoff");
	loop {
		
		let result = do_check_and_operation(min_swap_usage, excess_ram_needed);
		if let Err(err) = result {
			eprintln!("Warning: encountered error while performing operation: {err}");
		}
		
		std::thread::sleep(Duration::from_secs(seconds_per_run));
		
	}
}



pub fn do_check_and_operation(min_swap_usage: u64, excess_ram_needed: u64) -> Result<()> {
	
	let (swap_used, mem_avail) = get_swap_and_avail_mem()?;
	if !(swap_used > min_swap_usage && mem_avail > swap_used + excess_ram_needed) { return Ok(()); }
	
	println!("Detected unideal swap usage (swap used: {swap_used}, ram available: {mem_avail}), detecting swap devices...");
	do_swap_off_on()?;
	println!("Done");
	
	Ok(())
}



pub fn get_swap_and_avail_mem() -> Result<(u64, u64)> {
	
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
