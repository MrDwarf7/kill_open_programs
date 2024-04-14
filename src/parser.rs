use crate::exit_codes::AppError;

#[derive(Debug, Clone)]
pub struct Parser {
    pub args: Vec<String>,
}

impl Parser {
    pub fn new(mut args: Vec<String>) -> Self {
        args.remove(0);

        match args.len() {
            0 => {
                AppError::ArgLengthIsZero.exit();
            }
            _ => Self { args },
        }
    }

    pub fn parse_arg(&mut self) {
        self.args.iter_mut().for_each(|a| {
            a.parse::<String>()
                .map_err(|_e| {
                    AppError::ArgNotPassableToString.exit();
                })
                .unwrap();
        });
    }
}
