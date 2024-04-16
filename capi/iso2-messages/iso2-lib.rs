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
use iso2::*;

pub(self) mod cglue {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    // force reuse of C bitstream from exi-encoder
    use crate::prelude::exi_bitstream_t;
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

pub mod iso2 {
    pub use super::authorization::*;
    pub use super::body_element::*;
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
    pub use super::body_encoder::*;
}

pub enum Iso2MessageBody {
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

impl Iso2MessageBody {
    pub fn get_tagid(&self) -> MessageTagId {
        match self {
            Iso2MessageBody::SessionSetupReq(_) => MessageTagId::SessionSetupReq,
            Iso2MessageBody::SessionSetupRes(_) => MessageTagId::SessionSetupRes,
            Iso2MessageBody::ServiceDiscoveryReq(_) => MessageTagId::ServiceDiscoveryReq,
            Iso2MessageBody::ServiceDiscoveryRes(_) => MessageTagId::ServiceDiscoveryRes,
            Iso2MessageBody::ServiceDetailReq(_) => MessageTagId::ServiceDetailReq,
            Iso2MessageBody::ServiceDetailRes(_) => MessageTagId::ServiceDetailRes,
            Iso2MessageBody::AuthorizationReq(_) => MessageTagId::AuthorizationReq,
            Iso2MessageBody::AuthorizationRes(_) => MessageTagId::AuthorizationRes,
            Iso2MessageBody::BodyElement(_) => MessageTagId::BodyElement,
            Iso2MessageBody::CableCheckReq(_) => MessageTagId::CableCheckReq,
            Iso2MessageBody::CableCheckRes(_) => MessageTagId::CableCheckRes,
            Iso2MessageBody::CertificateInstallReq(_) => MessageTagId::CertificateInstallReq,
            Iso2MessageBody::CertificateInstallRes(_) => MessageTagId::CertificateInstallRes,
            Iso2MessageBody::CertificateUpdateReq(_) => MessageTagId::CertificateUpdateReq,
            Iso2MessageBody::CertificateUpdateRes(_) => MessageTagId::CertificateUpdateRes,
            Iso2MessageBody::ParamDiscoveryReq(_) => MessageTagId::ParamDiscoveryReq,
            Iso2MessageBody::ParamDiscoveryRes(_) => MessageTagId::ParamDiscoveryRes,
            Iso2MessageBody::ChargingStatusReq(_) => MessageTagId::ChargingStatusReq,
            Iso2MessageBody::ChargingStatusRes(_) => MessageTagId::ChargingStatusRes,
            Iso2MessageBody::CurrentDemandReq(_) => MessageTagId::CurrentDemandReq,
            Iso2MessageBody::CurrentDemandRes(_) => MessageTagId::CurrentDemandRes,
            Iso2MessageBody::MeteringReceiptReq(_) => MessageTagId::MeteringReceiptReq,
            Iso2MessageBody::MeteringReceiptRes(_) => MessageTagId::MeteringReceiptRes,
            Iso2MessageBody::PaymentDetailsReq(_) => MessageTagId::PaymentDetailsReq,
            Iso2MessageBody::PaymentDetailsRes(_) => MessageTagId::PaymentDetailsRes,
            Iso2MessageBody::PaymentSelectionReq(_) => MessageTagId::PaymentSelectionReq,
            Iso2MessageBody::PaymentSelectionRes(_) => MessageTagId::PaymentSelectionRes,
            Iso2MessageBody::PowerDeliveryReq(_) => MessageTagId::PowerDeliveryReq,
            Iso2MessageBody::PowerDeliveryRes(_) => MessageTagId::PowerDeliveryRes,
            Iso2MessageBody::PreChargeReq(_) => MessageTagId::PreChargeReq,
            Iso2MessageBody::PreChargeRes(_) => MessageTagId::PreChargeRes,
            Iso2MessageBody::SessionStopReq(_) => MessageTagId::SessionStopReq,
            Iso2MessageBody::SessionStopRes(_) => MessageTagId::SessionStopRes,
            Iso2MessageBody::WeldingDetectionReq(_) => MessageTagId::WeldingDetectionReq,
            Iso2MessageBody::WeldingDetectionRes(_) => MessageTagId::WeldingDetectionRes,
            Iso2MessageBody::Unsupported => MessageTagId::Unsupported
        }
    }
    pub fn decode(v2g_body: &cglue::iso2_BodyType) -> Result<Self, AfbError> {
        // SessionSetup
        let body = if v2g_body.SessionSetupReq_isUsed() == 1 {
            let body =
                SessionSetupRequest::decode(unsafe { v2g_body.__bindgen_anon_1.SessionSetupReq });
            Iso2MessageBody::SessionSetupReq(body)
        } else if v2g_body.SessionSetupRes_isUsed() == 1 {
            let body =
                SessionSetupResponse::decode(unsafe { v2g_body.__bindgen_anon_1.SessionSetupRes });
            Iso2MessageBody::SessionSetupRes(body)

        // ServiceDiscovery
        } else if v2g_body.ServiceDiscoveryReq_isUsed() == 1 {
            let body = ServiceDiscoveryRequest::decode(unsafe {
                v2g_body.__bindgen_anon_1.ServiceDiscoveryReq
            });
            Iso2MessageBody::ServiceDiscoveryReq(body)
        } else if v2g_body.ServiceDiscoveryRes_isUsed() == 1 {
            let body = ServiceDiscoveryResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.ServiceDiscoveryRes
            });
            Iso2MessageBody::ServiceDiscoveryRes(body)

