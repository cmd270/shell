use std::process::Command;
use std::fmt;

#[derive(Debug)]
pub enum ProcessError {
    ExecutionFailed(std::io::Error),
    NonUtf8Output(std::string::FromUtf8Error),
    ProcessFailed(String, i32)
}

impl std::error::Error for ProcessError {}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessError::ExecutionFailed(e) => write!(f, "Failed to execute process: {}", e),
            ProcessError::NonUtf8Output(e) => write!(f, "Process output is not valid UTF-8: {}", e),
            ProcessError::ProcessFailed(msg, code) => write!(f, "Process failed with code {}: {}", code, msg),
        }
    }
}

pub fn execute_process(executable: &str, args: Vec<&str>) -> Result<String, ProcessError> {
    let output = Command::new(executable)
        .args(args)
        .output()
        .map_err(ProcessError::ExecutionFailed)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ProcessError::ProcessFailed(
            stderr.to_string(),
            output.status.code().unwrap_or(-1)
        ));
    }

    String::from_utf8(output.stdout)
        .map_err(ProcessError::NonUtf8Output)
}
