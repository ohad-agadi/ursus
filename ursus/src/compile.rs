use std::process::Command;

fn compile_project(scarb_project_path: &str) {
    // invoke a shell command to compile the program and show stdout
    // set asdf scarb version
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd {} && scarb --profile release build",
            scarb_project_path
        ))
        .output()
        .expect("Failed to compile program");
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("Finished `release`") {
        println!("Successfully compiled program");
    } else {
        panic!("Failed to compile program");
    }
}

fn execute_program(scarb_project_path: &str) {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd {} && scarb execute",
            scarb_project_path
        ))
        .output()
        .expect("Failed to execute program");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_program() {
        let scarb_project_path = "../playground";
        compile_project(scarb_project_path);
    }
}
