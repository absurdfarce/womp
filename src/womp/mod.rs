extern crate procfs;
use self::procfs::net::TcpNetEntry;
use self::procfs::process::FDInfo;
use self::procfs::process::FDTarget;
use self::procfs::process::Process;

extern crate partial_application;
use self::partial_application::partial;

/**
  * This fn (like it's twin find_process_in_list()) isn't public in order to avoid exposing
  * anything that relies on crates that are local to this module as part of the module API.
  */
fn find_tcp_entry_in_list(port_to_find:u16, entries:Vec<TcpNetEntry>) -> Option<TcpNetEntry> {
    entries.into_iter().find(|entry| entry.local_address.port() == port_to_find)
}

pub fn find_tcp_entry(port_to_find:u16) -> Result<TcpNetEntry, String> {
    match procfs::net::tcp() {

        Err(e) => Err(format!("Error retrieving TCP entries: {:?}", e)),
        Ok(entries) => match find_tcp_entry_in_list(port_to_find, entries) {
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

fn find_process_in_list(inode_to_find:u32, processes:Vec<Process>, ) -> Option<Process> {
    processes.into_iter().find(partial!(process_contains_inode => _, inode_to_find))
}

/**
  * TODO: A definite design flaw here.  We don't want to expose any structs from crates that are internal
  * to this module as either args or return values.  But in order to do so here something has to get
  * the inode from the TCP entry.  We work around that by adding a pub function to handle this, but
  * that still exposes a struct from a crate local to this module as an arg.
  *
  * This is definitely not a great solution.
  */
pub fn get_inode(tcp_entry:TcpNetEntry) -> u32 {
    tcp_entry.inode
}

pub fn find_process(inode_to_find:u32) -> Result<Process, String> {
    match procfs::process::all_processes() {

        Err(e) => { Err(format!("Error retrieving process entries: {:?}", e)) },
        Ok(procs) => match find_process_in_list(inode_to_find, procs) {
            None => Err(
                format!(
                    "Could not find process containing inode {:?}, look for errors determining file descriptors for processes",
                    inode_to_find)),
            Some(proc) => Ok(proc)
        }
    }
}
