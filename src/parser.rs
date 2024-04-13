use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Parser {
    pub args: Vec<String>,
}

impl Parser {
    pub fn new(mut args: Vec<String>) -> Self {
        args.remove(0);

        match args.len() {
            0 => {
                // NOTE:: Exit here if no arguments are provided
                println!("No arguments provided...");
                std::process::exit(1);
            }
            _ => Self { args },
        }
    }

    pub fn parse_arg(&mut self) -> Result<()> {
        self.args.iter_mut().for_each(|a| {
            a.parse::<String>()
                .map_err(|e| {
                    // NOTE: Exit here if there is an error parsing the arguments
                    println!("Error: {}", e);
                    println!("Error parsing arguments...")
                })
                .unwrap();
        });
        Ok(())
    }
}
