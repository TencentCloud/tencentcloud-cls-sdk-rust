// Automatically generated rust module for 'log.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;

use bytes::Bytes;
use quick_protobuf::{BytesReader, MessageInfo, MessageRead, MessageWrite, Result, Writer, WriterBackend};
use quick_protobuf::sizeofs::*;

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Log<'a> {
    pub time: i64,
    pub contents: Vec<cls_log::mod_Log::Content<'a>>,
    pub collectTime: Option<i64>,
}

impl<'a> MessageRead<'a> for Log<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.time = r.read_int64(bytes)?,
                Ok(18) => msg.contents.push(r.read_message::<cls_log::mod_Log::Content>(bytes)?),
                Ok(24) => msg.collectTime = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Log<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.time) as u64)
        + self.contents.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.collectTime.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.time))?;
        for s in &self.contents { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.collectTime { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

pub mod mod_Log {
    use std::borrow::Cow;
    
    use super::*;
    
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Content<'a> {
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> Content<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        Self {
            key: Cow::Borrowed(key),
            value: Cow::Borrowed(value)
        }
    }
}

impl<'a> MessageRead<'a> for Content<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Content<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.key).len())
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.key))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LogTag<'a> {
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for LogTag<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LogTag<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.key).len())
        + 1 + sizeof_len((&self.value).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.key))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.value))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LogGroup<'a> {
    pub logs: Vec<cls_log::Log<'a>>,
    pub contextFlow: Option<Cow<'a, str>>,
    pub filename: Option<Cow<'a, str>>,
    pub source: Option<Cow<'a, str>>,
    pub logTags: Vec<cls_log::LogTag<'a>>,
    pub hostname: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for LogGroup<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.logs.push(r.read_message::<cls_log::Log>(bytes)?),
                Ok(18) => msg.contextFlow = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.filename = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.source = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.logTags.push(r.read_message::<cls_log::LogTag>(bytes)?),
                Ok(50) => msg.hostname = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LogGroup<'a> {
    fn get_size(&self) -> usize {
        0
        + self.logs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.contextFlow.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.filename.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.source.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.logTags.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.hostname.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.logs { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.contextFlow { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.filename { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        for s in &self.logTags { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.hostname { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LogGroupList<'a> {
    pub logGroupList: Vec<cls_log::LogGroup<'a>>,
}

impl<'a> LogGroupList<'a> {
    pub fn encode(&self) -> Result<Bytes> {
        let mut buf = Vec::with_capacity(self.get_size());
        let mut writer = Writer::new(&mut buf);
        self.write_message(&mut writer)?;
        Ok(Bytes::from(buf))
    }
}

impl<'a> MessageRead<'a> for LogGroupList<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.logGroupList.push(r.read_message::<cls_log::LogGroup>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LogGroupList<'a> {
    fn get_size(&self) -> usize {
        0
        + self.logGroupList.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.logGroupList { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

