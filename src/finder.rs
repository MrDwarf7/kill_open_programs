use crate::exit_codes::AppError;
use crate::prelude::*;
use crate::utils::Utils;

pub struct Finder {
    pub app_name: String,
}

impl Finder {
    /// # Safety
    ///
    /// This function is unsafe because it accesses and manipulates external processes.
    pub unsafe fn find_processes(
        app_name: &str,
        collection_vec: ArcMutexVec,
    ) -> std::result::Result<ArcMutexVec, AppError> {
        let tl = tasklist::Tasklist::new();
        let target_variations = Utils::generate_variations(app_name);

        let mut filtered_tasks: Vec<Process> = Vec::new();

        tl.into_iter().for_each(|process| {
            let matched_processes = Utils::test_if_match(&process, target_variations.as_slice());
            filtered_tasks.extend(matched_processes);
        });

        filtered_tasks.drain(..).for_each(|process| {
            collection_vec.lock().push(process);
        });

        if collection_vec.lock().len() == 0 {
            return Err(AppError::NoMatchingProcessesFound {
                pid: 0,
                pname: target_variations.first().unwrap().clone(),
            });
        }

        Ok(collection_vec)
    }
}
