// use tasklist::Process;
use crate::prelude::Process;

pub struct Utils {}

impl Utils {
    pub fn test_if_match(process: &Process, target_variations: &[String]) -> Vec<Process> {
        let mut matched_processes: Vec<Process> = Vec::new();

        if target_variations
            .iter()
            .any(|variation| process.pname.to_lowercase().contains(variation))
        {
            matched_processes.push(Process {
                pid: process.pid,
                pname: process.pname.clone(),
            });
        }
        matched_processes
    }

    pub fn generate_variations(app_name: &str) -> Vec<String> {
        vec![
            app_name.to_lowercase(),
            format!("{}{}", app_name.to_lowercase(), ".exe"),
        ]
    }
}
