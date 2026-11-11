param(
    [ValidateSet("build", "run", "test", "check")]
    [string] $Command = "build",
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $CargoArgs = @()
)

$ErrorActionPreference = "Stop"
$target = "x86_64-pc-windows-gnu"
$msys2Root = if ($env:MSYS2_ROOT) { $env:MSYS2_ROOT } else { "C:\msys64" }
$mingwBin = Join-Path $msys2Root "ucrt64\bin"

foreach ($tool in @("gcc.exe", "g++.exe", "mingw32-make.exe")) {
    if (-not (Test-Path (Join-Path $mingwBin $tool))) {
        throw "Missing $tool in $mingwBin. Install the MSYS2 UCRT64 GCC and make packages."
    }
}

if (-not (Get-Command cmake -ErrorAction SilentlyContinue)) {
    throw "CMake is required. Install CMake and make sure cmake.exe is on PATH."
}

$env:PATH = "$mingwBin;$env:PATH"
$env:CMAKE_GENERATOR = "MinGW Makefiles"
$gccInclude = (& gcc -print-file-name=include).Trim()
$mingwInclude = Join-Path $msys2Root "ucrt64\include"
$env:BINDGEN_EXTRA_CLANG_ARGS = "-I`"$gccInclude`" -I`"$mingwInclude`""
if (-not $env:LIBCLANG_PATH) {
    $llvmBin = Join-Path $env:ProgramFiles "LLVM\bin"
    if (Test-Path (Join-Path $llvmBin "libclang.dll")) {
        $env:LIBCLANG_PATH = $llvmBin
    }
}

$installedTargets = & rustup target list --installed
if ($LASTEXITCODE -ne 0) {
    throw "Could not query Rust targets. Ensure rustup is installed and available on PATH."
}
if ($installedTargets -notcontains $target) {
    & rustup target add $target
    if ($LASTEXITCODE -ne 0) {
        throw "Could not install the Rust target $target."
    }
}

& cargo $Command --target $target @CargoArgs
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}
