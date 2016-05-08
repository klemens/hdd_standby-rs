# hdd_standby

[Documentation](https://klemens.github.io/hdd_standby-rs/hdd_standby/)

Library to check the power state of a hdd using ioctls. Currently only
supported on unix.

# Example

```rust
extern crate hdd_standby;

fn main() {
    let status = hdd_standby::get_power_state("/dev/sda");
    println!("{:?}", status.unwrap_or(PowerState::Unknown));
}
```

# Licence

This library is licenced under the terms of the MIT licence.
