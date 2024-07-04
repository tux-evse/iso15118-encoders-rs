/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

//use afbv4::prelude::*;
use crate::prelude::*;
use core::slice;
use std::alloc;
use std::boxed::Box;
use std::mem;
use std::os::raw;
use std::pin::Pin;
use std::str;
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumString};


mod cglue {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!("_exi-capi.rs");
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[allow(non_camel_case_types)]
pub enum PkiErrorStatus {
    MAKE_DIGEST = cglue::isox_sign_status_t_isox_sign_ERROR_MAKE_DIGEST,
    ERROR_DIGEST = cglue::isox_sign_status_t_isox_sign_ERROR_DIGEST,
    DIGEST_LENGTH = cglue::isox_sign_status_t_isox_sign_ERROR_DIGEST_LENGTH,
    DIGEST_MISMATCH = cglue::isox_sign_status_t_isox_sign_ERROR_DIGEST_MISMATCH,
    NOT_SINGLE_SIGNED = cglue::isox_sign_status_t_isox_sign_ERROR_NOT_SINGLE_SIGNED,
    NOT_AUTHORIZATION_REQ = cglue::isox_sign_status_t_isox_sign_ERROR_NOT_AUTHORIZATION_REQ,
    NOT_METERING_RECEIPT_REQ = cglue::isox_sign_status_t_isox_sign_ERROR_NOT_METERING_RECEIPT_REQ,
    NOT_PAYEMENT_DETAIL_REQ = cglue::isox_sign_status_t_isox_sign_ERROR_NOT_PAYEMENT_DETAIL_REQ,
    NOT_METERING_CONFIRMATION_REQ =
        cglue::isox_sign_status_t_isox_sign_ERROR_NOT_METERING_CONFIRMATION_REQ,
    NOT_PNC_AUTHORIZATION_REQ = cglue::isox_sign_status_t_isox_sign_ERROR_NOT_PNC_AUTHORIZATION_REQ,
    NO_SIGNATURE = cglue::isox_sign_status_t_isox_sign_ERROR_NO_SIGNATURE,
    NO_CHALLENGE = cglue::isox_sign_status_t_isox_sign_ERROR_NO_CHALLENGE,
    CHALLENGE_SIZE = cglue::isox_sign_status_t_isox_sign_ERROR_CHALLENGE_SIZE,
    CHALLENGE_MISMATCH = cglue::isox_sign_status_t_isox_sign_ERROR_CHALLENGE_MISMATCH,
    BAD_SIGNATURE = cglue::isox_sign_status_t_isox_sign_ERROR_BAD_SIGNATURE,
    CERT_IMPORT = cglue::isox_sign_status_t_isox_sign_ERROR_CERT_IMPORT,
    SUBCERT_IMPORT = cglue::isox_sign_status_t_isox_sign_ERROR_SUBCERT_IMPORT,
    ROOTCERT_OPEN = cglue::isox_sign_status_t_isox_sign_ERROR_ROOTCERT_OPEN,
    ROOTCERT_READ = cglue::isox_sign_status_t_isox_sign_ERROR_ROOTCERT_READ,
    ROOTCERT_OVERFLOW = cglue::isox_sign_status_t_isox_sign_ERROR_ROOTCERT_OVERFLOW,
    ROOTCERT_IMPORT = cglue::isox_sign_status_t_isox_sign_ERROR_ROOTCERT_IMPORT,
    SUBJECT_CN = cglue::isox_sign_status_t_isox_sign_ERROR_SUBJECT_CN,
    EMAID_MISMATCH = cglue::isox_sign_status_t_isox_sign_ERROR_EMAID_MISMATCH,
    TOO_MANY_CERT = cglue::isox_sign_status_t_isox_sign_ERROR_TOO_MANY_CERT,
    INVALID_CERT = cglue::isox_sign_status_t_isox_sign_ERROR_INVALID_CERT,
    SIGN_FAILED = cglue::isox_sign_status_t_isox_sign_ERROR_SIGN_FAILED,
    ERROR_INTERNAL1 = cglue::isox_sign_status_t_isox_sign_ERROR_INTERNAL1,
    ERROR_INTERNAL2 = cglue::isox_sign_status_t_isox_sign_ERROR_INTERNAL2,
    ERROR_INTERNAL3 = cglue::isox_sign_status_t_isox_sign_ERROR_INTERNAL3,
    ERROR_INTERNAL4 = cglue::isox_sign_status_t_isox_sign_ERROR_INTERNAL4,
    ERROR_INTERNAL5 = cglue::isox_sign_status_t_isox_sign_ERROR_INTERNAL5,
    ERROR_INTERNAL6 = cglue::isox_sign_status_t_isox_sign_ERROR_INTERNAL6,
    ERROR_INTERNAL7 = cglue::isox_sign_status_t_isox_sign_ERROR_INTERNAL7,
    ERROR_INTERNAL8 = cglue::isox_sign_status_t_isox_sign_ERROR_INTERNAL8,
    ERROR_INTERNAL9 = cglue::isox_sign_status_t_isox_sign_ERROR_INTERNAL9,
}

impl PkiErrorStatus {
    pub fn from_u32(value: u32) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => return afb_error!("pki-error-status", "deserialize({}):{}", json, error),
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

// reexport some C type in order to share them without all capi modules
#[allow(non_camel_case_types)]
pub(crate) type exi_bitstream_t = cglue::exi_bitstream_t;

#[derive(Debug)]
#[allow(non_camel_case_types, dead_code)]
#[repr(i32)]
pub enum ExiErrorCode {
    NO_ERROR = cglue::EXI_ERROR__NO_ERROR as i32,
    BITSTREAM_OVERFLOW = cglue::EXI_ERROR__BITSTREAM_OVERFLOW,
    HEADER_COOKIE_NOT_SUPPORTED = cglue::EXI_ERROR__HEADER_COOKIE_NOT_SUPPORTED,
    HEADER_OPTIONS_NOT_SUPPORTED = cglue::EXI_ERROR__HEADER_OPTIONS_NOT_SUPPORTED,
    SUPPORTED_MAX_OCTETS_OVERRUN = cglue::EXI_ERROR__SUPPORTED_MAX_OCTETS_OVERRUN,
    OCTET_COUNT_LARGER_THAN_TYPE_SUPPORTS = cglue::EXI_ERROR__OCTET_COUNT_LARGER_THAN_TYPE_SUPPORTS,
    UNKNOWN_EVENT_FOR_DECODING = cglue::EXI_ERROR__UNKNOWN_EVENT_FOR_DECODING,
    DECODER_NOT_IMPLEMENTED = cglue::EXI_ERROR__DECODER_NOT_IMPLEMENTED,
    UNKNOWN_EVENT_FOR_ENCODING = cglue::EXI_ERROR__UNKNOWN_EVENT_FOR_ENCODING,
    ENCODER_NOT_IMPLEMENTED = cglue::EXI_ERROR__ENCODER_NOT_IMPLEMENTED,
    BIT_COUNT_LARGER_THAN_TYPE_SIZE = cglue::EXI_ERROR__BIT_COUNT_LARGER_THAN_TYPE_SIZE,
    BYTE_COUNT_LARGER_THAN_TYPE_SIZE = cglue::EXI_ERROR__BYTE_COUNT_LARGER_THAN_TYPE_SIZE,
    ARRAY_OUT_OF_BOUNDS = cglue::EXI_ERROR__ARRAY_OUT_OF_BOUNDS,
    CHARACTER_BUFFER_TOO_SMALL = cglue::EXI_ERROR__CHARACTER_BUFFER_TOO_SMALL,
    BYTE_BUFFER_TOO_SMALL = cglue::EXI_ERROR__BYTE_BUFFER_TOO_SMALL,
    UNKNOWN_GRAMMAR_ID = cglue::EXI_ERROR__UNKNOWN_GRAMMAR_ID,
    UNKNOWN_EVENT_CODE = cglue::EXI_ERROR__UNKNOWN_EVENT_CODE,
    UNSUPPORTED_SUB_EVENT = cglue::EXI_ERROR__UNSUPPORTED_SUB_EVENT,
    DEVIANTS_NOT_SUPPORTED = cglue::EXI_ERROR__DEVIANTS_NOT_SUPPORTED,
    STRINGVALUES_NOT_SUPPORTED = cglue::EXI_ERROR__STRINGVALUES_NOT_SUPPORTED,
    UNSUPPORTED_INTEGER_VALUE_TYPE = cglue::EXI_ERROR__UNSUPPORTED_INTEGER_VALUE_TYPE,
    UNSUPPORTED_DATETIME_TYPE = cglue::EXI_ERROR__UNSUPPORTED_DATETIME_TYPE,
    UNSUPPORTED_CHARACTER_VALUE = cglue::EXI_ERROR__UNSUPPORTED_CHARACTER_VALUE,
    INCORRECT_END_FRAGMENT_VALUE = cglue::EXI_ERROR__INCORRECT_END_FRAGMENT_VALUE,
    NOT_IMPLEMENTED_YET = cglue::EXI_ERROR__NOT_IMPLEMENTED_YET,
}

pub const EXI_MAX_DOCUMENT_SIZE: usize = cglue::EXI_MAX_DOCUMENT_SIZE;

#[track_caller]
pub fn str_to_array(src: &str, dst: &mut [raw::c_char], max: u32) -> Result<u16, AfbError> {
    if src.len() > max as usize {
        return afb_error!("str-to-array", "fail (src:{} longer than:{})", src, max);
    }

    for idx in 0..src.len() {
        dst[idx] = src.as_bytes()[idx] as raw::c_char;
    }
    Ok(src.len() as u16)
}

#[track_caller]
pub fn str_equal_array(src: &str, data: &[raw::c_char], len: u32) -> bool {
    if src.len() != len as usize {
        return false;
    }
    for idx in 0..src.len() {
        if src.as_bytes()[idx] != data[idx] as u8 {
            return false;
        }
    }
    return true;
}

#[track_caller]
pub fn bytes_to_array(src: &[u8], dst: &mut [u8], max: u32) -> Result<u16, AfbError> {
    if src.len() > max as usize {
        return afb_error!("byte-to-array", "fail (src:{:?} longer than:{})", src, max);
    }

    for idx in 0..src.len() {
        dst[idx] = src[idx];
    }
    Ok(src.len() as u16)
}

#[track_caller]
pub fn byte_equal_array(src: &[u8], data: &[u8], len: u16) -> bool {
    if src.len() != len as usize {
        return false;
    }
    for idx in 0..src.len() {
        if src[idx] != data[idx] {
            return false;
        }
    }
    return true;
}


pub fn array_to_str<'a>(data: &'a [raw::c_char], len: u16) -> Result<&'a str, AfbError> {
    let slice = unsafe { slice::from_raw_parts(data.as_ptr() as *const u8, len as usize) };
    let text = match str::from_utf8(slice) {
        Ok(value) => value,
        Err(_) => return afb_error!("array_to_str", "not a valid UTF string"),
    };
    Ok(text)
}

