use std::env;

mod womp;

fn main() {
    let arg: Option<String> = env::args().skip(1).next();
    let port_arg = arg.map(|s| s.to_string().parse::<u16>().unwrap()).unwrap();
    println!("Searching for port {:?}", port_arg);

    let tcp_entry = match womp::find_tcp_entry(port_arg) {
        Err(errstr) => {
            println!("{}", errstr);
            std::process::exit(1);
        }
        Ok(entry) => entry
    };
    println!("Found TCP entry: {:?}", tcp_entry);

    let proc_entry = match womp::find_process(womp::get_inode(tcp_entry)) {

        Err(errstr) => {
            println!("{}", errstr);
            std::process::exit(2);
        }
        Ok(entry) => entry
    };

    println!("Found process entry: {:?}", proc_entry);
    println!("cmdline: {:?}", proc_entry.cmdline().unwrap());
}
