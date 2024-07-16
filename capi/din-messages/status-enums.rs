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
use std::convert::AsRef;
use std::mem;
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumString};

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
#[allow(dead_code)]
pub enum MessageTagId {
    SessionSetupReq,
    SessionSetupRes,
    ServiceDiscoveryReq,
    ServiceDiscoveryRes,
    ServiceDetailReq,
    ServiceDetailRes,
    AuthorizationReq,
    AuthorizationRes,
    BodyElement,
    CableCheckReq,
    CableCheckRes,
    CertificateInstallReq,
    CertificateInstallRes,
    CertificateUpdateReq,
    CertificateUpdateRes,
    ContractAuthenticationReq,
    ContractAuthenticationRes,
    ParamDiscoveryReq,
    ParamDiscoveryRes,
    ChargingStatusReq,
    ChargingStatusRes,
    CurrentDemandReq,
    CurrentDemandRes,
    MeteringReceiptReq,
    MeteringReceiptRes,
    PaymentDetailsReq,
    PaymentDetailsRes,
    PaymentSelectionReq,
    PaymentSelectionRes,
    PowerDeliveryReq,
    PowerDeliveryRes,
    PreChargeReq,
    PreChargeRes,
    SessionStopReq,
    SessionStopRes,
    WeldingDetectionReq,
    WeldingDetectionRes,
    Unsupported,
}

impl MessageTagId {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }

    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!(
                    "message_tagid_from_label",
                    "deserialize({}):{}",
                    json,
                    error
                )
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }

    pub fn match_resid(&self) -> Self {
        let response = match self {
            MessageTagId::ContractAuthenticationReq => MessageTagId::ContractAuthenticationRes,
            MessageTagId::SessionSetupReq => MessageTagId::SessionSetupRes,
            MessageTagId::ServiceDiscoveryReq => MessageTagId::ServiceDiscoveryRes,
            MessageTagId::ServiceDetailReq => MessageTagId::ServiceDetailRes,
            MessageTagId::AuthorizationReq => MessageTagId::AuthorizationRes,
            MessageTagId::CableCheckReq => MessageTagId::CableCheckRes,
            MessageTagId::CertificateInstallReq => MessageTagId::CertificateInstallRes,
            MessageTagId::CertificateUpdateReq => MessageTagId::CertificateUpdateRes,
            MessageTagId::ParamDiscoveryReq => MessageTagId::ParamDiscoveryRes,
            MessageTagId::ChargingStatusReq => MessageTagId::ChargingStatusRes,
            MessageTagId::CurrentDemandReq => MessageTagId::CurrentDemandRes,
            MessageTagId::MeteringReceiptReq => MessageTagId::MeteringReceiptRes,
            MessageTagId::PaymentDetailsReq => MessageTagId::PaymentDetailsRes,
            MessageTagId::PaymentSelectionReq => MessageTagId::PaymentSelectionRes,
            MessageTagId::PowerDeliveryReq => MessageTagId::PowerDeliveryRes,
            MessageTagId::PreChargeReq => MessageTagId::PreChargeRes,
            MessageTagId::SessionStopReq => MessageTagId::SessionStopRes,
            MessageTagId::WeldingDetectionReq => MessageTagId::WeldingDetectionRes,
            _ => MessageTagId::Unsupported,
        };
        response
    }
}

