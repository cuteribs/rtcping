# Release Setup Guide

This guide explains how to set up automated publishing for rtcping to various package managers.

## Prerequisites

Before the automated workflows can function, you need to set up several external repositories and GitHub secrets.

## GitHub Secrets Required

Add these secrets to your GitHub repository settings (Settings → Secrets and variables → Actions):

- `TAP_GITHUB_TOKEN`: Personal access token with repo permissions for the Homebrew tap
- `WINGET_TOKEN`: Token for submitting to winget (see winget-releaser docs)
- `SCOOP_TOKEN`: Personal access token for the Scoop bucket repository
- `CHOCOLATEY_API_KEY`: API key from Chocolatey.org
- `AUR_TOKEN`: Token for AUR repository access
- `ALPINE_TOKEN`: Token for Alpine aports repository

## External Repositories Setup

### 1. Homebrew Tap

Create a new repository named `homebrew-rtcping` on GitHub:

```bash
# Create new repo on GitHub, then:
git clone https://github.com/cuteribs/homebrew-rtcping.git
cd homebrew-rtcping
mkdir Formula
cp ../rtcping/homebrew/Formula/rtcping.rb Formula/
cp ../rtcping/homebrew/README.md .
git add .
git commit -m "Initial Homebrew formula"
git push
```

### 2. Scoop Bucket

Create a repository named `scoop-bucket`:

```bash
git clone https://github.com/cuteribs/scoop-bucket.git
cd scoop-bucket
mkdir bucket
cp ../rtcping/windows-packages/scoop/rtcping.json bucket/
git add .
git commit -m "Initial Scoop manifest"
git push
```

### 3. AUR Package

For Arch User Repository, you need to:

1. Create an account on https://aur.archlinux.org/
2. Set up SSH keys for AUR
3. Create the package repository:

```bash
git clone ssh://aur@aur.archlinux.org/rtcping.git aur-rtcping
cd aur-rtcping
cp ../rtcping/linux-packages/arch/PKGBUILD .
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Initial import"
git push
```

### 4. Alpine aports

For Alpine Linux, you'll need to:

1. Fork the official aports repository or create your own
2. Add the APKBUILD to the appropriate section:

```bash
git clone https://github.com/cuteribs/alpine-aports.git
cd alpine-aports
mkdir -p testing/rtcping
cp ../rtcping/linux-packages/alpine/APKBUILD testing/rtcping/
git add testing/rtcping
git commit -m "testing/rtcping: new aport"
git push
```

## First Release

To create your first release:

1. Update version in `Cargo.toml`
2. Commit the version change
3. Create and push a git tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This will trigger all the workflows:
- Build binaries for all platforms
- Create GitHub release with artifacts
- Update Homebrew formula
- Update Windows package managers
- Build Linux packages

## Manual Steps

Some package managers require manual submission for the first release:

### Winget

The first submission to winget needs to be done manually or reviewed:
1. Fork microsoft/winget-pkgs
2. Create a manifest in manifests/c/cuteribs/rtcping/0.1.0/
3. Submit a PR

After the first release, the automation will handle updates.

### Chocolatey

For the first Chocolatey package:
1. Create an account on https://community.chocolatey.org/
2. Generate an API key
3. The workflow will automatically push updates

### Debian PPA / RPM Repositories

For official Debian/Ubuntu and Fedora repositories, packages need to go through their respective review processes. The workflows build .deb and .rpm files that can be:
- Hosted on your own repository
- Submitted to official repositories
- Distributed via GitHub Releases

## Testing Releases

Before tagging a release, test the workflows:

1. Create a pre-release manually
2. Upload test binaries
3. Verify checksums
4. Test installation on each platform

## Troubleshooting

### Workflow Failures

Check the Actions tab on GitHub for detailed error logs.

### Permission Issues

Ensure all tokens have the necessary permissions:
- Repo access for tap/bucket repositories
- Write permissions for package registries
- SSH access for AUR

### Binary Build Failures

- Linux ARM64: Requires cross-compilation setup
- macOS: Universal binaries may need additional configuration
- Windows: Ensure MSVC toolchain is available

## Maintenance

### Updating Versions

1. Update Cargo.toml version
2. Commit changes
3. Create new tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
4. Workflows will automatically handle the rest

### Updating Workflows

To modify automation:
1. Edit workflow files in `.github/workflows/`
2. Test changes with a pre-release
3. Monitor Actions tab for results

### Deprecating Old Versions

Most package managers handle version updates automatically. For manual cleanup:
- Delete old GitHub release assets if needed
- Homebrew maintains version history automatically
- AUR and Alpine keep git history

## Security Considerations

- All binaries are built via GitHub Actions for transparency
- Checksums are generated and published with releases
- No credentials are stored in the repository
- All secrets use GitHub's encrypted secrets feature
