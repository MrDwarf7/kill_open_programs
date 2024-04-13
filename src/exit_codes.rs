use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct ExitReasons {
    pub reason: String,
    pub code: i32,
}

impl Display for ExitReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

#[derive(Error, Debug)]
pub enum ExitCodes {
    #[error("Error: {0}")]
    ExitCodeError(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeOk(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotFound(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeFound(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeUnknown(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeInvalid(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotImplemented(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotSupported(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotReady(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotAvailable(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotPermitted(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotAuthorized(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotValid(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutable(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableFile(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutablePath(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableCommand(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableProgram(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableScript(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableBinary(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableLibrary(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableService(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableDaemon(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableKernel(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableSystem(ExitReasons),
    #[error("Error: {0}")]
    ExitCodeNotExecutableUser(ExitReasons),
}

impl ExitCodes {
    pub fn exit_code_error(reason: String) -> Self {
        Self::ExitCodeError(ExitReasons { reason, code: 1 })
    }

    pub fn process_exit_code_error(reason: String) -> ! {
        eprintln!("{}", Self::exit_code_error(reason));
        std::process::exit(1);
    }
}
