pub use crate::exit_codes::AppError;
pub use parking_lot::Mutex;
pub use std::sync::Arc;
pub use tasklist::Process;

pub type ArcMutexVec = Arc<Mutex<Vec<Process>>>;
