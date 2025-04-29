# FootballTracker
An application written in Rust that uses machine learning, computer vision and deep learning to create a football analysis system

## TODO list
  - can I move opencv_world4110.dll to same place as executable in build.rs? By placing the dll at root (or assets)

## Requirements and installation
https://doc.rust-lang.org/cargo/index.html
### Crates
https://crates.io/crates/object-detection-opencv-rust
https://crates.io/crates/opencv/0.66.0

### Installation
This was done on Windows 10:

Using chocolatey
https://chocolatey.org/
```bash
choco install llvm opencv
```
Relevant links for troubleshooting:
[https://blog.devgenius.io/rust-and-opencv-bb0467bf35ff](https://blog.devgenius.io/rust-and-opencv-bb0467bf35ff)
[https://github.com/twistedfall/opencv-rust](https://github.com/twistedfall/opencv-rust)
[https://github.com/twistedfall/opencv-rust/issues/118#issuecomment-619608278](https://github.com/twistedfall/opencv-rust/issues/118#issuecomment-619608278)

Installing `vcpkg`, documentation here:
[https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-powershell](https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-powershell)

Installing `cmake`, documentation here:
[https://cmake.org/download/](https://cmake.org/download/)

Installing `pkg-config `, documentation here:
[https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html#pkg-config](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html#pkg-config)


Move `opencv_world4110.dll` (or you equivalent) to same place as executable (`target/debug`)

### Environment variables

#### OpenCV
According to issue thread linked above, add these new environment variables:

```bash
OPENCV_LINK_LIBS = opencv_world4110
OPENCV_LINK_PATHS = C:\tools\opencv\build\x64\vc16\lib
OPENCV_INCLUDE_PATHS = C:\tools\opencv\build\include
```
#### vcpkg
Add this new environment variable:
```bash
VCPKG_ROOT = "C:\path\to\vcpkg"
```
and add `C:\path\to\vcpkg` to PATH

#### pkg-config 
Add this new environment variable:
```bash
PKG_CONFIG_PATH = "C:\gnome\lib\pkgconfig"
```
and add `C:\pkg-config-lite-0.28-1\bin` and `C:\gnome\bin` to PATH