use anyhow::Result;
use parking_lot::Mutex;
use std::sync::Arc;
use tasklist::Process;

use super::Utils;

pub struct Finder {
    pub app_name: String,
}

impl Finder {
    /// # Safety
    ///
    /// This function is unsafe because it accesses and manipulates external processes.
    pub unsafe fn find_processes(
        app_name: String,
        collection_vec: Arc<Mutex<Vec<Process>>>,
    ) -> Result<Arc<Mutex<Vec<Process>>>> {
        let tl = tasklist::Tasklist::new();
        let target_variations = Utils::generate_variations(app_name);

        let mut filtered_tasks: Vec<Process> = Vec::new();

        // process is a reference to each Process struct in the tasklist.(ie: each system process)
        tl.into_iter().for_each(|process| {
            match Utils::test_if_match(&process, &target_variations) {
                Ok(matched_processes) => {
                    filtered_tasks.extend(matched_processes);
                }
                Err(e) => {
                    // NOTE: Exit here if len is 0, or if there are no processes found
                    println!("Error: {}", e);
                    println!("No processes found... in Finder as a match error")
                }
            }
        });

        println!("Filtered tasks: {:?}", filtered_tasks);

        filtered_tasks.drain(..).for_each(|process| {
            collection_vec.lock().push(process);
        });

        dbg!(&collection_vec);

        match collection_vec.lock().is_empty() {
            true => {
                // NOTE: Exit here if len is 0
                println!("No processes found... in Finder as a match error");
            }
            false => {}
        }
        Ok(collection_vec)
    }
}
