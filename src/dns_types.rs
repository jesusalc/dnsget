//! Common DNS types that get used in several different parts of the codebase.
use bitvec::prelude::*;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    A,
    Aaaa,
    Cname,
    Soa,
    Ns,
    Mx,
    Txt,
    Ptr,
    Srv,
    All,
}

impl FromStr for RecordType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rt = match s.to_uppercase().as_str() {
            "A" => Self::A,
            "AAAA" => Self::Aaaa,
            "CNAME" => Self::Cname,
            "SOA" => Self::Soa,
            "NS" => Self::Ns,
            "MX" => Self::Mx,
            "TXT" => Self::Txt,
            "PTR" => Self::Ptr,
            "SRV" => Self::Srv,
            "ALL" => Self::All,
            other => return Err(format!("{other} is not a valid DNS record type")),
        };
        Ok(rt)
    }
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::A => "A",
            Self::Aaaa => "AAAA",
            Self::Cname => "CNAME",
            Self::Soa => "SOA",
            Self::Ns => "NS",
            Self::Mx => "MX",
            Self::Txt => "TXT",
            Self::Ptr => "PTR",
            Self::Srv => "SRV",
            Self::All => "ALL",
        };
        s.fmt(f)
    }
}

impl RecordType {
    pub fn serialize<T: BitStore>(&self, bv: &mut BitVec<T, Msb0>) {
        let type_num: u16 = match self {
            Self::A => 1,
            Self::Ns => 2,
            Self::Cname => 5,
            Self::Soa => 6,
            Self::Ptr => 12,
            Self::Mx => 15,
            Self::Txt => 16,
            Self::Aaaa => 28,
            Self::Srv => 33,
            Self::All => 255,
        };
        bv.extend_from_bitslice(type_num.view_bits::<Msb0>())
    }
}

impl TryFrom<u16> for RecordType {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let record_type = match value {
            1 => Self::A,
            2 => Self::Ns,
            5 => Self::Cname,
            6 => Self::Soa,
            12 => Self::Ptr,
            15 => Self::Mx,
            16 => Self::Txt,
            28 => Self::Aaaa,
            33 => Self::Srv,
            255 => Self::All,
            other => anyhow::bail!("Invalid record type number {other}"),
        };
        Ok(record_type)
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum Class {
    IN,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::IN => "IN",
        };
        s.fmt(f)
    }
}

impl Class {
    pub fn serialize<T: BitStore>(&self, bv: &mut BitVec<T, Msb0>) {
        let type_num: u16 = match self {
            Self::IN => 1,
        };
        bv.extend_from_bitslice(type_num.view_bits::<Msb0>())
    }
}

impl TryFrom<u16> for Class {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let record_type = match value {
            1 => Self::IN,
            other => anyhow::bail!("Invalid class number {other}"),
        };
        Ok(record_type)
    }
}
