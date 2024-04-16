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

#[path = "v2g-protocols.rs"]
mod protocols;

#[path = "v2g-sdp.rs"]
mod sdp;

pub(self) mod cglue {
        #![allow(dead_code)]
        #![allow(non_upper_case_globals)]
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]
        // force reuse of C bitstream from exi-encoder
        use crate::prelude::exi_bitstream_t;
        include!("_v2g-capi.rs");
}

pub mod v2g {
    pub use super::protocols::*;
    pub use super::sdp::*;
}
