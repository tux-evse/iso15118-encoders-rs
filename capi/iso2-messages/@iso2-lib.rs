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

use crate::prelude::*;
use iso2_exi::*;

pub(self) mod cglue {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    // force reuse of C bitstream from exi-encoder
    use crate::prelude::exi_bitstream_t;
    use crate::prelude::cglue::{gnutls_pubkey_t, gnutls_privkey_t, gnutls_x509_trust_list_t, gnutls_x509_crt_t};
    include!("_iso2-capi.rs");
}


#[path = "status-enums.rs"]
mod status_enums;

#[path = "param-value.rs"]
mod param_value;

#[path = "param-discovery.rs"]
mod param_discovery;

#[path = "session-setup.rs"]
mod session_setup;

#[path = "service-discovery.rs"]
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
mod welding_detection;

#[path = "body-encoder.rs"]
mod body_encoder;

pub mod iso2_exi {
    pub use afbv4::prelude::*;
    pub use super::authorization::*;
    pub use super::body_element::*;
    pub use super::body_encoder::*;
    pub use super::cable_check::*;
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
    pub use super::service_discovery::*;
    pub use super::session_setup::*;
    pub use super::session_stop::*;
    pub use super::status_enums::*;
    pub use super::welding_detection::*;

    pub enum MessageBody {
        SessionSetupReq(SessionSetupRequest),
        SessionSetupRes(SessionSetupResponse),
        ServiceDiscoveryReq(ServiceDiscoveryRequest),
        ServiceDiscoveryRes(ServiceDiscoveryResponse),
        ServiceDetailReq(ServiceDetailRequest),
        ServiceDetailRes(ServiceDetailResponse),
        AuthorizationReq(AuthorizationRequest),
        AuthorizationRes(AuthorizationResponse),
        BodyElement(BodyBaseElement),
        CableCheckReq(CableCheckRequest),
        CableCheckRes(CableCheckResponse),
        CertificateInstallReq(CertificateInstallRequest),
        CertificateInstallRes(CertificateInstallResponse),
        CertificateUpdateReq(CertificateUpdateRequest),
        CertificateUpdateRes(CertificateUpdateResponse),
        ParamDiscoveryReq(ParamDiscoveryRequest),
        ParamDiscoveryRes(ParamDiscoveryResponse),
        ChargingStatusReq(ChargingStatusRequest),
        ChargingStatusRes(ChargingStatusResponse),
        CurrentDemandReq(CurrentDemandRequest),
        CurrentDemandRes(CurrentDemandResponse),
        MeteringReceiptReq(MeteringReceiptRequest),
        MeteringReceiptRes(MeteringReceiptResponse),
        PaymentDetailsReq(PaymentDetailsRequest),
        PaymentDetailsRes(PaymentDetailsResponse),
        PaymentSelectionReq(PaymentSelectionRequest),
        PaymentSelectionRes(PaymentSelectionResponse),
        PowerDeliveryReq(PowerDeliveryRequest),
        PowerDeliveryRes(PowerDeliveryResponse),
        PreChargeReq(PreChargeRequest),
        PreChargeRes(PreChargeResponse),
        SessionStopReq(SessionStopRequest),
        SessionStopRes(SessionStopResponse),
        WeldingDetectionReq(WeldingDetectionRequest),
        WeldingDetectionRes(WeldingDetectionResponse),
        Unsupported,
    }

