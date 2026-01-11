# TCP Ping Tool in Rust
Create a simple CLI tool to measure TCP connection latency to a target host and port.

## Proposed Changes
[Component Name] RTCP-PING

[NEW] main.rs
- Implement CLI argument parsing using `clap`.
  - Arguments: `<host>`, `<port>`.
  - `-i` (interval in ms, default 1000ms, 0 for fast ping).
  - `-n`: Number of pings (default 4). If it ends with `s` (e.g., `10s`), it specifies duration in seconds.
  - `-4`: Force IPv4.
  - `-6`: Force IPv6.
  - `-w`: Warmup seconds (default 1).
- Implement a loop that:
  - Creates a TCP connection to the target.
  - Measures the time taken to establish the connection.
  - Closes the connection.
  - Displays the RTT.
- Implement graceful shutdown with Ctrl+C using the `ctrlc` crate.
- Display statistics (min/max/avg RTT, loss percentage).

[MODIFY] Cargo.toml
- Add dependencies: `clap`, `ctrlc`.

[New] README.md

## Verification Plan
- Automated Tests
  - Run `cargo test` once tests are implemented.
  - Run `cargo run -- <host> <port>` to verify basic functionality.
- Manual Verification
  - Test against a known open port (e.g., `google.com 80`).
  - Test against a closed port to ensure timeout handling works.
  - Test Ctrl+C to ensure statistics are displayed.

## CI/CD
Create GitHub Actions workflow to build the app for Windows/macOS/linux and publish to GitHub Releases.