# AutoSwapOff

This is a simple utility that runs in the background to turn swap off and on again whenever swap contents can and should be moved back to ram. This is the basic cycle of this program:

- 1: If the swap usage is under a certain amount, wait 30 seconds then check again
- 2: Once it does reach a certain amount, start tracking the amount of available ram
- 3: Wait until the ram availability is stable (minimum wait duration is 15 seconds)
- 4: Run swapoff then swapon

Some things to note:

- The ram availability is not considered stable until the amount of available ram is enough to hold all of swap's contents plus another 2GB
- While tracking the ram, it also predicts how full the ram will be 20 seconds ahead (by simply fitting a line to the collected data), and the predicted amount of ram also needs to hold all of swap + 2GB
- If the ram availability does not stabilize within 45 seconds, it waits ~2 minutes and starts again at step 1
- This will dynamically check the currently used swap devices and turns all devices off then all previous devices on
- This is linux only, and also should only be run on systems where `swapoff` is safe to run

## Installation

There's no official/good way to install this, but here's how I personally do it:

- Run `cargo build --release` on this repo
- Take the `autoswapoff` file in `/target/release` and move it to `/usr/local/bin`
- Make the file `/etc/systemd/system/autoswapoff.service` with [these contents](example_systemd.service)
- Run these commands:
  - `sudo systemctl daemon-reload`
  - `sudo systemctl enable autoswapoff`
  - `sudo systemctl start autoswapoff`
- Other useful commands:
  - Check if autoswapoff is running: `systemctl status autoswapoff`
  - See and watch autoswapoff output: `sudo journalctl -u autoswapoff -f`

Or, you can run `./install.sh` after downloading the repo

## Configuration

This is configured entirely through command-line arguments:

```
    --once                                      Runs the check and operations only once instead of looping.
    --help | -h                                 Prints this help screen.
    --seconds-per-run <SECS>                    Sets how frequently this should check the current swap usage. Unit is seconds, default is 30.
    --swap-usage-needed <AMOUNT_MB>             This will not run swapoff/swapon unless the swap usage exceeds this amount. Unit is megabytes, default is 1024.
    --stability-check-min-duration <DUR_SEC>    Once the 'swap usage' check passes, it starts tracking the ram usage for at least this long to make sure it isn't still being filled up. Unit is seconds, default is 20.
    --stability-check-max-duration <DUR_SEC>    If the stability check lasts longer than this then the operation is aborted and the program will wait 3x this duration before doing another 'swap usage' check. Unit is seconds, default is 60.
    --stability-check-interval <DUR_MS>         Sets how frequently the ram usage is checked during the stability check. Unit is milliseconds, default is 200.
    --stability-check-look-ahead <DUR_SEC>      As the ram usage is tracked, an estimate is made for how full the ram will likely be several seconds later (by simply fitting a line to the tracked data), this sets how far ahead it estimates. Unit is seconds, default is 30.
    --excess-ram-needed <AMOUNT_MB>             This will not run swapoff/swapon unless the available ram exceeds the amount of data currently stored in swap by at least this amount. This applies for both the current ram usage and predicted ram usage. Unit is megabytes, default is 2048.
```
