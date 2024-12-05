use std::process::Command;

fn main() {
    // Run maturin build
    let output = Command::new("maturin")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to execute maturin build");

    // Print maturin output
    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
}
