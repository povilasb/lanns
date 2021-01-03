use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::net::Ipv4Addr;
use std::os::unix::ffi::OsStringExt;

use hostman::{ManagedHostsFile, MatchType};
use hosts_parser::HostsFileLine;
use async_std::task;
use futures::{future, StreamExt};
use log;
use peer_discovery::{discover_peers, DiscoveryMsg, TransportProtocol};
use simple_logger::SimpleLogger;
use unwrap::unwrap;
use hostname;

const DRY_RUN: bool = false;

fn main() -> io::Result<()> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    let host_name = unwrap!(String::from_utf8(hostname::get()?.into_vec()));
    let ip = Ipv4Addr::new(1, 2, 3, 4);

    // TODO(povilas): get my ip addresses
    let mut msg = DiscoveryMsg::new(host_name.into(), TransportProtocol::Tcp, 5000);
    assert!(msg.add_addrv4(ip));

    let rx_msgs = unwrap!(discover_peers(msg));
    task::block_on(rx_msgs.for_each(|msg| {
        println!("Received: {:?}", msg);

        // TODO(povilas): extract hostname/ip from msg
        if DRY_RUN {
            // let hosts_file_body = add_host(host_name, ip);
            // println!("{}", hosts_file_body);
        }
        // write_to_hosts(&hosts_file_body);
        future::ready(())
    }));

    Ok(())
}

fn write_to_hosts(content: &str) {
    let mut whosts_proc = Command::new("/Users/povilas/projects/libredrop/lanns/whosts")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run whosts");
    {
        let stdin = whosts_proc.stdin.as_mut().expect("Failed to open whosts stdin");
        stdin.write_all(content.as_bytes()).expect("Failed to write to whosts stdin");
    }
}

fn add_host(host_name: &str, ip_addr: &str) -> String {
    let mut hosts_file = ManagedHostsFile::must_load();
    let matches = hosts_file.get_multi_match(&[host_name], &MatchType::Exact);
    if !matches.is_empty() {
        hosts_file.remove_host(host_name);
    }

    let host_line = format!("{} {} # Added by lanns", ip_addr, host_name);
    // Just to validate our own hand crafted hosts entry.
    let _ = HostsFileLine::from_string(&host_line).expect("Failed to parse host line");
    hosts_file.add_line(&host_line);

    hosts_file.to_string()
}
