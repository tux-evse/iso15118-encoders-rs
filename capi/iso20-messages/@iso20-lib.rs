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
 
 /*
 #[path = "param-value.rs"]
 mod param_value;
 
 #[path = "param-discovery.rs"]
 mod param_discovery;*/
 
 #[path = "session-setup.rs"]
 mod session_setup;
 
 /*#[path = "service-discovery.rs"]
 mod service_discovery;
 
 #[path = "service-detail.rs"]
 mod service_detail;
 
 #[path = "authorization.rs"]
 mod authorization;
 
 #[path = "body-element.rs"]
 mod body_element;
 
 #[path = "cable-check.rs"]
 mod cable_check;
 
 #[path = "certificate-common.rs"]
 mod certificate_common;
 
 #[path = "certificate-install.rs"]
 mod certificate_install;
 
 #[path = "certificate-update.rs"]
 mod certificate_update;
 
 #[path = "charging-status.rs"]
 mod charging_status;
 
 #[path = "current-demand.rs"]
 mod current_demand;
 
 #[path = "metering-receipt.rs"]
 mod metering_receipt;
 
 #[path = "payment-details.rs"]
 mod payment_details;
 
 #[path = "payment-selection.rs"]
 mod payment_selection;
 
 #[path = "power-delivery.rs"]
 mod power_delivery;
 
 #[path = "pre-charge.rs"]
 mod pre_charge;
 
 #[path = "session-stop.rs"]
 mod session_stop;
 
 #[path = "welding-detection.rs"]
 mod welding_detection;*/
 
 #[path = "exi-encoder.rs"]
 mod exi_encoder;
 
 pub mod iso20_exi {
     pub use afbv4::prelude::*;
     /*pub use super::authorization::*;
     pub use super::body_element::*;*/
     pub use super::exi_encoder::*;
     /*pub use super::cable_check::*;
     pub use super::certificate_common::*;
     pub use super::certificate_install::*;
     pub use super::certificate_update::*;
     pub use super::charging_status::*;
     pub use super::current_demand::*;
     pub use super::metering_receipt::*;
     pub use super::param_discovery::*;
     pub use super::param_value::*;
     pub use super::payment_details::*;
     pub use super::payment_selection::*;
     pub use super::power_delivery::*;
     pub use super::pre_charge::*;
     pub use super::service_detail::*;
     pub use super::service_discovery::*;*/
     pub use super::session_setup::*;
     /*pub use super::session_stop::*;*/
     pub use super::status_enums::*;
     /*pub use super::welding_detection::*;*/
 
     pub enum MessageBody {
         SessionSetupReq(SessionSetupRequest),
         SessionSetupRes(SessionSetupResponse),
         /*pub AuthorizationSetupReq: iso20_AuthorizationSetupReqType,
         pub AuthorizationSetupRes: iso20_AuthorizationSetupResType,
         pub AuthorizationReq: iso20_AuthorizationReqType,
         pub AuthorizationRes: iso20_AuthorizationResType,
         pub ServiceDiscoveryReq: iso20_ServiceDiscoveryReqType,
         pub ServiceDiscoveryRes: iso20_ServiceDiscoveryResType,
         pub ServiceDetailReq: iso20_ServiceDetailReqType,
         pub ServiceDetailRes: iso20_ServiceDetailResType,
         pub ServiceSelectionReq: iso20_ServiceSelectionReqType,
         pub ServiceSelectionRes: iso20_ServiceSelectionResType,
         pub ScheduleExchangeReq: iso20_ScheduleExchangeReqType,
         pub ScheduleExchangeRes: iso20_ScheduleExchangeResType,
         pub PowerDeliveryReq: iso20_PowerDeliveryReqType,
         pub PowerDeliveryRes: iso20_PowerDeliveryResType,
         pub MeteringConfirmationReq: iso20_MeteringConfirmationReqType,
         pub MeteringConfirmationRes: iso20_MeteringConfirmationResType,
         pub SessionStopReq: iso20_SessionStopReqType,
         pub SessionStopRes: iso20_SessionStopResType,
         pub CertificateInstallationReq: iso20_CertificateInstallationReqType,
         pub CertificateInstallationRes: iso20_CertificateInstallationResType,
         pub VehicleCheckInReq: iso20_VehicleCheckInReqType,
         pub VehicleCheckInRes: iso20_VehicleCheckInResType,
         pub VehicleCheckOutReq: iso20_VehicleCheckOutReqType,
         pub VehicleCheckOutRes: iso20_VehicleCheckOutResType,*/

         Unsupported,
     }
 
     impl MessageBody {
         pub fn get_tagid(&self) -> MessageTagId {
             match self {
                 MessageBody::SessionSetupReq(_) => MessageTagId::SessionSetupReq,
                 MessageBody::SessionSetupRes(_) => MessageTagId::SessionSetupRes,
                 MessageBody::Unsupported => MessageTagId::Unsupported,
             }
         }
     }
 }
 