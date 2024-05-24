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

// cargo test --package iso15118 --test test-v2g

#[path = "exi-encoder.rs"]
mod exi_encoder;

#[path = "v2g-messages/v2g-lib.rs"]
mod v2g_encoder;

#[path = "iso2-messages/iso2-lib.rs"]
mod iso2_encoder;

#[path = "din-messages/din-lib.rs"]
mod din_encoder;

pub mod prelude {
    pub use afbv4::prelude::*;
    pub use crate::capi::exi_encoder::*;
    pub use crate::capi::v2g_encoder::*;
    pub use crate::capi::iso2_encoder::*;
    pub use crate::capi::din_encoder::*;
}