# OpenDiscord
## Building the binary
To build the binary you'll need the standard Rust toolchain installed. You can do that [here](https://www.rust-lang.org/tools/install)

### 1) Clone this repository
- Navigate into your desired folder
- Run `git clone https://github.com/CrumblyLiquid/OpenDiscord`

### 2) Build the actual binary
- Move into the OpenDiscord folder (`cd ./OpenDiscord`)
- Run `cargo build --release`
- You'll find the final binary in `./target/release` as `open_discord.exe` (or just `open_discord`)

### 3) Prepare for setup
 - Copy the final binary into your desired folder
 - Copy `config_example.json` to your desired folder and rename it to `config.json`

Now you're ready to continue to the [setup part](SETUP.md)!