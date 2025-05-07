use crate::{
    cli::AppArgs,
    dns_types::{Class, RecordType},
    message::Message,
};
use rand::Rng;
use std::process::ExitCode;
mod cli;
mod dns_types;
mod io;
mod message;
mod parse;


fn main() -> ExitCode {
    let args = match AppArgs::parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error parsing arguments: {e}\n\nRun with --help to see usage.");
            return ExitCode::from(1);
        }
    };

    let AppArgs {
        name,
        record_type,
        resolver,
        verbose,
    } = args;

    if record_type == RecordType::All {
        query_all_records(&name, resolver, verbose);
    } else {
        query_and_print(&name, record_type, resolver, verbose);
    }

    ExitCode::SUCCESS
}

fn query_all_records(name: &str, resolver: std::net::SocketAddr, verbose: bool) {
    use RecordType::*;
    let types = [A, Aaaa, Cname, Ns, Soa, Mx, Txt, Ptr, Srv];
    for record_type in types {
        println!("--- {} RECORDS ---", record_type);
        query_and_print(name, record_type, resolver, verbose);
    }
}

fn query_and_print(name: &str, record_type: RecordType, resolver: std::net::SocketAddr, verbose: bool) {
    let query_id = rand::thread_rng().gen();
    match Message::new_query(query_id, name.to_string(), record_type) {
        Ok(msg) => match io::send_req(msg, resolver, verbose) {
            Ok((resp, len)) => {
                if let Err(e) = io::print_resp(resp, len, query_id, verbose) {
                    eprintln!("Error: {e}");
                }
            }
            Err(e) => eprintln!("Network error: {e}"),
        },
        Err(e) => eprintln!("Failed to build query: {e}"),
    }
}