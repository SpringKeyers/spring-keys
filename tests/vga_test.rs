#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_vga_screen() {
        // Start the program with the test command
        let mut child = Command::new(env!("CARGO_BIN_EXE_spring-keys"))
            .arg("test")
            .spawn()
            .expect("Failed to start spring-keys");

        // Wait for a short duration to let the VGA test screen run
        thread::sleep(Duration::from_secs(2));

        // Terminate the process
        child.kill().expect("Failed to kill spring-keys process");
    }
} 