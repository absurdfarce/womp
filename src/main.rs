use std::env;
use procfs::process::FDInfo;
use procfs::process::FDTarget;

extern crate procfs;

// What's wrong with, say, "netstat -tlp4 --numeric-ports"?
//
// Absolutely nothing.  Except I can never remember it.

fn extract_inode(arg:&FDTarget) -> u32 {
   if let FDTarget::Socket(inode) = arg { *inode }
   else { 0 }
}

fn contains_inode(fds: &Vec<FDInfo>, target_inode:u32) -> bool {
   fds.iter().map(|fd| extract_inode(&fd.target)).any(|inode| inode == target_inode)
}

fn main() {
    let arg: Option<String> = env::args().skip(1).next();
    let port_arg = arg.map(|s| s.to_string().parse::<u16>().unwrap()).unwrap();
    println!("Searching for port {:?}", port_arg);

    let all_tcp_entries = procfs::net::tcp().unwrap();
    let tcp_entry = all_tcp_entries.iter().filter(|entry| entry.local_address.port() == port_arg).next().unwrap();
    println!("Found TCP entry: {:?}", tcp_entry);

    let all_proc_entries = procfs::process::all_processes().unwrap();
    let proc_entry = all_proc_entries.iter().filter(|entry| contains_inode(&entry.fd().unwrap(), tcp_entry.inode)).next().unwrap();
    println!("Found proc entry: {:?}", proc_entry);
}
