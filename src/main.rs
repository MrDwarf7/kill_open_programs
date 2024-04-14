extern crate anyhow;
extern crate crossbeam;
extern crate tasklist;
extern crate thiserror;

mod exit_codes;
mod finder;
mod parser;
mod prelude;
mod utils;

use anyhow::Result;
use finder::Finder;
use parser::Parser;
use prelude::{
    AppError,
    //
    Arc,
    Mutex,
};

fn main() -> Result<()> {
    let mut parser = Parser::new(std::env::args().collect::<Vec<String>>());
    parser.parse_arg()?;

    let (tx, rx) = crossbeam::channel::unbounded();
    let process_list = Arc::new(Mutex::new(Vec::new()));

    unsafe {
        crossbeam::thread::scope(|s| {
            let tx = tx.clone();
            let process_list = Arc::clone(&process_list);

            parser.args.iter().for_each(|arg| {
                let tx = tx.clone();
                let process_list_clone = Arc::clone(&process_list);

                s.spawn(move |_| {
                    match Finder::find_processes(arg.to_string(), process_list_clone) {
                        Ok(processes) => {
                            tx.send(processes).unwrap();
                            drop(tx);
                        }
                        Err(e) => {
                        }
                    }
                });
            });
        })
        .unwrap();

        crossbeam::thread::scope(|s| {
            let rx = rx.clone();
            let process_list = Arc::clone(&process_list);
            let proc_list_len = process_list.lock().len();

            match proc_list_len {
                0 => {
                }
                _ => {

                    s.spawn(move |_| {
                        let processes = rx.recv().unwrap();


                        for process in processes.lock().iter() {
                            tasklist::kill(process.pid);
                        }
                    });
                }
            }
        })
        .unwrap();
    }
    Ok(())
}
