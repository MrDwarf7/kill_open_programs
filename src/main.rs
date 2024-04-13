extern crate anyhow;
extern crate crossbeam;
extern crate tasklist;
extern crate thiserror;

mod finder;
mod parser;
mod utils;

// TODO: Implement bubble up for error codes & have main call run_app() to handle the error codes.
//
// mod exit_codes;
// use exit_codes::ExitCodes;

use finder::Finder;
use parser::Parser;
use utils::Utils;

use anyhow::Result;
use parking_lot::Mutex;
use std::sync::Arc;

fn main() -> Result<()> {
    let mut parser = Parser::new(std::env::args().collect::<Vec<String>>());
    parser.parse_arg()?;

    dbg!(parser.args.clone());
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
                            println!("Found processes: {:?}", processes.lock());
                            tx.send(processes).unwrap();
                            drop(tx);
                        }
                        Err(e) => {
                            // NOTE: Exit here if len is 0, or if there are no processes found
                            println!("Error: {}", e);
                            println!("Error in the Finder::find_processes() function...")
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
                    // NOTE: Exit here if len is 0
                    println!("No processes found... in main length check");
                }
                _ => {
                    let cur_thread = std::thread::current();
                    dbg!(cur_thread.id());

                    s.spawn(move |_| {
                        let processes = rx.recv().unwrap();
                        println!("RX received data from the TX channel...");

                        let cur_thread = std::thread::current();
                        dbg!(cur_thread.id());

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
