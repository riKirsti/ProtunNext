#requires -Version 7.2
param(
    [string]$OutputDirectory = (Join-Path $PSScriptRoot '../target/windows-package'),
    [string]$BuildDirectory = (Join-Path $PSScriptRoot '../target/release')
)
$ErrorActionPreference = 'Stop'
$releaseRoot = [IO.Path]::GetFullPath($BuildDirectory)
$nativeLibrary = Join-Path $releaseRoot 'protun_next.dll'
if (!(Test-Path -LiteralPath $nativeLibrary)) { throw 'Build protun_next.dll first or set BuildDirectory' }
$expectedMachine = 0x8664 # native Windows x64 only
function Assert-PackageArchitecture([byte[]]$Bytes) {
    if ($Bytes.Length -lt 64 -or $Bytes[0] -ne 0x4d -or $Bytes[1] -ne 0x5a) { throw 'Invalid PE image' }
    $pe = [BitConverter]::ToInt32($Bytes, 0x3c)
    if ($pe -lt 0 -or $pe -gt $Bytes.Length - 6 -or [BitConverter]::ToUInt32($Bytes, $pe) -ne 0x00004550) { throw 'Invalid PE header' }
    if ([BitConverter]::ToUInt16($Bytes, $pe + 4) -ne $expectedMachine) { throw 'Package architecture does not match PE image' }
}
Assert-PackageArchitecture ([IO.File]::ReadAllBytes($nativeLibrary))
$packageRoot = [IO.Path]::GetFullPath($OutputDirectory)
[IO.Directory]::CreateDirectory($packageRoot) | Out-Null
$archiveUrl = 'https://www.wintun.net/builds/wintun-0.14.1.zip'
# Official release archive, additionally checked with Windows Authenticode.
$archiveHash = '07C256185D6EE3652E09FA55C0B673E2624B565E02C4B9091C79CA7D2F24EF51'
$client = [Net.Http.HttpClient]::new()
try { $archiveBytes = $client.GetByteArrayAsync($archiveUrl).GetAwaiter().GetResult() }
finally { $client.Dispose() }
$actualHash = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($archiveBytes))
if ($actualHash -ne $archiveHash) { throw 'Official Wintun archive hash mismatch' }
$memory = [IO.MemoryStream]::new($archiveBytes, $false)
$archive = [IO.Compression.ZipArchive]::new($memory, [IO.Compression.ZipArchiveMode]::Read)
function Read-PackageEntry([string]$Name) {
    $entry = $archive.GetEntry($Name)
    if ($null -eq $entry) { throw "Missing Wintun package entry: $Name" }
    $stream = $entry.Open()
    $copy = [IO.MemoryStream]::new()
    try { $stream.CopyTo($copy); return ,$copy.ToArray() }
    finally { $stream.Dispose(); $copy.Dispose() }
}
$verificationFile = Join-Path $packageRoot ('.wintun-verify-' + [Guid]::NewGuid().ToString('N') + '.dll')
try {
    $driver = Read-PackageEntry 'wintun/bin/amd64/wintun.dll'
    Assert-PackageArchitecture $driver
    [IO.File]::WriteAllBytes($verificationFile, $driver)
    $signature = Get-AuthenticodeSignature -LiteralPath $verificationFile
    if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notmatch '(^|,\s*)O=WireGuard LLC(,|$)') {
        throw 'Wintun does not have a valid WireGuard LLC Authenticode signature'
    }
    $destination = Join-Path $packageRoot 'wintun.dll'
    if (Test-Path -LiteralPath $destination) {
        $expected = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($driver))
        if ((Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash -ne $expected) {
            throw 'Output directory contains a different wintun.dll; choose an empty package directory'
        }
    } else { [IO.File]::WriteAllBytes($destination, $driver) }
    [IO.File]::WriteAllBytes((Join-Path $packageRoot 'wintun-LICENSE.txt'), (Read-PackageEntry 'wintun/LICENSE.txt'))
    foreach ($name in @('protun_next.dll', 'protun-next-cli.exe')) {
        $source = Join-Path $releaseRoot $name
        if (Test-Path -LiteralPath $source) {
            Assert-PackageArchitecture ([IO.File]::ReadAllBytes($source))
            Copy-Item -LiteralPath $source -Destination (Join-Path $packageRoot $name)
        }
    }
    Write-Output "Prepared native x64 development package in $packageRoot; no driver loaded or installed"
} finally {
    $archive.Dispose()
    $memory.Dispose()
    # Only this invocation's named verification file is removed, never a directory.
    if (Test-Path -LiteralPath $verificationFile) { Remove-Item -LiteralPath $verificationFile }
}
