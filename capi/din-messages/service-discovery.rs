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

use super::*;
use std::mem;

#[derive(Clone)]
pub struct ServiceOtherx {
    id: u16,
    name: String,
    scope: String,
    category: ServiceCategory,
    isfree: bool,
}
impl ServiceOtherx {
    pub fn new(id: u16, name: &str, scope: &str, category: ServiceCategory, isfree: bool) -> Self {
        Self {
            id,
            name: name.to_string(),
            scope: scope.to_string(),
            category,
            isfree,
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_scope(&self) -> String {
        self.scope.clone()
    }
    pub fn get_isfree(&self) -> bool {
        self.isfree
    }
    pub fn get_category(&self) -> ServiceCategory {
        self.category.clone()
    }
}
#[derive(Clone)]
pub struct ServiceTag {
    payload: cglue::din_ServiceTagType,
}

impl ServiceTag {
    pub fn new(id: u16, category: ServiceCategory) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ServiceTagType>() };
        payload.ServiceID = id;
        payload.ServiceCategory = category as u32;
        Self { payload }
    }

    pub fn get_id(&self) -> u16 {
        self.payload.ServiceID
    }

    pub fn get_category(&self) -> ServiceCategory {
        ServiceCategory::from_u32(self.payload.ServiceCategory)
    }

    pub fn set_name(&mut self, name: &str) -> Result<&mut Self, AfbError> {
        let len = str_to_array(
            name,
            &mut self.payload.ServiceName.characters,
            cglue::din_ServiceName_CHARACTER_SIZE,
        )?;
        self.payload.ServiceName.charactersLen = len;
        self.payload.set_ServiceName_isUsed(1);

        Ok(self)
    }

    pub fn get_name(&self) -> Option<&str> {
        let name = if self.payload.ServiceName_isUsed() == 0 {
            None
        } else {
            array_to_str(
                &self.payload.ServiceName.characters,
                self.payload.ServiceName.charactersLen,
            )
            .ok()
        };
        name
    }

    pub fn set_scope(&mut self, scope: &str) -> Result<&mut Self, AfbError> {
        let len = str_to_array(
            scope,
            &mut self.payload.ServiceScope.characters,
            cglue::din_ServiceScope_CHARACTER_SIZE,
        )?;
        self.payload.ServiceScope.charactersLen = len;
        self.payload.set_ServiceScope_isUsed(1);

        Ok(self)
    }

    pub fn get_scope(&self) -> Option<&str> {
        let scope = if self.payload.ServiceScope_isUsed() == 0 {
            None
        } else {
            array_to_str(
                &self.payload.ServiceScope.characters,
                self.payload.ServiceScope.charactersLen,
            )
            .ok()
        };
        scope
    }

    pub fn decode(payload: cglue::din_ServiceTagType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::din_ServiceTagType {
        self.payload
    }
}

pub struct ServiceOther {
    payload: cglue::din_ServiceType,
}

impl ServiceOther {
    pub fn new(service_tag: &ServiceTag, isfree: bool) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ServiceType>() };
        payload.ServiceTag = service_tag.encode();
        payload.FreeService = if isfree { 1 } else { 0 };

        Self { payload }
    }

    pub fn get_tag(&self) -> ServiceTag {
        ServiceTag::decode(self.payload.ServiceTag)
    }

    pub fn get_isfree(&self) -> bool {
        if self.payload.FreeService == 0 {
            false
        } else {
            true
        }
    }

    pub fn decode(payload: cglue::din_ServiceType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::din_ServiceType {
        self.payload
    }
}

#[derive(Clone)]
pub struct ServiceCharging {
    payload: cglue::din_ServiceChargeType,
}

impl ServiceCharging {
    pub fn new(service_tag: &ServiceTag, transfer: EvRequestTransfertMode, isfree: bool) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ServiceChargeType>() };
        payload.ServiceTag = service_tag.encode();
        payload.EnergyTransferType= transfer as u32;

        if isfree {
            payload.FreeService = 1;
        }
        Self { payload }
    }

    pub fn get_tag(&self) -> ServiceTag {
        ServiceTag::decode(self.payload.ServiceTag)
    }

    pub fn get_transfer(&self) -> EvRequestTransfertMode {
       EvRequestTransfertMode::from_u32(self.payload.EnergyTransferType)

    }

    pub fn get_isfree(&self) -> bool {
        if self.payload.FreeService != 0 {
            true
        } else {
            false
        }
    }

    pub fn decode(payload: cglue::din_ServiceChargeType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::din_ServiceChargeType {
        self.payload
    }
}

