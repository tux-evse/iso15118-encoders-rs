use strum_macros::{AsRefStr, Display, EnumString};

use super::*;

use core::mem;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
#[allow(dead_code)]
pub enum MessageTagId {
    SessionSetupReq,
    SessionSetupRes,
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
            MessageTagId::SessionSetupReq => MessageTagId::SessionSetupRes,
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
    Ok = cglue::iso20_responseCodeType_iso20_responseCodeType_OK,
    OkCertificateExpiresSoon =
        cglue::iso20_responseCodeType_iso20_responseCodeType_OK_CertificateExpiresSoon,
    OkNewSessionEstablished =
        cglue::iso20_responseCodeType_iso20_responseCodeType_OK_NewSessionEstablished,
    OkOldSessionJoined = cglue::iso20_responseCodeType_iso20_responseCodeType_OK_OldSessionJoined,
    OkPowerToleranceConfirmed =
        cglue::iso20_responseCodeType_iso20_responseCodeType_OK_PowerToleranceConfirmed,
    WarningAuthorizationSelectionInvalid =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_AuthorizationSelectionInvalid,
    WarningCertificateExpired =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_CertificateExpired,
    WarningCertificateNotYetValid =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_CertificateNotYetValid,
    WarningCertificateRevoked =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_CertificateRevoked,
    WarningCertificateValidationError =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_CertificateValidationError,
    WarningChallengeInvalid =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_ChallengeInvalid,
    WarningEIMAuthorizationFailure =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_EIMAuthorizationFailure,
    WarningeMSPUnknown = cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_eMSPUnknown,
    WarningEVPowerProfileViolation =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_EVPowerProfileViolation,
    WarningGeneralPnCAuthorizationError =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_GeneralPnCAuthorizationError,
    WarningNoCertificateAvailable =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_NoCertificateAvailable,
    WarningNoContractMatchingPCIDFound =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_NoContractMatchingPCIDFound,
    WarningPowerToleranceNotConfirmed =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_PowerToleranceNotConfirmed,
    WarningScheduleRenegotiationFailed =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_ScheduleRenegotiationFailed,
    WarningStandbyNotAllowed =
        cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_StandbyNotAllowed,
    WarningWPT = cglue::iso20_responseCodeType_iso20_responseCodeType_WARNING_WPT,
    Failed = cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED,
    FailedAssociationError =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_AssociationError,
    FailedContactorError =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_ContactorError,
    FailedEVPowerProfileInvalid =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_EVPowerProfileInvalid,
    FailedEVPowerProfileViolation =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_EVPowerProfileViolation,
    FailedMeteringSignatureNotValid =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_MeteringSignatureNotValid,
    FailedNoEnergyTransferServiceSelected =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_NoEnergyTransferServiceSelected,
    FailedNoServiceRenegotiationSupported =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_NoServiceRenegotiationSupported,
    FailedPauseNotAllowed =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_PauseNotAllowed,
    FailedPowerDeliveryNotApplied =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_PowerDeliveryNotApplied,
    FailedPowerToleranceNotConfirmed =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_PowerToleranceNotConfirmed,
    FailedScheduleRenegotiation =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_ScheduleRenegotiation,
    FailedScheduleSelectionInvalid =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_ScheduleSelectionInvalid,
    FailedSequenceError = cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_SequenceError,
    FailedServiceIDInvalid =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_ServiceIDInvalid,
    FailedServiceSelectionInvalid =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_ServiceSelectionInvalid,
    FailedSignatureError =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_SignatureError,
    FailedUnknownSession =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_UnknownSession,
    FailedWrongChargeParameter =
        cglue::iso20_responseCodeType_iso20_responseCodeType_FAILED_WrongChargeParameter,
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
                return afb_error!(
                    "response-code-from-label",
                    "fail deserialize:{} {}",
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
