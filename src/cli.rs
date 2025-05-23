use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    process::exit,
};

use ascii::AsciiString;

use crate::dns_types::RecordType;

const HELP: &str = "\
dnsget -- domain information gatherer, obviously
USAGE:
  dnsget [OPTIONS] --record-type TYPE NAME
FLAGS:
  -h, --help                Prints help information
  -v, --verbose             Enable verbose output
OPTIONS:
  -t, --record-type TYPE    Choose the DNS record type (A, AAAA, CNAME, SOA, NS, MX, TXT, PTR, SRV, or ALL)
  -r, --resolver IP         Which DNS resolver to query (default is 1.1.1.1:53)
ARGS:
  NAME A domain name to look up. Remember, these must be ASCII.
";


/// Values derived from the CLI arguments.
#[derive(Debug)]
pub struct AppArgs {
    pub record_type: RecordType,
    pub name: String,
    pub resolver: SocketAddr,
    pub verbose: bool,
}

impl AppArgs {
    pub fn parse() -> Result<Self, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();

        if pargs.contains(["-h", "--help"]) {
            print!("{}", HELP);
            std::process::exit(0);
        }

        let record_type = match pargs
            .opt_value_from_str("--record-type")?
            .xor(pargs.opt_value_from_str("-t")?)
        {
            Some(rt) => rt,
            None => {
                eprintln!("You must supply exactly one of either -t or --record-type");
                print!("{}", HELP);
                std::process::exit(1);
            }
        };

        let default_resolver = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 1, 1, 1), 53));
        let resolver = pargs
            .opt_value_from_str("--resolver")?
            .or(pargs.opt_value_from_str("-r")?)
            .unwrap_or(default_resolver);

        let mut name: String = pargs.free_from_str()?;
        use std::str::FromStr;
        if AsciiString::from_str(&name).is_err() {
            eprintln!("DNS names must be ASCII, and {name} is not.");
            exit(1);
        }
        if !name.ends_with('.') {
            name.push('.');
        }

        let verbose = pargs.contains(["-v", "--verbose"]);

        let args = AppArgs {
            record_type,
            name,
            resolver,
            verbose,
        };

        let remaining = pargs.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unused arguments left: {:?}.", remaining);
        }

        Ok(args)
    }
}