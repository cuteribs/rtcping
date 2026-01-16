# Windows Package Manager Manifests

This directory contains manifests and configurations for Windows package managers.

## Winget

To publish to winget, the manifest needs to be submitted to the [microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs) repository. This process is automated via GitHub Actions.

## Chocolatey

Chocolatey package specifications are in the `chocolatey` subdirectory.

## Scoop

Scoop manifest is in the `scoop` subdirectory.

## Automated Publishing

The `.github/workflows/update-windows-packages.yml` workflow automatically creates PRs or updates these packages when a new release is published.
