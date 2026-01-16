$ErrorActionPreference = 'Stop'

$packageName = 'rtcping'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64 = 'https://github.com/cuteribs/rtcping/releases/download/v0.1.0/rtcping-windows-x86_64.exe'
$checksum64 = 'PLACEHOLDER_SHA256'

$packageArgs = @{
  packageName   = $packageName
  fileFullPath  = "$toolsDir\rtcping.exe"
  url64bit      = $url64
  checksum64    = $checksum64
  checksumType64= 'sha256'
}

Get-ChocolateyWebFile @packageArgs
