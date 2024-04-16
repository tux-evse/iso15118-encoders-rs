/*
 * Copyright (C) 2015-2022 IoT.bzh Pionix, Chargebyte and Everest contributors
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Rust largely inspired from Everest C++ git@github.com:/EVerest/libiso15118.git
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *
 */

use crate::prelude::*;
use ::std::os::raw;
use std::slice;
use std::str;
use std::sync::{Mutex, MutexGuard};

#[track_caller]
pub fn dump_buffer(buffer: &[u8]) -> String {
    let mut dump = "".to_string();
    for byte in buffer {
        dump = dump + format!("{:#02x}", byte).as_str() + ",";
    }
    dump
}

#[track_caller]
pub fn dump_hexa(buffer: &[u8]) -> String {
    let mut dump = "".to_string();
    for byte in buffer {
        dump = dump + format!("{:02x}", byte).as_str();
    }
    dump
}

#[track_caller]
pub fn dump_string(buffer: &[raw::c_char]) -> String {
    unsafe { std::str::from_utf8_unchecked(std::mem::transmute(buffer)) }.to_string()
}

#[track_caller]
pub fn array_to_string(array: &[raw::c_char], len: u16) -> Result<String, AfbError> {
    let slice = unsafe { slice::from_raw_parts(array.as_ptr() as *const u8, len as usize) };
    let text = match str::from_utf8(slice) {
        Ok(value) => value,
        Err(_) => {
            return afb_error!("byte_to_string", "invalid UFT8 data");
        }
    };
    Ok(text.to_string())
}

#[derive(Clone)]
pub struct ExiByteArray<T>
where
    T: Sized,
{
    array: T,
    len: usize,
}

impl<T> ExiByteArray<T> {
    pub fn new(array: T, len: u16) -> Self
    where
        T: Sized,
    {
        Self {
            array: array,
            len: len as usize,
        }
    }

    pub fn get_array(&self) -> &T {
        &self.array
    }

    pub fn get_len(&self) -> u16 {
        self.len as u16
    }

    pub fn to_string(&self) -> Result<String, AfbError> {
        let ptr = &self.array as *const _ as *const u8;
        let slice = unsafe { slice::from_raw_parts(ptr, self.len) };
        let text = match str::from_utf8(slice) {
            Ok(value) => value,
            Err(_) => {
                return afb_error!("iso2-array-tostring", "invalid UFT8 data");
            }
        };
        Ok(text.to_string())
    }

    pub fn to_bytes(&self) -> &[u8] {
        let ptr = &self.array as *const _ as *const u8;
        let slice = unsafe { slice::from_raw_parts(ptr, self.len) };
        slice
    }

    pub fn dump_buffer(&self) -> String {
        dump_buffer(self.to_bytes())
    }

    pub fn equal(&self, array: &[u8]) -> bool {
        self.to_bytes() == array
    }
}

pub enum ExiDump {
    V2gHeader,
    IsoPayload,
    Everything,
}

pub struct ExiStream {
    raw: Mutex<RawStream>,
}

impl ExiStream {
    pub fn new() -> Self {
        ExiStream {
            raw: Mutex::new(RawStream::new()),
        }
    }
    pub fn lock_stream(&self) -> MutexGuard<RawStream> {
        self.raw.lock().unwrap()
    }

    pub fn get_buffer<'a>(&self, lock: &'a MutexGuard<RawStream>) -> &'a [u8] {
        lock.get_buffer()
    }

    pub fn dump_buffer(
        &self,
        lock: &MutexGuard<RawStream>,
        chunk: ExiDump,
    ) -> Result<String, AfbError> {
        let len = lock.get_length();
        if len < v2g::SDP_V2G_HEADER_LEN as usize {
            return afb_error!(
                "stream-buffer-dump",
                "stream size less that v2gheader len:{}",
                len
            );
        }

        let (start, stop) = match chunk {
            ExiDump::V2gHeader => (0, v2g::SDP_V2G_HEADER_LEN),
            ExiDump::IsoPayload => (v2g::SDP_V2G_HEADER_LEN, len),
            ExiDump::Everything => (0, len),
        };
        let buffer = lock.get_buffer();
        Ok(dump_buffer(&buffer[start..stop]))
    }

    pub fn get_length(&self, lock: &MutexGuard<RawStream>) -> usize {
        lock.get_length()
    }

    pub fn get_index(&self, lock: &MutexGuard<RawStream>) -> (usize, usize) {
        lock.get_index()
    }

    pub fn drop(&self) {
        let lock = self.lock_stream();
        (*lock).drop()
    }

    pub fn reset(&self, lock: &MutexGuard<RawStream>) {
        lock.reset()
    }

    // (decode only) remove header from data buffer stream to match exi decoder
    pub fn finalize(&self, lock: &MutexGuard<RawStream>, doc_size: u32) -> Result<(), AfbError> {
        match unsafe { lock.stream.as_mut() } {
            Some(data) => {
                data.data_size = doc_size as usize; // (cglue::SDP_V2G_HEADER_LEN+doc_size) as usize;
                data.byte_pos= v2g::SDP_V2G_HEADER_LEN as usize; // SDP_V2G_HEADER_LEN as usize
                data.bit_count=0;
            }
            None => return afb_error!("exi-stream-shift", "fail to shift header (invalid stream)"),
        };
        Ok(())
    }

    pub fn header_check(&self, lock: &MutexGuard<RawStream>) -> Result<u32, AfbError> {
        // check vg2tp exi message header
        let count = v2g::v2gtp_header_check(v2g::V2gTypeId::EXI_V2G_MSG, lock.buffer.as_ref())?;
        if count > EXI_MAX_DOCUMENT_SIZE as u32 {
            return afb_error!(
                "exi_header_check",
                "doc size::{} to big max:{}",
                count,
                EXI_MAX_DOCUMENT_SIZE
            );
        }
        Ok(count)
    }
}