#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
#[allow(dead_code)]
pub enum ResponseCode {
    Ok = cglue::din_responseCodeType_din_responseCodeType_OK,
    Failed = cglue::din_responseCodeType_din_responseCodeType_FAILED,
    NewSession = cglue::din_responseCodeType_din_responseCodeType_OK_NewSessionEstablished,
    OldSessionJoin = cglue::din_responseCodeType_din_responseCodeType_OK_OldSessionJoined,
    CertificateExpiresSoon =
        cglue::din_responseCodeType_din_responseCodeType_OK_CertificateExpiresSoon,
    SequenceError = cglue::din_responseCodeType_din_responseCodeType_FAILED_SequenceError,
    ServiceIDInvalid = cglue::din_responseCodeType_din_responseCodeType_FAILED_ServiceIDInvalid,
    UnknownSession = cglue::din_responseCodeType_din_responseCodeType_FAILED_UnknownSession,
    ServiceSelectionInvalid =
        cglue::din_responseCodeType_din_responseCodeType_FAILED_ServiceSelectionInvalid,
    PaymentSelectionInvalid =
        cglue::din_responseCodeType_din_responseCodeType_FAILED_PaymentSelectionInvalid,
    CertificateExpired = cglue::din_responseCodeType_din_responseCodeType_FAILED_CertificateExpired,
    SignatureError = cglue::din_responseCodeType_din_responseCodeType_FAILED_SignatureError,
    NoCertificateAvailable =
        cglue::din_responseCodeType_din_responseCodeType_FAILED_NoCertificateAvailable,
    CertChainError = cglue::din_responseCodeType_din_responseCodeType_FAILED_CertChainError,
    ChallengeInvalid = cglue::din_responseCodeType_din_responseCodeType_FAILED_ChallengeInvalid,
    ContractCanceled = cglue::din_responseCodeType_din_responseCodeType_FAILED_ContractCanceled,
    WrongChargeParameter =
        cglue::din_responseCodeType_din_responseCodeType_FAILED_WrongChargeParameter,
    PowerDeliveryNotApplied =
        cglue::din_responseCodeType_din_responseCodeType_FAILED_PowerDeliveryNotApplied,
    TariffSelectionInvalid =
        cglue::din_responseCodeType_din_responseCodeType_FAILED_TariffSelectionInvalid,
    ChargingProfileInvalid =
        cglue::din_responseCodeType_din_responseCodeType_FAILED_ChargingProfileInvalid,
    MeteringSignatureNotValid =
        cglue::din_responseCodeType_din_responseCodeType_FAILED_MeteringSignatureNotValid,
    EVSEPresentVoltageToLow =
        cglue::din_responseCodeType_din_responseCodeType_FAILED_EVSEPresentVoltageToLow,
    WrongEnergyTransferType =
        cglue::din_responseCodeType_din_responseCodeType_FAILED_WrongEnergyTransferType,
}

impl ResponseCode {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }

    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!("response-code-from-label", "fail deserialize:{}", error)
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
pub enum ServiceCategory {
    EvCharger = cglue::din_serviceCategoryType_din_serviceCategoryType_EVCharging,
    Internet = cglue::din_serviceCategoryType_din_serviceCategoryType_Internet,
    Certificate = cglue::din_serviceCategoryType_din_serviceCategoryType_ContractCertificate,
    Other = cglue::din_serviceCategoryType_din_serviceCategoryType_OtherCustom,
}

