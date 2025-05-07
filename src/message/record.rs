use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{Class, RecordType};

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Record {
    pub name: String,
    pub class: Class,
    pub ttl: u32,
    pub data: RecordData,
}


impl Record {
    pub fn as_dns_response(&self) -> String {
        let rdata = match &self.data {
            RecordData::A(ipv4) => ipv4.to_string(),
            RecordData::Aaaa(ipv6) => ipv6.to_string(),
            RecordData::Cname(name) => name.to_string(),
            RecordData::Ns(name) => name.to_string(),
            RecordData::Ptr(name) => name.to_string(),
            RecordData::Mx { preference, exchange } => format!("{} {}", preference, exchange),
            RecordData::Srv { priority, weight, port, target } => {
                format!("{} {} {} {}", priority, weight, port, target)
            }
            RecordData::Txt(txt) => txt.clone(),
            // RecordData::Soa(soa) => format!("{:?}", soa),
            RecordData::Soa(soa) => format!(
                "{} {} (serial {}, refresh {}, retry {}, expire {})",
                soa.mname, soa.rname, soa.serial, soa.refresh, soa.retry, soa.expire
            ),

            RecordData::Raw(data) => format!("RAW({} bytes)", data.len()),
        };
        format!("{}: {rdata} (TTL {})", self.data.as_type(), self.ttl)
    }
}
#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum RecordData {
    A(Ipv4Addr),
    Aaaa(Ipv6Addr),
    Cname(String),
    Ns(String),
    Ptr(String),
    Mx { preference: u16, exchange: String },
    Srv { priority: u16, weight: u16, port: u16, target: String },
    Txt(String),
    Soa(SoaData),
    Raw(Vec<u8>), // For unsupported/ALL queries
}

impl RecordData {
    pub fn as_type(&self) -> RecordType {
        match self {
            Self::A(_) => RecordType::A,
            Self::Aaaa(_) => RecordType::Aaaa,
            Self::Cname(_) => RecordType::Cname,
            Self::Ns(_) => RecordType::Ns,
            Self::Ptr(_) => RecordType::Ptr,
            Self::Mx { .. } => RecordType::Mx,
            Self::Srv { .. } => RecordType::Srv,
            Self::Txt(_) => RecordType::Txt,
            Self::Soa(_) => RecordType::Soa,
            Self::Raw(_) => RecordType::All,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct SoaData {
    /// name server that was the original or primary source of data for this zone.
    pub mname: String,
    /// mailbox of the person responsible for this zone.
    pub rname: String,
    /// The unsigned 32 bit version number of the original copy
    /// of the zone.  Zone transfers preserve this value.  This
    /// value wraps and should be compared using sequence space
    /// arithmetic.
    pub serial: u32,
    /// time interval before the zone should be refreshed.
    pub refresh: u32,
    /// time interval that should elapse before a failed refresh should be retried.
    pub retry: u32,
    /// upper limit on the time interval that can elapse before the zone is no longer authoritative.
    pub expire: u32,
}
