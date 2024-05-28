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

    pub fn match_res_id(&self) -> Self {
        let response = match self {
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

#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
pub enum CostKind {
    PricePercent = cglue::iso2_costKindType_iso2_costKindType_relativePricePercentage,
    RenewGenPercent = cglue::iso2_costKindType_iso2_costKindType_RenewableGenerationPercentage,
    CarbonEmission = cglue::iso2_costKindType_iso2_costKindType_CarbonDioxideEmission,
}
impl CostKind {
    pub fn from_u32(value: u32) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!(
                    "cost_kind_from_label",
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
}

#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
#[allow(dead_code)]
pub enum ResponseCode {
    Ok = cglue::iso2_responseCodeType_iso2_responseCodeType_OK,
    Failed = cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED,
    NewSession = cglue::iso2_responseCodeType_iso2_responseCodeType_OK_NewSessionEstablished,
    OldSessionJoin = cglue::iso2_responseCodeType_iso2_responseCodeType_OK_OldSessionJoined,
    CertificateExpiresSoon =
        cglue::iso2_responseCodeType_iso2_responseCodeType_OK_CertificateExpiresSoon,
    SequenceError = cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_SequenceError,
    ServiceIDInvalid = cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_ServiceIDInvalid,
    UnknownSession = cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_UnknownSession,
    ServiceSelectionInvalid =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_ServiceSelectionInvalid,
    PaymentSelectionInvalid =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_PaymentSelectionInvalid,
    CertificateExpired =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_CertificateExpired,
    SignatureError = cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_SignatureError,
    NoCertificateAvailable =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_NoCertificateAvailable,
    CertChainError = cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_CertChainError,
    ChallengeInvalid = cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_ChallengeInvalid,
    ContractCanceled = cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_ContractCanceled,
    WrongChargeParameter =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_WrongChargeParameter,
    PowerDeliveryNotApplied =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_PowerDeliveryNotApplied,
    TariffSelectionInvalid =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_TariffSelectionInvalid,
    ChargingProfileInvalid =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_ChargingProfileInvalid,
    MeteringSignatureNotValid =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_MeteringSignatureNotValid,
    NoChargeServiceSelected =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_NoChargeServiceSelected,
    WrongEnergyTransferMode =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_WrongEnergyTransferMode,
    ContactorError = cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_ContactorError,
    CertificateNotAllowedAtThisEVSE =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_CertificateNotAllowedAtThisEVSE,
    CertificateRevoked =
        cglue::iso2_responseCodeType_iso2_responseCodeType_FAILED_CertificateRevoked,
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
    EvCharger = cglue::iso2_serviceCategoryType_iso2_serviceCategoryType_EVCharging,
    Internet = cglue::iso2_serviceCategoryType_iso2_serviceCategoryType_Internet,
    Certificate = cglue::iso2_serviceCategoryType_iso2_serviceCategoryType_ContractCertificate,
    Other = cglue::iso2_serviceCategoryType_iso2_serviceCategoryType_OtherCustom,
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
    Contract = cglue::iso2_paymentOptionType_iso2_paymentOptionType_Contract,
    External = cglue::iso2_paymentOptionType_iso2_paymentOptionType_ExternalPayment,
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

#[derive(Clone, Copy, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[allow(dead_code)]
#[repr(u32)]
pub enum ChargingSessionType {
    Terminate = cglue::iso2_chargingSessionType_iso2_chargingSessionType_Terminate,
    Pause = cglue::iso2_chargingSessionType_iso2_chargingSessionType_Pause,
}

impl ChargingSessionType {
    pub fn from_u32(value: u32) -> Self {
        unsafe { std::mem::transmute(value) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!("charging-session-from-label", "fail deserialize:{}", error)
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
pub enum ChargeProgress {
    Start = cglue::iso2_chargeProgressType_iso2_chargeProgressType_Start,
    Stop = cglue::iso2_chargeProgressType_iso2_chargeProgressType_Stop,
    Renegotiate = cglue::iso2_chargeProgressType_iso2_chargeProgressType_Renegotiate,
}
impl ChargeProgress {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }
    #[track_caller]
    pub fn from_label(json: &str) -> Result<Self, AfbError> {
        match Self::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => {
                return afb_error!("charge-progress-from-label", "fail deserialize:{}", error)
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
    Finished = cglue::iso2_EVSEProcessingType_iso2_EVSEProcessingType_Finished,
    Ongoing = cglue::iso2_EVSEProcessingType_iso2_EVSEProcessingType_Ongoing,
    CustomerInteraction = cglue::iso2_EVSEProcessingType_iso2_EVSEProcessingType_Ongoing_WaitingForCustomerInteraction,
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
    NoError = cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_NO_ERROR,
    FailRessTempInhibit =
        cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_FAILED_RESSTemperatureInhibit,
    FailEvShiftPos = cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_FAILED_EVShiftPosition,
    FailChargeConnectLock =
        cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_FAILED_ChargerConnectorLockFault,
    FailEvresFault =
        cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_FAILED_EVRESSMalfunction,
    FailCurrentDifferential =
        cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_FAILED_ChargingCurrentdifferential,
    FailVoltOutOfRange =
        cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_FAILED_ChargingVoltageOutOfRange,
    FailReserveA = cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_Reserved_A,
    FailReserveB = cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_Reserved_B,
    FailReserveC = cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_Reserved_C,
    FailIncompatible =
        cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_FAILED_ChargingSystemIncompatibility,
    FailCodeNoData = cglue::iso2_DC_EVErrorCodeType_iso2_DC_EVErrorCodeType_NoData,
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
pub enum EngyTransfertMode {
    AcSinglePhase =
        cglue::iso2_EnergyTransferModeType_iso2_EnergyTransferModeType_AC_single_phase_core,
    AcTreePhase =
        cglue::iso2_EnergyTransferModeType_iso2_EnergyTransferModeType_AC_three_phase_core,
    DcBasic = cglue::iso2_EnergyTransferModeType_iso2_EnergyTransferModeType_DC_core,
    DcExtended = cglue::iso2_EnergyTransferModeType_iso2_EnergyTransferModeType_DC_extended,
    DcCombo = cglue::iso2_EnergyTransferModeType_iso2_EnergyTransferModeType_DC_combo_core,
    DcUnique = cglue::iso2_EnergyTransferModeType_iso2_EnergyTransferModeType_DC_unique,
}
impl EngyTransfertMode {
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
    // cglue::iso2_EVSENotificationType
    None = cglue::iso2_EVSENotificationType_iso2_EVSENotificationType_None,
    StopCharging = cglue::iso2_EVSENotificationType_iso2_EVSENotificationType_StopCharging,
    ReNegotiation = cglue::iso2_EVSENotificationType_iso2_EVSENotificationType_ReNegotiation,
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
    Invalid = cglue::iso2_isolationLevelType_iso2_isolationLevelType_Invalid,
    Valid = cglue::iso2_isolationLevelType_iso2_isolationLevelType_Valid,
    Warning = cglue::iso2_isolationLevelType_iso2_isolationLevelType_Warning,
    Fault = cglue::iso2_isolationLevelType_iso2_isolationLevelType_Fault,
    NoImd = cglue::iso2_isolationLevelType_iso2_isolationLevelType_No_IMD,
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
    NotReady = cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_EVSE_NotReady,
    Ready = cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_EVSE_Ready,
    Shutdown = cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_EVSE_Shutdown,
    UtilInteruptEvt =
        cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_EVSE_UtilityInterruptEvent,
    MonitoringActive =
        cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_EVSE_IsolationMonitoringActive,
    EmergencyShutdown =
        cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_EVSE_EmergencyShutdown,
    EvseMalfunction = cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_EVSE_Malfunction,
    Reserve8 = cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_Reserved_8,
    Reserve9 = cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_Reserved_9,
    ReserveA = cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_Reserved_A,
    ReserveB = cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_Reserved_B,
    ReserveC = cglue::iso2_DC_EVSEStatusCodeType_iso2_DC_EVSEStatusCodeType_Reserved_C,
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
    Hour = cglue::iso2_unitSymbolType_iso2_unitSymbolType_h,
    Minute = cglue::iso2_unitSymbolType_iso2_unitSymbolType_m,
    Second = cglue::iso2_unitSymbolType_iso2_unitSymbolType_s,
    Ampere = cglue::iso2_unitSymbolType_iso2_unitSymbolType_A,
    Volt = cglue::iso2_unitSymbolType_iso2_unitSymbolType_V,
    Watt = cglue::iso2_unitSymbolType_iso2_unitSymbolType_W,
    Wh = cglue::iso2_unitSymbolType_iso2_unitSymbolType_Wh,
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
    payload: cglue::iso2_DC_EVSEStatusType,
}

impl DcEvseStatusType {
    pub fn new(error: DcEvseErrorCode, notification: EvseNotification, delay: u16) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_DC_EVSEStatusType>() };
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

    pub fn get_delay(&self) -> u16 {
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

    pub fn decode(payload: cglue::iso2_DC_EVSEStatusType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::iso2_DC_EVSEStatusType {
        self.payload
    }
}

#[derive(Clone, Copy)]
pub struct DcEvStatusType {
    payload: cglue::iso2_DC_EVStatusType,
}

impl DcEvStatusType {
    pub fn new(ready: bool, error: DcEvErrorCode, evresssoc: i8) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_DC_EVStatusType>() };
        payload.EVReady = if ready { 1 } else { 0 };
        payload.EVRESSSOC = evresssoc;
        payload.EVErrorCode = error as u32;
        Self { payload }
    }

    pub fn decode(payload: cglue::iso2_DC_EVStatusType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::iso2_DC_EVStatusType {
        self.payload
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

    pub fn get_evresssoc(&self) -> i8 {
        self.payload.EVRESSSOC
    }
}

pub struct AcEvseStatusType {
    payload: cglue::iso2_AC_EVSEStatusType,
}

impl AcEvseStatusType {
    pub fn new(notification: EvseNotification, delay: u16, rcd: bool) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_AC_EVSEStatusType>() };
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

    pub fn get_delay(&self) -> u16 {
        self.payload.NotificationMaxDelay
    }

    pub fn get_rcd(&self) -> bool {
        if self.payload.RCD == 0 {
            false
        } else {
            true
        }
    }

    pub fn decode(payload: cglue::iso2_AC_EVSEStatusType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::iso2_AC_EVSEStatusType {
        self.payload
    }
}

pub struct EvseStatusType {
    payload: cglue::iso2_EVSEStatusType,
}

impl EvseStatusType {
    pub fn new(
        notification: EvseNotification,
        delay: u16,
        ac_status: &AcEvseStatusType,
        dc_status: &DcEvseStatusType,
    ) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_EVSEStatusType>() };
        payload.NotificationMaxDelay = delay;
        payload.EVSENotification = notification as u32;
        payload.AC_EVSEStatus = ac_status.encode();
        payload.DC_EVSEStatus = dc_status.encode();
        Self { payload }
    }

    pub fn get_notification(&self) -> EvseNotification {
        EvseNotification::from_u32(self.payload.EVSENotification)
    }

    pub fn get_delay(&self) -> u16 {
        self.payload.NotificationMaxDelay
    }

    pub fn get_ac_status(&self) -> AcEvseStatusType {
        AcEvseStatusType::decode(self.payload.AC_EVSEStatus)
    }
    pub fn get_dc_status(&self) -> DcEvseStatusType {
        DcEvseStatusType::decode(self.payload.DC_EVSEStatus)
    }

    pub fn decode(payload: cglue::iso2_EVSEStatusType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::iso2_EVSEStatusType {
        self.payload
    }
}
