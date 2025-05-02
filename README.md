# FootballTracker
An application written in Rust that uses machine learning, computer vision and deep learning to create a football analysis system

## TODO list
- fix pre-commit hook for cargo fmt, cargo fix? See if there is one good online

## Requirements and installation
https://doc.rust-lang.org/cargo/index.html
### Crates
https://crates.io/crates/object-detection-opencv-rust
https://crates.io/crates/opencv/0.66.0


### Models
From the *ultralytics* [GitHub](https://github.com/ultralytics/ultralytics) you can download different models in *.pt* format. To use the models in Rust we need to convert them to *onnx* format. I created a small Python script that does this:

YOLO11n

```bash
wget https://github.com/ultralytics/assets/releases/download/<version>/<model> -O assets/models/yolov8n.pt # Or download it from github manually
pip install pip-tools
python -m piptools compile requirements.in
py -3.11 -m venv venv # Requires python3.11 runtime
.\venv\Scripts\Activate # In powershell
pip install -r requirements.txt # May require a --force-reinstall
python convert_pt_to_onnx.py # After moving the .pt file to assets/models/
```
I used python3.11  for this, as the python package *onnx* does not behave well with 3.13.

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

#### Get OpenCV dll file
Follow the instructions in issue thread above. In the build stage, `build.rs` moves the file `*.dll` to same place as the executable (`target/debug`). Therefore you have to move that file to `assets/` and the build script will automatically move it to the correct place. It is in `.gitignore` as one does not want to push those files.

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