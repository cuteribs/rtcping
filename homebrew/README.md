# Homebrew Tap for rtcping

This is the Homebrew tap for rtcping.

## Installation

```bash
brew tap cuteribs/rtcping
brew install rtcping
```

## For Maintainers

### Initial Setup

1. Create this repository as `homebrew-rtcping` under your GitHub account
2. Add the Formula file (Formula/rtcping.rb)
3. When releasing a new version, the formula will be automatically updated by the GitHub Action

### Manual Formula Update

If you need to manually update the formula:

```bash
brew bump-formula-pr --url=https://github.com/cuteribs/rtcping/archive/refs/tags/vX.X.X.tar.gz rtcping
```
