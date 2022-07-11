## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Install rendering dependencies:
  - On Ubuntu: `sudo apt install libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev`

## Useage

`LED_CONTROLLER=$PI_IP_AND_PORT RUST_LOG="nannou_led_controller=trace" cargo run --release`
