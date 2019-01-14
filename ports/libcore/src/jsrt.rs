use crate::{check, errors::*};
use std::process::Command;

pub fn read_output(code: &str) -> Result<String> {
    if !check_node() {
        return Err(err_msg("please install Node.js: https://nodejs.org"));
    }
    let output = Command::new("node").arg("-e").arg(code).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(err_msg(format!(
            "javascript execution failed, {}",
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

fn check_node() -> bool {
    check::exec_succeed("node", &["-v"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_output() {
        let code = r#"
        console.log('Hello world!');
        "#;
        let output = read_output(code).unwrap();
        assert_eq!(output, "Hello world!");
    }
}
