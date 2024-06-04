fn main() {
    // Taken from https://crates.io/crates/sdl2 (search for "rpath"):
    #[cfg(target_os="macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
}