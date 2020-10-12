# womp

Or "who owns my port".  A semi-useful utility for determining which process is already using a port you intended to use.

You might now be asking yourself why not just use "netstat -tlp4 --numeric-ports" or any of the many options which do something similar.  That works too.  I decided to write "womp" rather than use this or some other existing command for the following reasons:

* I can never remember these commands and the necessary args; for some reason they just don't stick with me
* When I wrap them up in a shell script I can never remember where I put the shell script
* It's an excuse to play with Rust

## Compiling

Shouldn't be much more than "cargo build"

## Running

The program accepts a single arg representing the port number you're inquiring about.  So let's assume the following:

* You're already in the project root directory
* You're trying to start a Cassandra database with the default ports, specifically 9042 for the CQL port
* The database fails to start due to a port conflict

In this case the following will help you determine what process is creating your problem:

"target/debug/womp 9042"