pub struct ServiceDiscoveryRequest {
    payload: cglue::din_ServiceDiscoveryReqType,
}

impl ServiceDiscoveryRequest {
    pub fn new() -> Self {
        let payload = unsafe { mem::zeroed::<cglue::din_ServiceDiscoveryReqType>() };
        Self { payload }
    }

    pub fn set_scope(&mut self, scope: &str) -> Result<&mut Self, AfbError> {
        let len = str_to_array(
            scope,
            &mut self.payload.ServiceScope.characters,
            cglue::din_ServiceScope_CHARACTER_SIZE,
        )?;

        if len > 0 {
            self.payload.ServiceScope.charactersLen = len;
            self.payload.set_ServiceScope_isUsed(1);
        }
        Ok(self)
    }

    pub fn get_scope<'a>(&'a self) -> Option<&'a str> {
        let scope = if self.payload.ServiceScope_isUsed() == 0 {
            None
        } else {
            array_to_str(
                &self.payload.ServiceScope.characters,
                self.payload.ServiceScope.charactersLen,
            )
            .ok()
        };
        scope
    }

    pub fn set_category(&mut self, category: ServiceCategory) -> &mut Self {
        self.payload.ServiceCategory = category as u32;
        self.payload.set_ServiceCategory_isUsed(1);
        self
    }

    pub fn get_category(&self) -> Option<ServiceCategory> {
        let category = if self.payload.ServiceCategory_isUsed() == 0 {
            None
        } else {
            Some(ServiceCategory::from_u32(self.payload.ServiceCategory))
        };
        category
    }

    pub fn decode(payload: cglue::din_ServiceDiscoveryReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.ServiceDiscoveryReq = self.payload;
            exi_body.set_ServiceDiscoveryReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct ServiceDiscoveryResponse {
    payload: cglue::din_ServiceDiscoveryResType,
}

impl ServiceDiscoveryResponse {
    pub fn new(rcode: ResponseCode, charge: &ServiceCharging) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ServiceDiscoveryResType>() };
        payload.ResponseCode = rcode as u32;
        payload.ChargeService= charge.encode();
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_charging(&self) -> ServiceCharging {
       ServiceCharging::decode(self.payload.ChargeService)
    }

    pub fn set_service(&mut self, service: &ServiceOther) -> &mut Self {
        self.payload.ServiceList.Service = service.encode();
        self.payload.set_ServiceList_isUsed(1);
        self
    }

    pub fn get_service(&self) -> Option<ServiceOther> {
        if self.payload.ServiceList_isUsed() == 0 {
            None
        } else {
            Some(ServiceOther::decode(self.payload.ServiceList.Service))
        }
    }

    pub fn add_payment(&mut self, payment: PaymentOption) -> Result<&mut Self, AfbError> {
        let idx = self.payload.PaymentOptions.PaymentOption.arrayLen;
        if idx == cglue::din_paymentOptionType_2_ARRAY_SIZE as u16 {
            return afb_error!("din-svc-discovery", "fail to add payment (array full)");
        }
        self.payload.PaymentOptions.PaymentOption.array[idx as usize] = payment as u32;
        self.payload.PaymentOptions.PaymentOption.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_payments(&self) -> Vec<PaymentOption> {
        let mut payments = Vec::new();
        for idx in 0..self.payload.PaymentOptions.PaymentOption.arrayLen {
            let payment = PaymentOption::from_u32(
                self.payload.PaymentOptions.PaymentOption.array[idx as usize],
            );
            payments.push(payment);
        }
        payments
    }

    pub fn decode(payload: cglue::din_ServiceDiscoveryResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.ServiceDiscoveryRes = self.payload;
            exi_body.set_ServiceDiscoveryRes_isUsed(1);
            exi_body
        };
        body
    }
}
