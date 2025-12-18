fn main() {
    let channel = version_check::Channel::read().expect("Failed to read rustc channel");

    // Allow for the #[cfg(nightly)] config
    println!("cargo::rustc-check-cfg=cfg(nightly)");

    if channel.is_nightly() {
        // Enable the #[cfg(nightly)] config
        println!("cargo:rustc-cfg=nightly");
    }
}
