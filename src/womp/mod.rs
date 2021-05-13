extern crate procfs;
use self::procfs::net::TcpNetEntry;
use self::procfs::process::FDInfo;
use self::procfs::process::FDTarget;
use self::procfs::process::Process;

extern crate partial_application;
use self::partial_application::partial;
use womp::WompError::{TCPError, ProcessError};

pub enum WompError {
    /* An error observed when evaluating TCP info */
    TCPError(String),
    /* An error observed wen evaluating process info */
    ProcessError(String)
}

fn find_tcp_entry_in_vec(port:u16, entries:Vec<TcpNetEntry>) -> Option<TcpNetEntry> {
    entries.into_iter().find(|entry| entry.local_address.port() == port)
}

fn find_tcp_entry(port:u16) -> Result<TcpNetEntry, WompError> {
    match procfs::net::tcp() {

        Err(e) => Err(TCPError(format!("Error retrieving TCP entries: {:?}", e))),
        Ok(entries) => match find_tcp_entry_in_vec(port, entries) {
            None => Err(TCPError(format!("Port {:?} appears to be unused", port))),
            Some(entry) => Ok(entry)
        }
    }
}

fn contains_socket_inode(info:&FDInfo, inode:u32) -> bool {
    match info.target {
        FDTarget::Socket(socket_inode) => socket_inode == inode,
        _ => false
    }
}

fn process_contains_inode(process:&Process, inode:u32) -> bool {
    match process.fd() {
        Err(e) => {
            /* Something of a special case here.  We're looping through a number of procs to see if they
             * have an inode matching our target but if we can't get fds for a given process we don't
             * necessarily want everything to fail.  For now we just log that fact via println!
             *
             * TODO: A more robust answer would be great here.  It's also the case that this function
             * shouldn't be deciding how to log things... it really should be passing this upstream and
             * letting higher-level functions do the routing of errors as necessary. */
            println!("Error determining file descriptors for process {:?}, ignoring: {:?}", process.pid, e);
            false
        },
        Ok(fds) => fds.iter().any(partial!(contains_socket_inode => _, inode))
    }
}

fn find_process_in_vec(inode:u32, processes:Vec<Process>, ) -> Option<Process> {
    processes.into_iter().find(partial!(process_contains_inode => _, inode))
}

fn find_process(inode:u32) -> Result<Process, WompError> {
    match procfs::process::all_processes() {

        Err(e) => { Err(ProcessError(format!("Error retrieving process entries: {:?}", e))) },
        Ok(procs) => match find_process_in_vec(inode, procs) {
            None => Err(
                ProcessError(
                    format!(
                        "Could not find process containing inode {:?}, look for errors determining file descriptors for processes",
                        inode))),
            Some(proc) => Ok(proc)
        }
    }
}

pub fn find_process_for_port(port:u16) -> Result<Process, WompError> {
    let tcp_entry = find_tcp_entry(port)?;
    //println!("Found TCP entry: {:?}", tcp_entry);
    let proc_entry = find_process(tcp_entry.inode)?;
    Ok(proc_entry)
}