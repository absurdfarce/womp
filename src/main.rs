use std::env;
use procfs::process::FDTarget;
use procfs::process::Process;

extern crate procfs;

// What's wrong with, say, "netstat -tlp4 --numeric-ports"?
//
// Absolutely nothing.  Except I can never remember it.

fn extract_inode(target:&FDTarget) -> u32 {
   if let FDTarget::Socket(inode) = target { *inode }
   else { 0 }
}

fn contains_inode(proc_entry: &Process, target_inode:u32) -> bool {
   if let Ok(fds) = proc_entry.fd() {
      fds.iter().map(|fd| extract_inode(&fd.target)).any(|inode| inode == target_inode)
   }
   else { false }
}

fn main() {
    let arg: Option<String> = env::args().skip(1).next();
    let port_arg = arg.map(|s| s.to_string().parse::<u16>().unwrap()).unwrap();
    println!("Searching for port {:?}", port_arg);

    let all_tcp_entries = procfs::net::tcp().unwrap();
    let tcp_entry = match all_tcp_entries.iter().find(|entry| entry.local_address.port() == port_arg) {
    	Some(entry) => entry,
	None => {
	     println!("Port {:?} appears to be unused", port_arg);
	     std::process::exit(1);
	},
    };
    println!("Found TCP entry: {:?}", tcp_entry);

    let all_proc_entries = procfs::process::all_processes().unwrap();
    let proc_entry = match all_proc_entries.iter().find(|entry| contains_inode(&entry, tcp_entry.inode)) {
    	Some(entry) => entry,
	None => {
	     println!("Could not find process that owns socket inode {:?}", tcp_entry.inode);
	     std::process::exit(2);
	},
    };
    println!("Found process entry: {:?}", proc_entry);
    println!("cmdline: {:?}", proc_entry.cmdline().unwrap());
}
