# Walkmon

Read-only monitoring of the *Xiaomi KingSmith WalkingPad A1*. Periodically reads current speed, distance and step count via BLE. For now, it just prints the values to stdout, with the eventual goal of some sort of persistent logging.

## Build

Just `cargo build --release`. For cross-comping for Raspberry Pi, use [cross](https://github.com/cross-rs/cross) with:

```sh
TARGET=aarch64-unknown-linux-gnu
cross build --release --target=${TARGET}
scp target/${TARGET}/release/walkmon rpi-host:
```