pub fn array_to_bytes<'a>(data: &'a [u8], len: u16) -> &'a [u8] {
    let slice = unsafe { slice::from_raw_parts(data.as_ptr() as *const u8, len as usize) };
    slice
}

impl ExiErrorCode {
    pub fn from_i32(code: i32) -> Self {
        unsafe { mem::transmute(code) }
    }
}

pub struct RawStream {
    pub buffer: Pin<Box<[u8; cglue::EXI_MAX_DOCUMENT_SIZE]>>,
    pub stream: *mut cglue::exi_bitstream_t,
    layout: alloc::Layout,
    pub current_len: u32,
    pub expected_len: u32,
}

#[no_mangle]
pub extern "C" fn exi_stream_cb(message_id: i32, status_code: i32, value_1: i32, value_2: i32) {
    afb_log_msg!(
        Debug,
        None,
        "message_id:{} status_code:{} val1:{} val2{}",
        message_id,
        status_code,
        value_1,
        value_2
    );
}

#[allow(dead_code)]
impl RawStream {
    /// reserve stream memory space
    pub fn new() -> Self {
        // allocate exi_bitstream_t in heap and freeze it
        let layout = alloc::Layout::new::<cglue::exi_bitstream_t>();
        let handle = unsafe { alloc::alloc(layout) as *mut cglue::exi_bitstream_t };

        let mut stream = RawStream {
            layout,
            buffer: Box::pin([0; cglue::EXI_MAX_DOCUMENT_SIZE]),
            stream: handle,
            current_len: 0,
            expected_len: 0,
        };

        // create an empty exi doc
        unsafe {
            cglue::exi_bitstream_init(
                handle,
                stream.buffer.as_mut_ptr(),
                cglue::EXI_MAX_DOCUMENT_SIZE,
                0,
                Some(exi_stream_cb),
            )
        }
        stream
    }