impl ServiceCategory {
    pub fn from_u32(value: u32) -> Self {
        unsafe { std::mem::transmute(value) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!("service-category-from_label", "fail deserialize:{}", error)
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[allow(dead_code)]
#[repr(u32)]
pub enum PaymentOption {
    Contract = cglue::din_paymentOptionType_din_paymentOptionType_Contract,
    External = cglue::din_paymentOptionType_din_paymentOptionType_ExternalPayment,
}

impl PaymentOption {
    pub fn from_u32(value: u32) -> Self {
        unsafe { std::mem::transmute(value) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!("payment-option-from-label", "fail deserialize:{}", error)
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
pub enum EvseProcessing {
    Finished = cglue::din_EVSEProcessingType_din_EVSEProcessingType_Finished,
    Ongoing = cglue::din_EVSEProcessingType_din_EVSEProcessingType_Ongoing,
}
impl EvseProcessing {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!("evse-processing-from-label", "fail deserialize:{}", error)
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
pub enum DcEvErrorCode {
    NoError = cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_NO_ERROR,
    FailRessTempInhibit =
        cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_FAILED_RESSTemperatureInhibit,
    FailEvShiftPos = cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_FAILED_EVShiftPosition,
    FailChargeConnectLock =
        cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_FAILED_ChargerConnectorLockFault,
    FailEvresFault = cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_FAILED_EVRESSMalfunction,
    FailCurrentDifferential =
        cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_FAILED_ChargingCurrentdifferential,
    FailVoltOutOfRange =
        cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_FAILED_ChargingVoltageOutOfRange,
    FailReserveA = cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_Reserved_A,
    FailReserveB = cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_Reserved_B,
    FailReserveC = cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_Reserved_C,
    FailIncompatible =
        cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_FAILED_ChargingSystemIncompatibility,
    FailCodeNoData = cglue::din_DC_EVErrorCodeType_din_DC_EVErrorCodeType_NoData,
}
impl DcEvErrorCode {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!("dc-error-code-from-label", "fail deserialize:{}", error)
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[allow(dead_code)]
#[repr(u32)]
pub enum EvRequestTransfertMode {
    AcSinglePhase = cglue::din_EVRequestedEnergyTransferType_din_EVRequestedEnergyTransferType_AC_single_phase_core,
    AcTreePhase = cglue::din_EVRequestedEnergyTransferType_din_EVRequestedEnergyTransferType_AC_three_phase_core,
    DcBasic = cglue::din_EVRequestedEnergyTransferType_din_EVRequestedEnergyTransferType_DC_core,
    DcExtended =
        cglue::din_EVRequestedEnergyTransferType_din_EVRequestedEnergyTransferType_DC_extended,
    DcCombo =
        cglue::din_EVRequestedEnergyTransferType_din_EVRequestedEnergyTransferType_DC_combo_core,
    DcUnique = cglue::din_EVRequestedEnergyTransferType_din_EVRequestedEnergyTransferType_DC_unique,
}
impl EvRequestTransfertMode {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!(
                    "engy-transfert-mode-from-label",
                    "fail deserialize:{}",
                    error
                )
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[allow(dead_code)]
#[repr(u32)]
pub enum EvseNotification {
    // cglue::din_EVSENotificationType
    None = cglue::din_EVSENotificationType_din_EVSENotificationType_None,
    StopCharging = cglue::din_EVSENotificationType_din_EVSENotificationType_StopCharging,
    ReNegotiation = cglue::din_EVSENotificationType_din_EVSENotificationType_ReNegotiation,
}
impl EvseNotification {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!("evse-notification-from-label", "fail deserialize:{}", error)
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[allow(dead_code)]
#[repr(u32)]
pub enum IsolationStatus {
    Invalid = cglue::din_isolationLevelType_din_isolationLevelType_Invalid,
    Valid = cglue::din_isolationLevelType_din_isolationLevelType_Valid,
    Warning = cglue::din_isolationLevelType_din_isolationLevelType_Warning,
    Fault = cglue::din_isolationLevelType_din_isolationLevelType_Fault,
}
impl IsolationStatus {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!("isolation-status-from-label", "fail deserialize:{}", error)
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[allow(dead_code)]
#[repr(u32)]
pub enum DcEvseErrorCode {
    NotReady = cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_EVSE_NotReady,
    Ready = cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_EVSE_Ready,
    Shutdown = cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_EVSE_Shutdown,
    UtilInteruptEvt =
        cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_EVSE_UtilityInterruptEvent,
    MonitoringActive =
        cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_EVSE_IsolationMonitoringActive,
    EmergencyShutdown =
        cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_EVSE_EmergencyShutdown,
    EvseMalfunction = cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_EVSE_Malfunction,
    Reserve8 = cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_Reserved_8,
    Reserve9 = cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_Reserved_9,
    ReserveA = cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_Reserved_A,
    ReserveB = cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_Reserved_B,
    ReserveC = cglue::din_DC_EVSEStatusCodeType_din_DC_EVSEStatusCodeType_Reserved_C,
}

impl DcEvseErrorCode {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!(
                    "dc-evse-error-code-from-label",
                    "fail deserialize:{}",
                    error
                )
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[allow(dead_code)]
#[repr(u32)]
pub enum PhysicalUnit {
    Hour = cglue::din_unitSymbolType_din_unitSymbolType_h,
    Minute = cglue::din_unitSymbolType_din_unitSymbolType_m,
    Second = cglue::din_unitSymbolType_din_unitSymbolType_s,
    Ampere = cglue::din_unitSymbolType_din_unitSymbolType_A,
    Volt = cglue::din_unitSymbolType_din_unitSymbolType_V,
    Watt = cglue::din_unitSymbolType_din_unitSymbolType_W,
    Wh = cglue::din_unitSymbolType_din_unitSymbolType_Wh,
    VolAmp = cglue::din_unitSymbolType_din_unitSymbolType_VA,
    AmpHour = cglue::din_unitSymbolType_din_unitSymbolType_Ah,
    WattSecond = cglue::din_unitSymbolType_din_unitSymbolType_W_s,
}
impl PhysicalUnit {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!("physical-unit-from-label", "fail deserialize:{}", error)
            }
        }
    }

    pub fn to_label(&self) -> &str {
        self.as_ref()
    }
}

pub struct DcEvseStatusType {
    payload: cglue::din_DC_EVSEStatusType,
}

impl DcEvseStatusType {
    pub fn new(error: DcEvseErrorCode, notification: EvseNotification, delay: u32) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_DC_EVSEStatusType>() };
        payload.EVSEStatusCode = error as u32;
        payload.NotificationMaxDelay = delay;
        payload.EVSENotification = notification as u32;
        Self { payload }
    }

    pub fn get_error(&self) -> DcEvseErrorCode {
        DcEvseErrorCode::from_u32(self.payload.EVSEStatusCode)
    }

    pub fn get_notification(&self) -> EvseNotification {
        EvseNotification::from_u32(self.payload.EVSENotification)
    }

    pub fn get_delay(&self) -> u32 {
        self.payload.NotificationMaxDelay
    }

    pub fn set_isolation_status(&mut self, isolation: IsolationStatus) -> &mut Self {
        self.payload.EVSEIsolationStatus = isolation as u32;
        self.payload.set_EVSEIsolationStatus_isUsed(1);
        self
    }

    pub fn get_isolation_status(&self) -> Option<IsolationStatus> {
        if self.payload.EVSEIsolationStatus_isUsed() == 0 {
            None
        } else {
            Some(IsolationStatus::from_u32(self.payload.EVSEIsolationStatus))
        }
    }

    pub fn decode(payload: cglue::din_DC_EVSEStatusType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::din_DC_EVSEStatusType {
        self.payload
    }
}

#[derive(Clone, Copy)]
pub struct DcEvStatusType {
    payload: cglue::din_DC_EVStatusType,
}

impl DcEvStatusType {
    pub fn new(ready: bool, error: DcEvErrorCode, evress_soc: i8) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_DC_EVStatusType>() };
        payload.EVReady = if ready { 1 } else { 0 };
        payload.EVRESSSOC = evress_soc;
        payload.EVErrorCode = error as u32;
        Self { payload }
    }
        pub fn get_ready(&self) -> bool {
        if self.payload.EVReady == 0 {
            false
        } else {
            true
        }
    }

    pub fn get_error(&self) -> DcEvErrorCode {
        DcEvErrorCode::from_u32(self.payload.EVErrorCode)
    }

    pub fn get_evress_soc(&self) -> i8 {
        self.payload.EVRESSSOC
    }

    pub fn set_evcabin_conditioning(&mut self, value: i32) -> &mut Self {
        self.payload.EVCabinConditioning = value;
        self.payload.set_EVCabinConditioning_isUsed(1);
        self
    }

    pub fn get_evcabin_conditioning(&self) -> Option<i32> {
        if self.payload.EVCabinConditioning_isUsed() == 0 {
            None
        } else {
            Some(self.payload.EVCabinConditioning)
        }
    }

    pub fn set_evress_conditioning(&mut self, value: i32) -> &mut Self {
        self.payload.EVCabinConditioning = value;
        self.payload.set_EVRESSConditioning_isUsed(1);
        self
    }

    pub fn get_evress_conditioning(&self) -> Option<i32> {
        if self.payload.EVRESSConditioning_isUsed() == 0 {
            None
        } else {
            Some(self.payload.EVCabinConditioning)
        }
    }

    pub fn decode(payload: cglue::din_DC_EVStatusType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::din_DC_EVStatusType {
        self.payload
    }


}

pub struct AcEvseStatusType {
    payload: cglue::din_AC_EVSEStatusType,
}

impl AcEvseStatusType {
    pub fn new(notification: EvseNotification, delay: u32, rcd: bool) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_AC_EVSEStatusType>() };
        if rcd {
            payload.RCD = 1;
        } else {
            payload.RCD = 0;
        }
        payload.NotificationMaxDelay = delay;
        payload.EVSENotification = notification as u32;
        Self { payload }
    }

    pub fn get_notification(&self) -> EvseNotification {
        EvseNotification::from_u32(self.payload.EVSENotification)
    }

    pub fn get_delay(&self) -> u32 {
        self.payload.NotificationMaxDelay
    }

    pub fn get_rcd(&self) -> bool {
        if self.payload.RCD == 0 {
            false
        } else {
            true
        }
    }

    pub fn decode(payload: cglue::din_AC_EVSEStatusType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::din_AC_EVSEStatusType {
        self.payload
    }
}

pub struct EvseStatusType {
    payload: cglue::din_EVSEStatusType,
}

impl EvseStatusType {
    pub fn new(unused: i32) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_EVSEStatusType>() };
        payload._unused = unused;
        Self { payload }
    }

    pub fn decode(payload: cglue::din_EVSEStatusType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::din_EVSEStatusType {
        self.payload
    }
}
