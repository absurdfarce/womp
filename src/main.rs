use std::env;

extern crate procfs;
use procfs::net::TcpNetEntry;
use procfs::process::FDInfo;
use procfs::process::FDTarget;
use procfs::process::Process;

extern crate partial_application;
use partial_application::partial;

fn find_tcp_entry(port_to_find:u16, entries:Vec<TcpNetEntry>) -> Option<TcpNetEntry> {
    entries.into_iter().find(|entry| entry.local_address.port() == port_to_find)
}

fn find_tcp_entry_global(port_to_find:u16) -> Result<TcpNetEntry, String> {
    match procfs::net::tcp() {

        Err(e) => Err(format!("Error retrieving TCP entries: {:?}", e)),
        Ok(entries) => match find_tcp_entry(port_to_find, entries) {
            None => Err(format!("Port {:?} appears to be unused", port_to_find)),
            Some(entry) => Ok(entry)
        }
    }
}

fn contains_socket_inode(info:&FDInfo, inode_to_find:u32) -> bool {
    match info.target {
        FDTarget::Socket(inode) => inode == inode_to_find,
        _ => false
    }
}

fn process_contains_inode(process:&Process, inode_to_find:u32) -> bool {
    match process.fd() {
        Err(e) => {
            println!("Error determining file descriptors for process {:?}, ignoring: {:?}", process.pid, e);
            false
        },
        Ok(fds) => fds.iter().any(partial!(contains_socket_inode => _, inode_to_find))
    }
}

fn find_process(inode_to_find:u32, processes:Vec<Process>, ) -> Option<Process> {
    processes.into_iter().find(partial!(process_contains_inode => _, inode_to_find))
}

fn find_process_global(inode_to_find:u32) -> Result<Process, String> {
    match procfs::process::all_processes() {

        Err(e) => { Err(format!("Error retrieving process entries: {:?}", e)) },
        Ok(procs) => match find_process(inode_to_find, procs) {
            None => Err(
                format!(
                    "Could not find process containing inode {:?}, look for errors determining file descriptors for processes",
                    inode_to_find)),
            Some(proc) => Ok(proc)
        }
    }
}

fn main() {
    let arg: Option<String> = env::args().skip(1).next();
    let port_arg = arg.map(|s| s.to_string().parse::<u16>().unwrap()).unwrap();
    println!("Searching for port {:?}", port_arg);

    let tcp_entry = match find_tcp_entry_global(port_arg) {
        Err(errstr) => {
            println!("{}", errstr);
            std::process::exit(1);
        }
        Ok(entry) => entry
    };
    println!("Found TCP entry: {:?}", tcp_entry);

    let proc_entry = match find_process_global(tcp_entry.inode) {

        Err(errstr) => {
            println!("{}", errstr);
            std::process::exit(2);
        }
        Ok(entry) => entry
    };

    println!("Found process entry: {:?}", proc_entry);
    println!("cmdline: {:?}", proc_entry.cmdline().unwrap());
}
