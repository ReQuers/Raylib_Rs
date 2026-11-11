# Raylib_Rs

A small desktop calculator written in Rust with a Raylib GUI. The Rust code is
portable across Windows and Linux; Raylib itself is native C code, so building
the application also requires a C toolchain and the platform's windowing and
graphics development libraries.

## Requirements

- 64-bit Windows 10/11 or a 64-bit Linux desktop.
- Rust stable **1.88 or newer**, installed with rustup. The Windows build
  script installs the GNU target automatically.
- CMake and LLVM's **libclang** library. `raylib-sys` uses CMake to build
  Raylib and libclang to generate Rust bindings from Raylib's C headers.
- A C/C++ compiler and the native development libraries listed below.

### Windows 10/11 (x64)

Install the prerequisites with winget (or their official installers), then
reopen PowerShell so the updated `PATH` is loaded:

```powershell
winget install --id Rustlang.Rustup --exact
winget install --id MSYS2.MSYS2 --exact
winget install --id Kitware.CMake --exact
winget install --id LLVM.LLVM --exact
```

Open the MSYS2 **UCRT64** terminal and install MinGW-w64:

```sh
pacman -S --needed mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-make
```

LLVM must include `libclang.dll`. If it is installed in a non-standard
location, set `LIBCLANG_PATH` to the directory containing that file. In
PowerShell, from the project directory:

```powershell
.\build.ps1
```

The script selects the `x86_64-pc-windows-gnu` Rust target and MinGW CMake
generator, so this build does not use Visual Studio Build Tools. MSYS2 defaults
to `C:\msys64`; set `MSYS2_ROOT` if it is installed elsewhere.

### Linux (Debian/Ubuntu)

Install Rust with rustup:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install the compiler, Raylib/GLFW development dependencies, and libclang:

```sh
sudo apt update
sudo apt install build-essential cmake pkg-config libclang-dev \
  libasound2-dev libgl1-mesa-dev \
  libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev
```

### Fedora (dnf)

Install the equivalent development packages:

```sh
sudo dnf install gcc gcc-c++ make cmake clang-devel alsa-lib-devel \
  mesa-libGL-devel libX11-devel libXrandr-devel libXinerama-devel \
  libXcursor-devel libXi-devel
```

### Build and run on Linux

From the project directory, build with Cargo:

```sh
cargo build
```

Or use the helper script for the same commands as on Windows:

```sh
bash ./build.sh
```

This project currently builds Raylib with its X11 desktop backend. On a
Wayland desktop, XWayland must be available to run the application. Other Linux
distributions need equivalent packages for GCC/G++, CMake, libclang, ALSA,
OpenGL/Mesa, and X11 development headers.

If bindgen cannot locate libclang, set `LIBCLANG_PATH` to the directory
containing the library (`libclang.so` on Linux, `libclang.dll` on Windows).

## Build, run, and test

The helper scripts accept `build` (the default), `run`, `test`, or `check`:

```powershell
.\build.ps1
.\build.ps1 run
.\build.ps1 test
.\build.ps1 check
```

```sh
bash ./build.sh
bash ./build.sh run
bash ./build.sh test
bash ./build.sh check
```

On Linux, the equivalent commands are also available directly: `cargo build`,
`cargo run`, `cargo test`, and `cargo check`. The first build compiles Raylib
and may take longer than subsequent builds. `cargo clean` removes build
artifacts, so the next build recompiles them.

## Project layout

- `src/main.rs` - Raylib window, input handling, and drawing.
- `src/calculator.rs` - expression parsing and calculator tests.
- `Cargo.toml` / `Cargo.lock` - Rust package settings and locked dependency
  versions.
- `build.ps1` / `build.sh` - platform-specific build helpers.

Edit the Rust source files, then use `run` to launch the app or `test` to run
the calculator tests. The GUI needs an active desktop session; tests do not
open a window.
