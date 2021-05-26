extern crate log;
use log::{info,error,debug};

extern crate env_logger;
use env_logger::Env;

extern crate clap;
use clap::Clap;

mod womp;

#[derive(Clap)]
struct Options {
    #[clap(short, long)]
    port:u16,
    #[clap(short = 'c', long)]
    cmdline: bool,
    #[clap(short = 'C', long)]
    cwd: bool,
    #[clap(short, long)]
    ruid: bool,
    #[clap(short, long)]
    euid: bool
}

fn main() {

    let env = Env::default()
        .filter_or("WOMP_LOG_LEVEL", "info")
        .write_style_or("WOMP_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let opts: Options = Options::parse();
    info!("Searching for port {:?}", opts.port);

    let proc_entry = match womp::find_process_for_port(opts.port) {
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
    println!("PID: {:?}", proc_entry.pid());
    if opts.cmdline {
        println!(
            "Command line: {:?}",
            proc_entry.cmdline().expect("Failed to retrieve command line for process").join(" "));
    }
    if opts.cwd {
        println!(
            "Current working directory: {:?}",
            proc_entry.cwd().expect("Failed to retrieve current working directory for process"));
    }
    if opts.ruid || opts.euid {
        let status = proc_entry.status().expect("Failed to retrieve status for process");
        if opts.ruid {
            println!("Real UID: {:?}", status.ruid);
        }
        if opts.euid {
            println!("Effective UID: {:?}", status.euid);
        }
    }
}
