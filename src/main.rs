use std::env;
use procfs::net::TcpNetEntry;
use procfs::process::FDTarget;
use procfs::process::Process;

extern crate procfs;

#[macro_use]
extern crate partial_application;

// What's wrong with, say, "netstat -tlp4 --numeric-ports"?
//
// Absolutely nothing.  Except I can never remember it.

fn find_tcp_entry_by_port(entries:Vec<TcpNetEntry>, port_to_find:u16) -> Option<TcpNetEntry> {
    entries.into_iter().find(|entry| entry.local_address.port() == port_to_find)
}

enum ProcSearchResults {
    Found(Process),
    MissingSearchedSome,
    // We were able to search the file descriptors for every process but somehow didn't find the
    // expected inode.  Logically possible but hard to imagine how this would happen in practice.
    MissingSearchedAll,
}

fn find_proc_entry_by_inode(entries:Vec<Process>, inode_to_find:u32) -> ProcSearchResults {
    let mut fail_flag = false;
    match entries.into_iter().find(|entry| {
        match entry.fd() {
            Err(e) => {
                println!("Error determining file descriptors for process {:?}, ignoring: {:?}", entry.pid, e);
                fail_flag = true;
                false
            },
            Ok(fds) => {
                fds.iter().any(|fd| {
                    match fd.target {
                        FDTarget::Socket(inode) => inode == inode_to_find,
                        _ => false
                    }
                })
            }
        }
    }) {
        Some(entry) => ProcSearchResults::Found(entry),
        None => if fail_flag { ProcSearchResults::MissingSearchedSome } else { ProcSearchResults::MissingSearchedAll }
    }
}

fn main() {
    let arg: Option<String> = env::args().skip(1).next();
    let port_arg = arg.map(|s| s.to_string().parse::<u16>().unwrap()).unwrap();
    println!("Searching for port {:?}", port_arg);

    let tcp_entry = match procfs::net::tcp()
        .map(partial!(find_tcp_entry_by_port => _, port_arg)) {

        Err(e) => {
            println!("Error retrieving TCP entries: {:?}", e);
	    std::process::exit(2);
        },
            Ok(opt) => match opt {
                None => {
	            println!("Port {:?} appears to be unused", port_arg);
                    std::process::exit(1);
                },
                Some(entry) => entry
            }
	};
    println!("Found TCP entry: {:?}", tcp_entry);

    let inode = tcp_entry.inode;
    let proc_entry = match procfs::process::all_processes()
        .map(partial!(find_proc_entry_by_inode => _, inode)) {

        Err(e) => {
            println!("Error retrieving process entries: {:?}", e);
            std::process::exit(3);
        },
            Ok(ProcSearchResults::Found(entry)) => entry,
            Ok(ProcSearchResults::MissingSearchedSome) => {
	        println!("Could not find process that owns socket inode {:?}.  Could not search all processes; consult stdout for additional info", inode);
	        std::process::exit(4);
            },
            Ok(ProcSearchResults::MissingSearchedAll) => {
	        println!("Could not find process that owns socket inode {:?}.  We searched all processes... and that's kinda weird", inode);
	        std::process::exit(5);
            }
        };

    println!("Found process entry: {:?}", proc_entry);
    println!("cmdline: {:?}", proc_entry.cmdline().unwrap());
}
