# mping
A cross-platform multi-target ping tool that supports ICMP and TCP protocols and can monitor the connectivity of multiple target servers at the same time.

## Features

- Support ping multiple target servers at the same time
- Support ICMP and TCP protocols
- Support real-time display or hiding of ping results
- Support saving results to specified log files
- Support background operation
- Support custom ping interval and number
- Gracefully handle Ctrl+C interruption
- Provide detailed statistics
- Cross-platform support (Windows/Linux)

## System requirements

### Windows
- Windows 7 or higher
- If using ICMP protocol, administrator privileges are required

### Linux
- Kernel version 2.6 or higher
- If using ICMP protocol, root privileges or set `CAP_NET_RAW` capability are required

## Installation

### Compile from source code

```bash
# Clone repository
git clone https://github.com/laishane/mping.git
cd mping

# Compile
cargo build --release

# The compiled executable file is located in target/release/mping
```

### Cross-compile

#### Linux to Windows (x86_64)
```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu
# Install MinGW-w64
sudo apt-get install mingw-w64 # Ubuntu/Debian
sudo dnf install mingw64-gcc # Fedora

# Compile
cargo build --target x86_64-pc-windows-gnu --release
```

#### Windows to Linux (x86_64)
```powershell
# Install Linux target
rustup target add x86_64-unknown-linux-gnu
# Install LLVM and Clang
choco install llvm

# Compile
cargo build --target x86_64-unknown-linux-gnu --release
```

## Usage

### Basic usage
```bash
# Windows (Administrator privileges)
mping.exe -t www.google.com www.apple.com

# Linux (root privileges)
sudo ./mping -t www.google.com www.apple.com
```

### Command line parameters

```
Options:
-t, --targets <TARGETS>... List of target servers to ping
-d, --display Display real-time results in the terminal [Default: true]
-l, --log <LOG> Log file save path
-b, --background Run in the background [Default: false]
-p, --protocol <PROTOCOL> Protocol to use [Optional: icmp, tcp] [Default: icmp]
-i, --interval <INTERVAL> Ping interval (seconds) [Default: 1]
-c, --count <COUNT> Total number of pings [Default: 0 (infinite)]
-h, --help Show help information
```

### Usage examples

1. Ping multiple targets using ICMP protocol:
```bash
# Windows
mping.exe -t www.google.com www.apple.com

# Linux
sudo ./mping -t www.google.com www.apple.com
```

2. Use TCP protocol (no privileges required):
```bash
mping -t www.google.com -p tcp
```

3. Run in the background, ping every 5 seconds, a total of 100 times:
```bash
mping -t www.google.com -b -i 5 -c 100 -l ping.log
```

4. Do not display real-time results, only save to the log:
```bash
mping -t www.google.com -d false -l ping.log
```

### Log format

The log file uses CSV format and contains the following fields:
```
Destination, Timestamp, Status, RTT (ms)
```

Example:
```
www.google.com,2024-03-20 10:30:15,Success,45
```

Statistics will be included at the end:
```
--- Final Statistics ---
Statistics for www.google.com:
Packets: Sent = 100, Received = 98, Lost = 2 (2% loss)
Round Trip Times: Min = 42ms, Max = 128ms, Avg = 56ms
--- End of Statistics ---
```

## Linux Privilege Settings

If you do not want to run ICMP ping with root privileges, you can set the capability:

```bash
sudo setcap cap_net_raw+ep ./target/release/mping
```

## Notes

1. Using ICMP protocol on Linux system requires root privileges or appropriate capabilities

2. When using TCP protocol, the target port 80 will be used by default

3. When running in the background, the statistics will be saved in the log file

4. The program only responds to Ctrl+C signal and will not be affected by other terminal operations

## Contribution

Welcome to submit Issues and Pull Requests!

## License

This project uses the MIT license - see the [LICENSE](LICENSE) file for details