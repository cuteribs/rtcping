Name:           rtcping
Version:        0.1.0
Release:        1%{?dist}
Summary:        TCP latency measurement tool

License:        MIT
URL:            https://github.com/cuteribs/rtcping
Source0:        https://github.com/cuteribs/rtcping/archive/v%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo

%description
A TCP latency measurement tool similar to ping but for TCP connections.
Measures round-trip time for establishing TCP connections with warmup,
statistics, and IPv4/IPv6 support.

%prep
%autosetup

%build
cargo build --release --locked

%install
install -Dm755 target/release/rtcping %{buildroot}%{_bindir}/rtcping
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%files
%license LICENSE
%doc README.md
%{_bindir}/rtcping

%changelog
* Thu Jan 16 2026 cuteribs <your-email@example.com> - 0.1.0-1
- Initial package
