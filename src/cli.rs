use std::env;

mod womp;

fn main() {
    let arg: Option<String> = env::args().skip(1).next();
    let port_arg = arg.map(|s| s.to_string().parse::<u16>().unwrap()).unwrap();
    println!("Searching for port {:?}", port_arg);

    let proc_entry = match womp::find_process_for_port(port_arg) {

        Err(womp::WompError::TCPError(errstr)) => {
            println!("{}", errstr);
            std::process::exit(1);
        }
        Err(womp::WompError::ProcessError(errstr)) => {
            println!("{}", errstr);
            std::process::exit(2);
        }
        Ok(entry) => entry
    };

    println!("Found process entry: {:?}", proc_entry);
    println!("cmdline: {:?}", proc_entry.cmdline().unwrap());
}
