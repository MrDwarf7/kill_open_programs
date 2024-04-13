use anyhow::Result;
use tasklist::Process;

pub struct Utils {}

impl Utils {
    pub fn test_if_match(process: &Process, target_variations: &[String]) -> Result<Vec<Process>> {
        let mut matched_processes: Vec<Process> = Vec::new();

        match target_variations
            .iter()
            .find(|&variation| process.pname.to_lowercase().contains(variation))
        {
            Some(_) => {
                matched_processes.push(Process {
                    pid: process.pid,
                    pname: process.pname.clone(),
                });
            }
            None => {
                // Current wasn't found in the target variations, do nothing.
            }
        }

        Ok(matched_processes)
    }

    pub fn generate_variations(app_name: String) -> Vec<String> {
        vec![
            app_name.to_lowercase(),
            format!("{}{}", app_name.to_lowercase(), ".exe"),
        ]
    }
}
