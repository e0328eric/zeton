// NOTE: The main machine that develops this project is mac
#[cfg(not(target_os = "linux"))]
fn main() {}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::process;

    process::Command::new("git")
        .args(["fetch", "--prune", "origin", "main"])
        .spawn()?;
    process::Command::new("git")
        .args(["merge", "origin", "main"])
        .spawn()?;

    Ok(())
}
