fn main() {
    let output = std::process::Command::new("python3-config")
        .args(["--ldflags", "--embed"])
        .output()
        .expect("Failed to run python3-config");
    
    if output.status.success() {
        let flags = String::from_utf8_lossy(&output.stdout);
        for flag in flags.split_whitespace() {
            if let Some(path) = flag.strip_prefix("-L") {
                println!("cargo:rustc-link-search=native={}", path);
            } else if let Some(lib) = flag.strip_prefix("-l") {
                println!("cargo:rustc-link-lib={}", lib);
            } else if flag == "-framework" {
            } else if let Some(framework) = flag.strip_prefix("-framework") {
                println!("cargo:rustc-link-arg=-framework");
                println!("cargo:rustc-link-arg={}", framework);
            }
        }
    }
}

