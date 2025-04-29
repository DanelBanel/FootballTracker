# TODO
todo

cargo 1.84.0 and rustc 1.84.0

## Build and run
```bash
cargo build # Build in debug
cargo build --release # Build in release
cargo run # Build and run
cargo update # Update crate dependencies
cargo update <package> # Update specific crate
cargo test # Runs tests
cargo test <test-name> # Run specific tests
cargo check # Check the current package
cargo clippy # Check for usual mistakes
cargo doc # Build documentation
cargo fmt # Fix format warnings
cargo fix # Fix lint warnings
```

### TODO
Special credit for inspiration @abdullahtarek

### Pre commit hooks

This repository runs several scripts and checks before a commit can be created. Check [.git-hooks](.git-hooks/) for what is actually run.

To activate the git hooks locally run this command:
```bash
git config --local core.hooksPath .git-hooks
```