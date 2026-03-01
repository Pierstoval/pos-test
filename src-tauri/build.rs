use std::path::Path;
use std::process::Command;

fn main() {
    // ── App version from release-name file ──────────────────────────────────
    let release_name_path = Path::new("../release-name");

    let version = if release_name_path.exists() {
        std::fs::read_to_string(release_name_path)
            .expect("Failed to read release-name file")
            .trim()
            .to_string()
    } else {
        let output = Command::new("git")
            .args(["describe", "--tags", "--abbrev=0"])
            .output()
            .expect("Failed to run git describe");
        let tag = String::from_utf8(output.stdout)
            .expect("Invalid UTF-8 from git describe")
            .trim()
            .to_string();
        let dev_version = format!("{tag}-dev");
        std::fs::write(release_name_path, &dev_version)
            .expect("Failed to write release-name file");
        dev_version
    };

    println!("cargo:rustc-env=APP_VERSION={version}");
    println!("cargo:rerun-if-changed=../release-name");

    // ── OS & architecture ───────────────────────────────────────────────────
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let os = target_os.as_str();
    println!("cargo:rustc-env=APP_OS={os}");

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let arch = target_arch.as_str();
    println!("cargo:rustc-env=APP_ARCH={arch}");

    tauri_build::build()
}
