//! Hi, I'm 'whosts', I do this
//!
//!    sudo echo "whatever comes from stdin" > /etc/hosts
//!
//! ... without sudo!
//!
//! I work with Linux and MacOS, maybe Windows some day as well.

use std::fs::OpenOptions;
use std::io::{self, copy};

use nix::unistd::{setuid, Uid};

const HOSTS_PATH: &str = "/etc/hosts";

fn main() {
    // Lets become root user.
    setuid(Uid::from_raw(0)).expect(
        "Failed to change UID to root. Probably 'chmod +s' has not been run on this binary",
    );

    let mut stdin = io::stdin();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(HOSTS_PATH)
        .unwrap_or_else(|e| panic!("Failed to open hosts file: {}. {}", HOSTS_PATH, e));
    copy(&mut stdin, &mut file).expect("Copying faled");
}
