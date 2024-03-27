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
 * Reference: https://github.com/redpesk-common/afb-librust
 * Object: mock log/error APIs
 *
 */
use std::fmt;
use std::panic::Location;

#[derive(Debug)]
pub struct DbgInfo {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

pub struct AfbError {
    uid: String,
    info: String,
    dbg_info: DbgInfo,
}

impl AfbError {
    #[track_caller]
    pub fn new<T>(uid: &str, msg: T) -> AfbError
    where
        AfbError: MakeError<T>,
    {
        Self::make(uid, msg, Location::caller())
    }
}

impl fmt::Display for AfbError {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "{}:{}\n{:?}", self.uid, self.info, self.dbg_info)
    }
}

impl fmt::Debug for AfbError {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "{}:{}\n{:?}", self.uid, self.info, self.dbg_info)
    }
}

#[derive(Debug)]
pub enum AfbLogLevel {
    Error,
    Debug,
    Notice,
    Critical,
    Warning,
    Emergency,
    Info,
    Unknown = -1,
}

pub use crate::func_name;
#[doc(hidden)]
#[macro_export]
macro_rules! func_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

pub trait MakeError<T> {
    fn make(uid: &str, msg: T, location: &'static Location<'static>) -> AfbError;
}

impl MakeError<&str> for AfbError {
    fn make(uid: &str, msg: &str, caller: &'static Location<'static>) -> AfbError {
        AfbError {
            uid: uid.to_string(),
            info: msg.to_string(),
            dbg_info: DbgInfo {
                name: func_name!(),
                file: caller.file(),
                line: caller.line(),
                column: caller.column(),
            },
        }
    }
}

impl MakeError<String> for AfbError {
    fn make(uid: &str, msg: String, caller: &'static Location<'static>) -> AfbError {
        AfbError {
            uid: uid.to_string(),
            info: msg,
            dbg_info: DbgInfo {
                name: func_name!(),
                file: caller.file(),
                line: caller.line(),
                column: caller.column(),
            },
        }
    }
}

pub use crate::afb_error;
#[macro_export]
macro_rules! afb_error {
 ( $label:expr, $format:expr, $( $args:expr ),*) => {
    {
    Err(AfbError::new ($label, format! ($format, $($args),*)))
    }
 };
 ( $label:expr, $format:expr) => {
    {
     Err(AfbError::new ($label, $format))
    }
 }
}

pub use crate::afb_log_msg;
#[macro_export]
macro_rules! afb_log_msg {
 ( $level:tt, $handle:expr,$format:expr, $( $args:expr ),*) => {
    let name= func_name!();
    let file= file!();
    let line= line!();
    let column= column!();
    let message= format! ($format, $($args),*);
    println! ("{:?}:{} {}\n {}:{}:{}", AfbLogLevel::$level, name, message, file,line,column)
 };
 ( $level:tt, $handle:expr,$format:expr) => {
    let name= func_name!();
    let file= file!();
    let line= line!();
    let column= column!();
    println! ("{:?}:{} {}\n {}:{}:{}", AfbLogLevel::$level, name, $format, file,line,column)
    }
}

// ignore any AfbDataConverter! call
pub use crate::AfbDataConverter;
#[macro_export]
macro_rules! AfbDataConverter{
    ($handle:expr,$format:expr) => {}
}