//! Doing network IO and printing to the terminal.
use crate::message::{header::ResponseCode, Message, MAX_UDP_BYTES};
use anyhow::{anyhow, Result as AResult};
use std::{
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

/// Sends the given DNS message to the given resolver.
/// Returns the binary response.
pub fn send_req(msg: Message, resolver: SocketAddr, verbose: bool) -> AResult<(Vec<u8>, usize)> {
    // Connect to the DNS resolver
    let local_addr = "0.0.0.0:0";
    let socket = UdpSocket::bind(local_addr).expect("couldn't bind to a local address");
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    if verbose {
        println!("Bound to local {}", socket.local_addr()?);
    }
    socket
        .connect(resolver)
        .expect("couldn't connect to the DNS resolver");
    if verbose {
        println!("Connected to remote {resolver}");
    }

    // Send the DNS resolver the message
    let body = msg.serialize_bytes()?;
    if verbose {
        println!("Request size: {} bytes", body.len());
    }
    let bytes_sent = socket.send(&body).expect("couldn't send data");
    if bytes_sent != body.len() {
        panic!("Only {bytes_sent} bytes, message was probably truncated");
    }

    // Get the resolver's response.
    // Note, you have to actually allocate space to write into.
    // I was originally using an empty vector, but reading into an empty vector always
    // instantly succeeds (by writing nothing), so I was discarding the response.
    // See <https://users.rust-lang.org/t/empty-response-from-udp-recv-w-tokio-and-futures/20241/2>
    let mut response_buf = vec![0; MAX_UDP_BYTES];
    match socket.recv(&mut response_buf) {
        Ok(received) => Ok((response_buf, received)),
        Err(e) => Err(anyhow!("recv function failed: {:?}", e)),
    }
}

/// Parse the binary response into a DNS message, and print it nicely.
pub fn print_resp(resp: Vec<u8>, len: usize, sent_query_id: u16, verbose: bool) -> AResult<()> {
    if verbose {
        println!("Response size: {len} bytes");
        println!("Raw response bytes: {resp:?}");
    }

    let input = resp[..len].to_vec();
    let response_msg = Message::deserialize(input).map_err(|e| anyhow!("Parsing error: {e}"))?;

    if response_msg.header.id != sent_query_id {
        eprintln!(
            "Warning: Mismatched query IDs. Sent {sent_query_id}, received {}",
            response_msg.header.id
        );
    }

    match response_msg.header.resp_code {
        ResponseCode::NoError => {}
        err => return Err(anyhow!("Error from resolver: {err}")),
    };

    println!("\nQuestions:");
    for q in &response_msg.question {
        println!("{q}");
    }

    if response_msg.answer.is_empty()
        && response_msg.authority.is_empty()
        && response_msg.additional.is_empty()
    {
        println!("No DNS records returned.");
        return Ok(());
    }

    if !response_msg.answer.is_empty() {
        println!("Answer Records:");
        for r in &response_msg.answer {
            println!("{}", r.as_dns_response());
        }
    }

    if verbose {
        if !response_msg.authority.is_empty() {
            println!("Authority Records:");
            for r in &response_msg.authority {
                println!("{}", r.as_dns_response());
            }
        }

        if !response_msg.additional.is_empty() {
            println!("Additional Records:");
            for r in &response_msg.additional {
                println!("{}", r.as_dns_response());
            }
        }
    }

    Ok(())
}