    pub fn drop(&self) {
        // to drop object move back RawStream self into Rust allocate space
        let _ = unsafe { alloc::dealloc(self.stream as *mut u8, self.layout) };
    }

    pub fn get_buffer(&self) -> &[u8] {
        &(self.buffer[0..self.get_size()])
    }

    pub fn get_cursor(&self) -> usize {
        unsafe { cglue::exi_bitstream_get_length(self.stream) }
    }

    pub fn get_index(&self) -> (usize, usize) {
        let index = unsafe { cglue::exi_bitstream_get_length(self.stream) };
        (index, cglue::EXI_MAX_DOCUMENT_SIZE - index)
    }

    pub fn get_size(&self) -> usize {
        unsafe {
            let stream = self.stream.as_ref().expect("stream.reset valid handle");
            stream.data_size
        }
    }

    pub fn set_size(&self, size:u32) {
        unsafe {
            let stream = self.stream.as_mut().expect("stream.reset valid handle");
            stream.data_size= size as usize
        }
    }
    pub fn reset(&mut self) {
        // reset everything including data_count
        unsafe {
            self.expected_len = 0;
            self.current_len = 0;
            let stream = self.stream.as_mut().expect("stream.reset valid handle");
            stream.data_size = cglue::EXI_MAX_DOCUMENT_SIZE;
            stream.byte_pos = 0;
            stream.bit_count = 0;
        }
    }

