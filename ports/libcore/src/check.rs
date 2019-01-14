use std::process::Command;

pub fn exec_succeed(program: &str, args: &[&str]) -> bool {
    let mut cmd = Command::new(program);
    for arg in args {
        cmd.arg(*arg);
    }
    match cmd.status() {
        Ok(status) => status.success(),
        Err(_e) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_succeed() {
        assert_eq!(true, exec_succeed("node", &["-v"]));
        assert_eq!(true, exec_succeed("ebook-convert", &["--version"]));
        // test failed
        assert_eq!(false, exec_succeed("node", &["-V"]));
    }
}