        // ServiceDetail
        } else if v2g_body.ServiceDetailReq_isUsed() == 1 {
            let body =
                ServiceDetailRequest::decode(unsafe { v2g_body.__bindgen_anon_1.ServiceDetailReq });
            Iso2MessageBody::ServiceDetailReq(body)
        } else if v2g_body.ServiceDetailRes_isUsed() == 1 {
            let body = ServiceDetailResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.ServiceDetailRes
            });
            Iso2MessageBody::ServiceDetailRes(body)

        // Authorization
        } else if v2g_body.AuthorizationReq_isUsed() == 1 {
            let body =
                AuthorizationRequest::decode(unsafe { v2g_body.__bindgen_anon_1.AuthorizationReq });
            Iso2MessageBody::AuthorizationReq(body)
        } else if v2g_body.AuthorizationRes_isUsed() == 1 {
            let body = AuthorizationResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.AuthorizationRes
            });
            Iso2MessageBody::AuthorizationRes(body)

        // ElementBody
        } else if v2g_body.BodyElement_isUsed() == 1 {
            let body = BodyBaseElement::decode(unsafe { v2g_body.__bindgen_anon_1.BodyElement });
            Iso2MessageBody::BodyElement(body)

        // CableCheck
        } else if v2g_body.CableCheckReq_isUsed() == 1 {
            let body =
                CableCheckRequest::decode(unsafe { v2g_body.__bindgen_anon_1.CableCheckReq });
            Iso2MessageBody::CableCheckReq(body)
        } else if v2g_body.CableCheckRes_isUsed() == 1 {
            let body =
                CableCheckResponse::decode(unsafe { v2g_body.__bindgen_anon_1.CableCheckRes });
            Iso2MessageBody::CableCheckRes(body)

        // CertificateInstallation
        } else if v2g_body.CertificateInstallationReq_isUsed() == 1 {
            let body = CertificateInstallRequest::decode(unsafe {
                v2g_body.__bindgen_anon_1.CertificateInstallationReq
            });
            Iso2MessageBody::CertificateInstallReq(body)
        } else if v2g_body.CertificateInstallationRes_isUsed() == 1 {
            let body = CertificateInstallResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.CertificateInstallationRes
            });
            Iso2MessageBody::CertificateInstallRes(body)

        // CertifcateUpdate
        } else if v2g_body.CertificateUpdateReq_isUsed() == 1 {
            let body = CertificateUpdateRequest::decode(unsafe {
                v2g_body.__bindgen_anon_1.CertificateUpdateReq
            });
            Iso2MessageBody::CertificateUpdateReq(body)
        } else if v2g_body.CertificateUpdateRes_isUsed() == 1 {
            let body = CertificateUpdateResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.CertificateUpdateRes
            });
            Iso2MessageBody::CertificateUpdateRes(body)

        // ParamDicovery
        } else if v2g_body.ChargeParameterDiscoveryReq_isUsed() == 1 {
            let body = ParamDiscoveryRequest::decode(unsafe {
                v2g_body.__bindgen_anon_1.ChargeParameterDiscoveryReq
            });
            Iso2MessageBody::ParamDiscoveryReq(body)
        } else if v2g_body.ChargeParameterDiscoveryRes_isUsed() == 1 {
            let body = ParamDiscoveryResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.ChargeParameterDiscoveryRes
            });
            Iso2MessageBody::ParamDiscoveryRes(body)

        // ChargingSatus
        } else if v2g_body.ChargingStatusReq_isUsed() == 1 {
            let body = ChargingStatusRequest::decode(unsafe {
                v2g_body.__bindgen_anon_1.ChargingStatusReq
            });
            Iso2MessageBody::ChargingStatusReq(body)
        } else if v2g_body.ChargingStatusRes_isUsed() == 1 {
            let body = ChargingStatusResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.ChargingStatusRes
            });
            Iso2MessageBody::ChargingStatusRes(body)

        // CurrentDemand
        } else if v2g_body.CurrentDemandReq_isUsed() == 1 {
            let body =
                CurrentDemandRequest::decode(unsafe { v2g_body.__bindgen_anon_1.CurrentDemandReq });
            Iso2MessageBody::CurrentDemandReq(body)
        } else if v2g_body.CurrentDemandRes_isUsed() == 1 {
            let body = CurrentDemandResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.CurrentDemandRes
            });
            Iso2MessageBody::CurrentDemandRes(body)

        // MeteringReceipt
        } else if v2g_body.MeteringReceiptReq_isUsed() == 1 {
            let body = MeteringReceiptRequest::decode(unsafe {
                v2g_body.__bindgen_anon_1.MeteringReceiptReq
            });
            Iso2MessageBody::MeteringReceiptReq(body)
        } else if v2g_body.MeteringReceiptRes_isUsed() == 1 {
            let body = MeteringReceiptResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.MeteringReceiptRes
            });
            Iso2MessageBody::MeteringReceiptRes(body)

        // PaymentDetails
        } else if v2g_body.PaymentDetailsReq_isUsed() == 1 {
            let body = PaymentDetailsRequest::decode(unsafe {
                v2g_body.__bindgen_anon_1.PaymentDetailsReq
            });
            Iso2MessageBody::PaymentDetailsReq(body)
        } else if v2g_body.PaymentDetailsRes_isUsed() == 1 {
            let body = PaymentDetailsResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.PaymentDetailsRes
            });
            Iso2MessageBody::PaymentDetailsRes(body)

        // PaymentServiceSelection
        } else if v2g_body.PaymentServiceSelectionReq_isUsed() == 1 {
            let body = PaymentSelectionRequest::decode(unsafe {
                v2g_body.__bindgen_anon_1.PaymentServiceSelectionReq
            });
            Iso2MessageBody::PaymentSelectionReq(body)
        } else if v2g_body.PaymentServiceSelectionRes_isUsed() == 1 {
            let body = PaymentSelectionResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.PaymentServiceSelectionRes
            });
            Iso2MessageBody::PaymentSelectionRes(body)

        // PowerDelivery
        } else if v2g_body.PowerDeliveryReq_isUsed() == 1 {
            let body =
                PowerDeliveryRequest::decode(unsafe { v2g_body.__bindgen_anon_1.PowerDeliveryReq });
            Iso2MessageBody::PowerDeliveryReq(body)
        } else if v2g_body.PowerDeliveryRes_isUsed() == 1 {
            let body = PowerDeliveryResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.PowerDeliveryRes
            });
            Iso2MessageBody::PowerDeliveryRes(body)

        // PreCharge
        } else if v2g_body.PreChargeReq_isUsed() == 1 {
            let body = PreChargeRequest::decode(unsafe { v2g_body.__bindgen_anon_1.PreChargeReq });
            Iso2MessageBody::PreChargeReq(body)
        } else if v2g_body.PreChargeRes_isUsed() == 1 {
            let body = PreChargeResponse::decode(unsafe { v2g_body.__bindgen_anon_1.PreChargeRes });
            Iso2MessageBody::PreChargeRes(body)

        // SessionStop
        } else if v2g_body.SessionStopReq_isUsed() == 1 {
            let body =
                SessionStopRequest::decode(unsafe { v2g_body.__bindgen_anon_1.SessionStopReq });
            Iso2MessageBody::SessionStopReq(body)
        } else if v2g_body.SessionStopRes_isUsed() == 1 {
            let body =
                SessionStopResponse::decode(unsafe { v2g_body.__bindgen_anon_1.SessionStopRes });
            Iso2MessageBody::SessionStopRes(body)

        // WeldingDetection
        } else if v2g_body.WeldingDetectionReq_isUsed() == 1 {
            let body = WeldingDetectionRequest::decode(unsafe {
                v2g_body.__bindgen_anon_1.WeldingDetectionReq
            });
            Iso2MessageBody::WeldingDetectionReq(body)
        } else if v2g_body.WeldingDetectionRes_isUsed() == 1 {
            let body = WeldingDetectionResponse::decode(unsafe {
                v2g_body.__bindgen_anon_1.WeldingDetectionRes
            });
            Iso2MessageBody::WeldingDetectionRes(body)
        } else {
            return afb_error!("iso2-decode-exi", "unknown/unsupported message");
        };
        Ok(body)
    }
}