    pub fn write_bits(&self, value: u32, bit_count: usize) -> Result<(), AfbError> {
        let status = unsafe { cglue::exi_bitstream_write_bits(self.stream, bit_count, value) };
        if status != 0 {
            return afb_error!(
                "exi-stream-wbits",
                "fail to write bit error:{:?}",
                ExiErrorCode::from_i32(status)
            );
        }
        Ok(())
    }
    pub fn write_octet(&self, value: u8) -> Result<(), AfbError> {
        let status = unsafe { cglue::exi_bitstream_write_octet(self.stream, value) };
        if status != 0 {
            return afb_error!(
                "exi-stream-woctets",
                "fail to write byte error:{:?}",
                ExiErrorCode::from_i32(status)
            );
        }
        Ok(())
    }
    pub fn read_bits(&self, bit_count: usize) -> Result<u32, AfbError> {
        let mut value: u32 = 0;
        let status = unsafe { cglue::exi_bitstream_read_bits(self.stream, bit_count, &mut value) };
        if status != 0 {
            return afb_error!(
                "exi-stream-rbits",
                "fail to read bit error:{:?}",
                ExiErrorCode::from_i32(status)
            );
        }
        Ok(value)
    }
    pub fn read_octet(&self) -> Result<u8, AfbError> {
        let mut value: u8 = 0;
        let status = unsafe { cglue::exi_bitstream_read_octet(self.stream, &mut value) };
        if status != 0 {
            return afb_error!(
                "exi-stream-roctets",
                "fail to read byte error:{:?}",
                ExiErrorCode::from_i32(status)
            );
        }
        Ok(value)
    }
}