    impl MessageBody {
        pub fn get_tagid(&self) -> MessageTagId {
            match self {
                MessageBody::SessionSetupReq(_) => MessageTagId::SessionSetupReq,
                MessageBody::SessionSetupRes(_) => MessageTagId::SessionSetupRes,
                MessageBody::ServiceDiscoveryReq(_) => MessageTagId::ServiceDiscoveryReq,
                MessageBody::ServiceDiscoveryRes(_) => MessageTagId::ServiceDiscoveryRes,
                MessageBody::ServiceDetailReq(_) => MessageTagId::ServiceDetailReq,
                MessageBody::ServiceDetailRes(_) => MessageTagId::ServiceDetailRes,
                MessageBody::AuthorizationReq(_) => MessageTagId::AuthorizationReq,
                MessageBody::AuthorizationRes(_) => MessageTagId::AuthorizationRes,
                MessageBody::BodyElement(_) => MessageTagId::BodyElement,
                MessageBody::CableCheckReq(_) => MessageTagId::CableCheckReq,
                MessageBody::CableCheckRes(_) => MessageTagId::CableCheckRes,
                MessageBody::CertificateInstallReq(_) => MessageTagId::CertificateInstallReq,
                MessageBody::CertificateInstallRes(_) => MessageTagId::CertificateInstallRes,
                MessageBody::CertificateUpdateReq(_) => MessageTagId::CertificateUpdateReq,
                MessageBody::CertificateUpdateRes(_) => MessageTagId::CertificateUpdateRes,
                MessageBody::ParamDiscoveryReq(_) => MessageTagId::ParamDiscoveryReq,
                MessageBody::ParamDiscoveryRes(_) => MessageTagId::ParamDiscoveryRes,
                MessageBody::ChargingStatusReq(_) => MessageTagId::ChargingStatusReq,
                MessageBody::ChargingStatusRes(_) => MessageTagId::ChargingStatusRes,
                MessageBody::CurrentDemandReq(_) => MessageTagId::CurrentDemandReq,
                MessageBody::CurrentDemandRes(_) => MessageTagId::CurrentDemandRes,
                MessageBody::MeteringReceiptReq(_) => MessageTagId::MeteringReceiptReq,
                MessageBody::MeteringReceiptRes(_) => MessageTagId::MeteringReceiptRes,
                MessageBody::PaymentDetailsReq(_) => MessageTagId::PaymentDetailsReq,
                MessageBody::PaymentDetailsRes(_) => MessageTagId::PaymentDetailsRes,
                MessageBody::PaymentSelectionReq(_) => MessageTagId::PaymentSelectionReq,
                MessageBody::PaymentSelectionRes(_) => MessageTagId::PaymentSelectionRes,
                MessageBody::PowerDeliveryReq(_) => MessageTagId::PowerDeliveryReq,
                MessageBody::PowerDeliveryRes(_) => MessageTagId::PowerDeliveryRes,
                MessageBody::PreChargeReq(_) => MessageTagId::PreChargeReq,
                MessageBody::PreChargeRes(_) => MessageTagId::PreChargeRes,
                MessageBody::SessionStopReq(_) => MessageTagId::SessionStopReq,
                MessageBody::SessionStopRes(_) => MessageTagId::SessionStopRes,
                MessageBody::WeldingDetectionReq(_) => MessageTagId::WeldingDetectionReq,
                MessageBody::WeldingDetectionRes(_) => MessageTagId::WeldingDetectionRes,
                MessageBody::Unsupported => MessageTagId::Unsupported,
            }
        }
        pub fn decode(payload: &super::cglue::iso2_BodyType) -> Result<Self, AfbError> {
            // SessionSetup
            let body = if payload.SessionSetupReq_isUsed() == 1 {
                let body = SessionSetupRequest::decode(unsafe {
                    payload.__bindgen_anon_1.SessionSetupReq
                });
                MessageBody::SessionSetupReq(body)
            } else if payload.SessionSetupRes_isUsed() == 1 {
                let body = SessionSetupResponse::decode(unsafe {
                    payload.__bindgen_anon_1.SessionSetupRes
                });
                MessageBody::SessionSetupRes(body)

            // ServiceDiscovery
            } else if payload.ServiceDiscoveryReq_isUsed() == 1 {
                let body = ServiceDiscoveryRequest::decode(unsafe {
                    payload.__bindgen_anon_1.ServiceDiscoveryReq
                });
                MessageBody::ServiceDiscoveryReq(body)
            } else if payload.ServiceDiscoveryRes_isUsed() == 1 {
                let body = ServiceDiscoveryResponse::decode(unsafe {
                    payload.__bindgen_anon_1.ServiceDiscoveryRes
                });
                MessageBody::ServiceDiscoveryRes(body)

            // ServiceDetail
            } else if payload.ServiceDetailReq_isUsed() == 1 {
                let body = ServiceDetailRequest::decode(unsafe {
                    payload.__bindgen_anon_1.ServiceDetailReq
                });
                MessageBody::ServiceDetailReq(body)
            } else if payload.ServiceDetailRes_isUsed() == 1 {
                let body = ServiceDetailResponse::decode(unsafe {
                    payload.__bindgen_anon_1.ServiceDetailRes
                });
                MessageBody::ServiceDetailRes(body)

            // Authorization
            } else if payload.AuthorizationReq_isUsed() == 1 {
                let body = AuthorizationRequest::decode(unsafe {
                    payload.__bindgen_anon_1.AuthorizationReq
                });
                MessageBody::AuthorizationReq(body)
            } else if payload.AuthorizationRes_isUsed() == 1 {
                let body = AuthorizationResponse::decode(unsafe {
                    payload.__bindgen_anon_1.AuthorizationRes
                });
                MessageBody::AuthorizationRes(body)

            // ElementBody
            } else if payload.BodyElement_isUsed() == 1 {
                let body = BodyBaseElement::decode(unsafe { payload.__bindgen_anon_1.BodyElement });
                MessageBody::BodyElement(body)

            // CableCheck
            } else if payload.CableCheckReq_isUsed() == 1 {
                let body =
                    CableCheckRequest::decode(unsafe { payload.__bindgen_anon_1.CableCheckReq });
                MessageBody::CableCheckReq(body)
            } else if payload.CableCheckRes_isUsed() == 1 {
                let body =
                    CableCheckResponse::decode(unsafe { payload.__bindgen_anon_1.CableCheckRes });
                MessageBody::CableCheckRes(body)

            // CertificateInstallation
            } else if payload.CertificateInstallationReq_isUsed() == 1 {
                let body = CertificateInstallRequest::decode(unsafe {
                    payload.__bindgen_anon_1.CertificateInstallationReq
                });
                MessageBody::CertificateInstallReq(body)
            } else if payload.CertificateInstallationRes_isUsed() == 1 {
                let body = CertificateInstallResponse::decode(unsafe {
                    payload.__bindgen_anon_1.CertificateInstallationRes
                });
                MessageBody::CertificateInstallRes(body)

            // CertifcateUpdate
            } else if payload.CertificateUpdateReq_isUsed() == 1 {
                let body = CertificateUpdateRequest::decode(unsafe {
                    payload.__bindgen_anon_1.CertificateUpdateReq
                });
                MessageBody::CertificateUpdateReq(body)
            } else if payload.CertificateUpdateRes_isUsed() == 1 {
                let body = CertificateUpdateResponse::decode(unsafe {
                    payload.__bindgen_anon_1.CertificateUpdateRes
                });
                MessageBody::CertificateUpdateRes(body)

            // ParamDicovery
            } else if payload.ChargeParameterDiscoveryReq_isUsed() == 1 {
                let body = ParamDiscoveryRequest::decode(unsafe {
                    payload.__bindgen_anon_1.ChargeParameterDiscoveryReq
                });
                MessageBody::ParamDiscoveryReq(body)
            } else if payload.ChargeParameterDiscoveryRes_isUsed() == 1 {
                let body = ParamDiscoveryResponse::decode(unsafe {
                    payload.__bindgen_anon_1.ChargeParameterDiscoveryRes
                });
                MessageBody::ParamDiscoveryRes(body)

            // ChargingSatus
            } else if payload.ChargingStatusReq_isUsed() == 1 {
                let body = ChargingStatusRequest::decode(unsafe {
                    payload.__bindgen_anon_1.ChargingStatusReq
                });
                MessageBody::ChargingStatusReq(body)
            } else if payload.ChargingStatusRes_isUsed() == 1 {
                let body = ChargingStatusResponse::decode(unsafe {
                    payload.__bindgen_anon_1.ChargingStatusRes
                });
                MessageBody::ChargingStatusRes(body)

            // CurrentDemand
            } else if payload.CurrentDemandReq_isUsed() == 1 {
                let body = CurrentDemandRequest::decode(unsafe {
                    payload.__bindgen_anon_1.CurrentDemandReq
                });
                MessageBody::CurrentDemandReq(body)
            } else if payload.CurrentDemandRes_isUsed() == 1 {
                let body = CurrentDemandResponse::decode(unsafe {
                    payload.__bindgen_anon_1.CurrentDemandRes
                });
                MessageBody::CurrentDemandRes(body)

            // MeteringReceipt
            } else if payload.MeteringReceiptReq_isUsed() == 1 {
                let body = MeteringReceiptRequest::decode(unsafe {
                    payload.__bindgen_anon_1.MeteringReceiptReq
                });
                MessageBody::MeteringReceiptReq(body)
            } else if payload.MeteringReceiptRes_isUsed() == 1 {
                let body = MeteringReceiptResponse::decode(unsafe {
                    payload.__bindgen_anon_1.MeteringReceiptRes
                });
                MessageBody::MeteringReceiptRes(body)

            // PaymentDetails
            } else if payload.PaymentDetailsReq_isUsed() == 1 {
                let body = PaymentDetailsRequest::decode(unsafe {
                    payload.__bindgen_anon_1.PaymentDetailsReq
                });
                MessageBody::PaymentDetailsReq(body)
            } else if payload.PaymentDetailsRes_isUsed() == 1 {
                let body = PaymentDetailsResponse::decode(unsafe {
                    payload.__bindgen_anon_1.PaymentDetailsRes
                });
                MessageBody::PaymentDetailsRes(body)

            // PaymentServiceSelection
            } else if payload.PaymentServiceSelectionReq_isUsed() == 1 {
                let body = PaymentSelectionRequest::decode(unsafe {
                    payload.__bindgen_anon_1.PaymentServiceSelectionReq
                });
                MessageBody::PaymentSelectionReq(body)
            } else if payload.PaymentServiceSelectionRes_isUsed() == 1 {
                let body = PaymentSelectionResponse::decode(unsafe {
                    payload.__bindgen_anon_1.PaymentServiceSelectionRes
                });
                MessageBody::PaymentSelectionRes(body)

            // PowerDelivery
            } else if payload.PowerDeliveryReq_isUsed() == 1 {
                let body = PowerDeliveryRequest::decode(unsafe {
                    payload.__bindgen_anon_1.PowerDeliveryReq
                });
                MessageBody::PowerDeliveryReq(body)
            } else if payload.PowerDeliveryRes_isUsed() == 1 {
                let body = PowerDeliveryResponse::decode(unsafe {
                    payload.__bindgen_anon_1.PowerDeliveryRes
                });
                MessageBody::PowerDeliveryRes(body)

            // PreCharge
            } else if payload.PreChargeReq_isUsed() == 1 {
                let body =
                    PreChargeRequest::decode(unsafe { payload.__bindgen_anon_1.PreChargeReq });
                MessageBody::PreChargeReq(body)
            } else if payload.PreChargeRes_isUsed() == 1 {
                let body =
                    PreChargeResponse::decode(unsafe { payload.__bindgen_anon_1.PreChargeRes });
                MessageBody::PreChargeRes(body)

            // SessionStop
            } else if payload.SessionStopReq_isUsed() == 1 {
                let body =
                    SessionStopRequest::decode(unsafe { payload.__bindgen_anon_1.SessionStopReq });
                MessageBody::SessionStopReq(body)
            } else if payload.SessionStopRes_isUsed() == 1 {
                let body =
                    SessionStopResponse::decode(unsafe { payload.__bindgen_anon_1.SessionStopRes });
                MessageBody::SessionStopRes(body)

            // WeldingDetection
            } else if payload.WeldingDetectionReq_isUsed() == 1 {
                let body = WeldingDetectionRequest::decode(unsafe {
                    payload.__bindgen_anon_1.WeldingDetectionReq
                });
                MessageBody::WeldingDetectionReq(body)
            } else if payload.WeldingDetectionRes_isUsed() == 1 {
                let body = WeldingDetectionResponse::decode(unsafe {
                    payload.__bindgen_anon_1.WeldingDetectionRes
                });
                MessageBody::WeldingDetectionRes(body)
            } else {
                return afb_error!("iso2-decode-exi", "unknown/unsupported message");
            };
            Ok(body)
        }
    }
}
