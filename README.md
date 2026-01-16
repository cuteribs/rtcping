# rtcping

A simple and lightweight CLI tool written in Rust to measure TCP connection latency (RTT) to a target host and port.

## Features

- **TCP Ping**: Measures the time taken to establish a TCP connection.
- **Warmup Phase**: Includes a configurable warmup period to stabilize measurements.
- **Flexible Duration**: Specify the number of pings or a fixed duration in seconds.
- **Statistics**: Provides min/avg/max RTT and packet loss percentage.
- **IPv4/IPv6 Support**: Force resolution to either IPv4 or IPv6.
- **Ctrl+C Handling**: Gracefully interrupts and displays statistics.

## Installation

### Linux

#### Debian/Ubuntu (.deb)
```bash
# Download the .deb package from the latest release
wget https://github.com/cuteribs/rtcping/releases/latest/download/rtcping_0.1.0_amd64.deb
sudo dpkg -i rtcping_0.1.0_amd64.deb
```

#### Fedora/RHEL/CentOS (.rpm)
```bash
# Download the .rpm package from the latest release
wget https://github.com/cuteribs/rtcping/releases/latest/download/rtcping-0.1.0-1.x86_64.rpm
sudo rpm -i rtcping-0.1.0-1.x86_64.rpm
```

#### Arch Linux (AUR)
```bash
# Using yay or paru
yay -S rtcping
# Or manually
git clone https://aur.archlinux.org/rtcping.git
cd rtcping
makepkg -si
```

#### Alpine Linux
```bash
# Add community repository if needed
sudo apk add rtcping
```

#### Universal Binary
```bash
# Download and install manually
wget https://github.com/cuteribs/rtcping/releases/latest/download/rtcping-linux-x86_64
chmod +x rtcping-linux-x86_64
sudo mv rtcping-linux-x86_64 /usr/local/bin/rtcping
```

### macOS

#### Homebrew
```bash
brew tap cuteribs/rtcping
brew install rtcping
```

#### Manual Installation
```bash
# For Intel Macs
curl -LO https://github.com/cuteribs/rtcping/releases/latest/download/rtcping-macos-x86_64
chmod +x rtcping-macos-x86_64
sudo mv rtcping-macos-x86_64 /usr/local/bin/rtcping

# For Apple Silicon Macs
curl -LO https://github.com/cuteribs/rtcping/releases/latest/download/rtcping-macos-aarch64
chmod +x rtcping-macos-aarch64
sudo mv rtcping-macos-aarch64 /usr/local/bin/rtcping
```

### Windows

#### Winget
```powershell
winget install cuteribs.rtcping
```

#### Chocolatey
```powershell
choco install rtcping
```

#### Scoop
```powershell
scoop bucket add cuteribs https://github.com/cuteribs/scoop-bucket
scoop install rtcping
```

#### Manual Installation
1. Download `rtcping-windows-x86_64.exe` from the [latest release](https://github.com/cuteribs/rtcping/releases/latest)
2. Rename it to `rtcping.exe`
3. Add it to a directory in your PATH

### From Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

```bash
cargo install --git https://github.com/cuteribs/rtcping
```

Or build locally:

```bash
git clone https://github.com/cuteribs/rtcping.git
cd rtcping
cargo build --release
```

The binary will be available at `target/release/rtcping`.

## Usage

```text
rtcping [OPTIONS] <HOST> <PORT>

Arguments:
  <HOST>  Target host
  <PORT>  Target port

Options:
  -i <INTERVAL>           Ping interval in milliseconds [default: 1000]
  -n <COUNT_OR_DURATION>  Number of pings (e.g., "4") or duration (e.g., "10s") [default: 4]
  -4                      Force IPv4
  -6                      Force IPv6
  -w <WARMUP>             Warmup seconds [default: 1]
  -h, --help              Print help
  -V, --version           Print version
```

### Samples

#### Basic Ping
Ping `google.com` on port 80 for 4 iterations:
```bash
rtcping google.com 80
```

#### Custom Count
Ping 10 times:
```bash
rtcping google.com 443 -n 10
```

#### Timed Duration
Ping for 30 seconds:
```bash
rtcping 1.1.1.1 53 -n 30s
```

#### Fast Interval
Ping with a 100ms interval:
```bash
rtcping example.com 80 -i 100
```

#### Force IPv4/IPv6
```bash
rtcping google.com 443 -4
rtcping google.com 443 -6
```

## Output Example

```text
rtcping google.com (142.250.190.46) port 443
Warming up for 1 seconds...
Connected to google.com:443 time=15.42ms
Connected to google.com:443 time=14.89ms
Connected to google.com:443 time=15.12ms
Connected to google.com:443 time=14.95ms

--- google.com tcp ping statistics ---
4 packets transmitted, 4 received, 0.0% packet loss
rtt min/avg/max = 14.89ms/15.095ms/15.42ms
```

## License

This project is licensed under the MIT License.
