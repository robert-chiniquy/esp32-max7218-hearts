# Setup

ESP pre-reqs: https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file#prerequisites

# Usage

```sh
. ../export-esp.sh
cargo run
# or, cargo build && cargo +esp espflash flash -M
```

# usb note

`/dev/cu.usbserial-210` is the RHS USB-C on my laptop

# origin

From: https://github.com/esp-rs/esp-idf-template?tab=readme-ov-file

```sh
cargo generate esp-rs/esp-idf-template cargo
```
