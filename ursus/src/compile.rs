use std::process::Command;

fn compile_project(scarb_project_path: &str) {
    // invoke a shell command to compile the program and show stdout
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("cd {}; scarb build;", scarb_project_path))
        .output()
        .expect("Failed to compile program");
    println!("stdout: {:?}", String::from_utf8_lossy(&output.stdout));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_program() {
        let scarb_project_path = "../playground";
        compile_project(scarb_project_path);
        // println!("Compiled program: {:?}", compiled_program);
    }
}
