use std::env;

extern crate log;
use log::{info,error,debug};

extern crate env_logger;
use env_logger::Env;

mod womp;

fn main() {

    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let arg: Option<String> = env::args().skip(1).next();
    let port_arg = arg.map(|s| s.to_string().parse::<u16>().unwrap()).unwrap();
    info!("Searching for port {:?}", port_arg);

    let proc_entry = match womp::find_process_for_port(port_arg) {

        Err(womp::WompError::TCPError(errstr)) => {
            error!("{}", errstr);
            std::process::exit(1);
        }
        Err(womp::WompError::ProcessError(errstr)) => {
            error!("{}", errstr);
            std::process::exit(2);
        }
        Ok(entry) => entry
    };

    debug!("Found process entry: {:?}", proc_entry);
    println!("cmdline: {:?}", proc_entry.cmdline().unwrap());
}
