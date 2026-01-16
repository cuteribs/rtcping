# Plan: Publish rtcping to Major Package Managers

Setting up automated publishing to apt, dnf, pacman, rpm, apk, winget, brew, Chocolatey, and scoop with GitHub Actions workflows and comprehensive installation documentation for the rtcping TCP latency tool.

## Steps

1. **Update project metadata** in Cargo.toml with actual repository URL, authors, description, keywords, and categories for package manager discovery.

2. **Create GitHub release workflow** (.github/workflows/release.yml) that builds binaries for Linux (x64, ARM64), macOS (x64, ARM64), and Windows, then creates GitHub releases with artifacts on version tags.

3. **Setup Homebrew publishing** by creating a tap repository and Formula file, with GitHub Action to auto-update the formula on new releases using `brew bump-formula-pr`.

4. **Setup Windows package managers** by adding GitHub Actions to automatically submit/update manifests to `microsoft/winget-pkgs` (winget), maintain Chocolatey package, and manage Scoop manifest in a custom bucket.

5. **Setup Linux package managers** by creating packaging files and automated submissions for:
   - Debian/Ubuntu: .deb packages via `cargo-deb`, submit to PPA or similar
   - Fedora/RHEL: .rpm packages via `cargo-rpm` or FPM, submit to Copr or similar
   - Arch Linux: PKGBUILD template for AUR, with automation to publish/update
   - Alpine Linux: APKBUILD for Alpine apk, with Alpine repository publishing
   - Generic rpm: Additional support for rpm-based distros

6. **Update README.md** with new installation sections organized by platform:
   - **Linux**: apt, dnf, pacman, rpm, apk (with distro-specific commands)
   - **macOS**: brew (with cask or formula)
   - **Windows**: winget, Chocolatey, scoop
   - **Universal**: cargo (source)
   Each section includes prerequisites and clear copy-paste commands.

7. **Signing and verification**: GitHub release verification will be used for artifact integrity. No manual GPG signing required.