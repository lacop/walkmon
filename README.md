# Walkmon

Read-only monitoring of the *Xiaomi KingSmith WalkingPad A1*. Periodically reads current speed, distance and step count via BLE. For now, it just prints the values to stdout, with the eventual goal of some sort of persistent logging.

## Build

Just `cargo build --release`. For cross-comping for Raspberry Pi, use [cross](https://github.com/cross-rs/cross) with:

```sh
TARGET=aarch64-unknown-linux-gnu
cross build --release --target=${TARGET}
scp target/${TARGET}/release/walkmon rpi-host:
```

## Demo

```console
$ ./walkmon
Scanning for Bluetooth adapters...
Scanning for WalkingPad...
Discovered device: Some(PeripheralProperties { ..snip.. })
Connecting to WalkingPad...
Reading data from WalkingPad...
Press Ctrl+C to stop.
Speed: 2.0 km/h, Time: 1:38:27, Distance: 3.280 m, Steps: 6219
Speed: 2.0 km/h, Time: 1:38:28, Distance: 3.280 m, Steps: 6220
Speed: 2.0 km/h, Time: 1:38:29, Distance: 3.280 m, Steps: 6221
^C
Stopping...
```
