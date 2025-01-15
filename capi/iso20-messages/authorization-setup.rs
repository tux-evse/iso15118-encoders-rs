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

use super::*;
use core::{iter::Iterator, str};
use std::mem;

use crate::{
    priv_string_field_getter, priv_string_field_getter_and_setter, string_field_getter_and_setter,
};

use v2g_macros::CGlue;

#[derive(CGlue)]
#[cglue_union_member(AuthorizationSetupReq)]
pub struct AuthorizationSetupRequest {
    payload: cglue::iso20_AuthorizationSetupReqType,
}
impl AuthorizationSetupRequest {
    pub fn from_cglue(payload: cglue::iso20_AuthorizationSetupReqType) -> Self {
        Self { payload }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub enum AuthorizationType {
    EIM = 0,
    PnC = 1,
}
impl AuthorizationType {
    pub fn from_u8(code: u8) -> Self {
        unsafe { mem::transmute(code) }
    }
}

pub struct PnCAuthorizationMode {
    // 128-bit long challenge = 16 x 8-bit bytes
    challenge: [u8; 16],
    // eMSP provider list
    // The list must contain 128 max elements
    provider_list: Vec<String>,
}

impl PnCAuthorizationMode {
    pub fn new(challenge: [u8; 16], provider_list: Vec<String>) -> Result<Self, AfbError> {
        if provider_list.len() > 128 {
            return afb_error!(
                "PnCAuthorizationMode::new",
                "Invalid size of provider_list ({}), must be <= 128",
                provider_list.len()
            );
        }
        for i in 0..provider_list.len() {
            if provider_list[i].len() > 81 {
                return afb_error!(
                    "PnCAuthorizationMode::new",
                    "Invalid size of provider ID ({}), must be <= 81",
                    provider_list[i].len()
                );
            }
        }
        Ok(Self {
            challenge,
            provider_list,
        })
    }

    pub fn get_challenge(&self) -> &[u8] {
        &self.challenge
    }

    pub fn get_provider_list(&self) -> &[String] {
        &self.provider_list
    }
}

#[derive(CGlue)]
#[cglue_union_member(AuthorizationSetupRes)]
pub struct AuthorizationSetupResponse {
    payload: cglue::iso20_AuthorizationSetupResType,
}

impl AuthorizationSetupResponse {
    pub fn from_cglue(payload: cglue::iso20_AuthorizationSetupResType) -> Self {
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_authorization_services(&self) -> Vec<AuthorizationType> {
        let mut v = Vec::<AuthorizationType>::new();
        for i in 0..self.payload.AuthorizationServices.arrayLen as usize {
            v.push(AuthorizationType::from_u8(
                self.payload.AuthorizationServices.array[i] as u8,
            ))
        }
        v
    }

    pub fn set_authorization_services(
        &mut self,
        auths: &[AuthorizationType],
    ) -> Result<(), AfbError> {
        if auths.len() < 1 || auths.len() > 2 {
            return afb_error!(
                "AuthorizationSetupResponse::set_authorization_services",
                "Invalid array size ({}), must be 1 or 2",
                auths.len()
            );
        }
        for (i, v) in auths.iter().enumerate() {
            self.payload.AuthorizationServices.array[i] = *v as u32;
        }
        self.payload.AuthorizationServices.arrayLen = auths.len() as u16;
        Ok(())
    }

    pub fn has_certificate_installation_service(&self) -> bool {
        if self.payload.CertificateInstallationService != 0 {
            true
        } else {
            false
        }
    }

    pub fn set_certificate_installation_service(&mut self, enabled: bool) {
        self.payload.CertificateInstallationService = if enabled { 1 } else { 0 };
    }

    pub fn set_eim_authorization_mode(&mut self, enabled: bool) {
        self.payload
            .set_EIM_ASResAuthorizationMode_isUsed(if enabled { 1 } else { 0 });
    }

    pub fn get_eim_authorization_mode(&self) -> bool {
        if self.payload.EIM_ASResAuthorizationMode_isUsed() != 0 {
            true
        } else {
            false
        }
    }

    pub fn get_pnc_authorization_mode(&self) -> Option<Result<PnCAuthorizationMode, AfbError>> {
        if self.payload.PnC_ASResAuthorizationMode_isUsed() == 0 {
            return None;
        } else {
            let challenge = array_to_bytes(
                &self.payload.PnC_ASResAuthorizationMode.GenChallenge.bytes,
                self.payload
                    .PnC_ASResAuthorizationMode
                    .GenChallenge
                    .bytesLen,
            );
            let mut provider_list = Vec::<String>::new();

            if self
                .payload
                .PnC_ASResAuthorizationMode
                .SupportedProviders_isUsed()
                != 0
            {
                for i in 0..self
                    .payload
                    .PnC_ASResAuthorizationMode
                    .SupportedProviders
                    .ProviderID
                    .arrayLen
                {
                    let str = match array_to_string(
                        &self
                            .payload
                            .PnC_ASResAuthorizationMode
                            .SupportedProviders
                            .ProviderID
                            .array[i as usize]
                            .characters,
                        self.payload
                            .PnC_ASResAuthorizationMode
                            .SupportedProviders
                            .ProviderID
                            .array[i as usize]
                            .charactersLen,
                    ) {
                        Ok(str) => str,
                        Err(err) => return Some(Err(err)),
                    };
                    provider_list.push(str);
                }
            }

            Some(Ok(PnCAuthorizationMode {
                challenge: challenge.try_into().unwrap(),
                provider_list,
            }))
        }
    }

    pub fn set_pnc_authorization_mode(
        &mut self,
        authorization_mode: Option<PnCAuthorizationMode>,
    ) -> Result<(), AfbError> {
        if authorization_mode.is_none() {
            self.payload.set_PnC_ASResAuthorizationMode_isUsed(0);
            return Ok(());
        }

        let authorization_mode = authorization_mode.unwrap();
        self.payload.set_PnC_ASResAuthorizationMode_isUsed(1);
        self.payload
            .PnC_ASResAuthorizationMode
            .GenChallenge
            .bytesLen = authorization_mode.challenge.len() as u16;
        self.payload.PnC_ASResAuthorizationMode.GenChallenge.bytes = authorization_mode.challenge;

        let provider_list = authorization_mode.get_provider_list();
        self.payload
            .PnC_ASResAuthorizationMode
            .set_SupportedProviders_isUsed(if provider_list.len() > 0 { 1 } else { 0 });
        if provider_list.len() > 0 {
            self.payload
                .PnC_ASResAuthorizationMode
                .SupportedProviders
                .ProviderID
                .arrayLen = provider_list.len() as u16;
            for i in 0..provider_list.len() {
                self.payload
                    .PnC_ASResAuthorizationMode
                    .SupportedProviders
                    .ProviderID
                    .array[i as usize]
                    .charactersLen = str_to_array(
                    &provider_list[i],
                    &mut self
                        .payload
                        .PnC_ASResAuthorizationMode
                        .SupportedProviders
                        .ProviderID
                        .array[i as usize]
                        .characters,
                    81,
                )?;
            }
        }
        Ok(())
    }
}
