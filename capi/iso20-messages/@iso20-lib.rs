/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Hugo Mercier <hugo.mercier@iot.bzh>
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

 use crate::prelude::*;
 use iso20_exi::*;
 
 pub(self) mod cglue {
     #![allow(dead_code)]
     #![allow(non_upper_case_globals)]
     #![allow(non_camel_case_types)]
     #![allow(non_snake_case)]
     // force reuse of C bitstream from exi-encoder
     use crate::prelude::exi_bitstream_t;
     use crate::prelude::cglue::{gnutls_pubkey_t, gnutls_privkey_t, gnutls_x509_trust_list_t, gnutls_x509_crt_t};
     include!("_iso20-capi.rs");
 }
 
 
 #[path = "status-enums.rs"]
 mod status_enums;
 
 #[path = "session-setup.rs"]
 mod session_setup;

 #[path = "authorization-setup.rs"]
 mod authorization_setup;
 
 #[path = "exi-encoder.rs"]
 mod exi_encoder;
 
 pub mod iso20_exi {
     pub use afbv4::prelude::*;
     pub use super::exi_encoder::*;

     pub use super::session_setup::*;
     pub use super::authorization_setup::*;

     pub use super::status_enums::*;
 
     pub enum MessageBody {
         SessionSetupReq(SessionSetupRequest),
         SessionSetupRes(SessionSetupResponse),
         AuthorizationSetupReq(AuthorizationSetupRequest),
         AuthorizationSetupRes(AuthorizationSetupResponse),
         Unsupported,
     }
 
     impl MessageBody {
         pub fn get_tagid(&self) -> MessageTagId {
             match self {
                 MessageBody::SessionSetupReq(_) => MessageTagId::SessionSetupReq,
                 MessageBody::SessionSetupRes(_) => MessageTagId::SessionSetupRes,
                 MessageBody::AuthorizationSetupReq(_) => MessageTagId::AuthorizationSetupReq,
                 MessageBody::AuthorizationSetupRes(_) => MessageTagId::AuthorizationSetupRes,
                 MessageBody::Unsupported => MessageTagId::Unsupported,
             }
         }
     }
 }
 