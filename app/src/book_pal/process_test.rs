use std::process::Command;

pub fn process_test() {
    let command = Command::new("rustc").arg("--version").output().unwrap_or_else(| e| {
            panic!("failed to execute process: {}", e)
        }
    );
    if command.status.success() {
        let str = String::from_utf8_lossy(&command.stdout);
        println!("rustc version: {}", str);
    } else {
        let str = String::from_utf8_lossy(&command.stderr);
        println!("rustc failed and stderr was:\n{}", str);
    }
}