use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    //
    #[allow(dead_code)]
    #[error("Error: {0}")]
    Error(String),

    #[error("Argument length must be greater than 0...")]
    ArgLengthIsZero,

    #[error("Argument could not be parsed to a string...")]
    ArgNotPassableToString,

    #[error("No matching processes found, name of process: pid: {pid} name: {pname}...")]
    NoMatchingProcessesFound { pid: u32, pname: String },

    #[error("Searched process list has a length of 0...")]
    ProcessListLengthIsZero,

    #[error("Attempt to kill process was unsuccessful...")]
    ProcessKillFailed,

    #[error("Unexpected error occurred...")]
    UnexpectedError,

    #[error("Threading error: {0}")]
    ThreadingError(String),
}

impl From<std::io::Error> for AppError {
    fn from(_error: std::io::Error) -> Self {
        AppError::UnexpectedError
    }
}

impl AppError {
    pub fn exit(&self) -> ! {
        // eprintln!("{}", self);
        std::process::exit(match self {
            AppError::ArgLengthIsZero => 1,
            AppError::ArgNotPassableToString => 2,
            AppError::NoMatchingProcessesFound { .. } => 3,
            AppError::ProcessListLengthIsZero => 4,
            AppError::ProcessKillFailed => 5,
            AppError::UnexpectedError => 6,
            AppError::Error(_) => 7,
            AppError::ThreadingError(_) => 8,
        });
    }
}
