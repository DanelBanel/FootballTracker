use copy_to_output::copy_to_output;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Re-runs script if any files in res are changed
    let assets_folder = Path::new("assets/");
    println!("cargo:rerun-if-changed=assets/*.dll");

    let dll_file = fs::read_dir(assets_folder)
        .expect("Failed to read assets folder")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| path.extension().and_then(|s| s.to_str()) == Some("dll"))
        .expect("No .dll file found in the folder, check README for more information");

    let dll_path_str = dll_file.to_str().expect("Path is not valid UTF-8");
    println!("cargo:warning=Found DLL at {}", dll_file.display());
    copy_to_output(dll_path_str, &env::var("PROFILE").unwrap()).expect("Could not copy");
}
