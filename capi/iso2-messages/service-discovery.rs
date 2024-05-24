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
pub struct ServiceOther {
    payload: cglue::iso2_ServiceType,
}

impl ServiceOther {
    pub fn new(id: u16, category: ServiceCategory, isfree: bool) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ServiceType>() };
        payload.ServiceID = id;
        payload.ServiceCategory = category as u32;

        if isfree {
            payload.FreeService = 1;
        }
        Self { payload }
    }

    pub fn get_id(&self) -> u16 {
        self.payload.ServiceID
    }

    pub fn get_category(&self) -> ServiceCategory {
        ServiceCategory::from_u32(self.payload.ServiceCategory)
    }

    pub fn get_isfree(&self) -> bool {
        if self.payload.FreeService != 0 {
            true
        } else {
            false
        }
    }

    pub fn set_name(&mut self, name: &str) -> Result<&mut Self, AfbError> {
        let len = str_to_array(
            name,
            &mut self.payload.ServiceName.characters,
            cglue::iso2_ServiceName_CHARACTER_SIZE,
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
            ).ok()
        };
        name
    }

    pub fn set_scope(&mut self, scope: &str) -> Result<&mut Self, AfbError> {
        let len = str_to_array(
            scope,
            &mut self.payload.ServiceScope.characters,
            cglue::iso2_ServiceScope_CHARACTER_SIZE,
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
            ).ok()
        };
        scope
    }

    pub fn decode(payload: cglue::iso2_ServiceType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::iso2_ServiceType {
        self.payload
    }
}


#[derive(Clone)]
pub struct ServiceCharging {
    payload: cglue::iso2_ChargeServiceType,
}

impl ServiceCharging {
    pub fn new(id: u16, isfree: bool) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ChargeServiceType>() };
        payload.ServiceID = id;

        if isfree {
            payload.FreeService = 1;
        }
        Self { payload }
    }
    pub fn get_id(&self) -> u16 {
        self.payload.ServiceID
    }

    pub fn get_isfree(&self) -> bool {
        if self.payload.FreeService != 0 {
            true
        } else {
            false
        }
    }

    pub fn set_name(&mut self, name: &str) -> Result<&mut Self, AfbError> {
        let len = str_to_array(
            name,
            &mut self.payload.ServiceName.characters,
            cglue::iso2_ServiceName_CHARACTER_SIZE,
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
            ).ok()
        };
        name
    }

    pub fn set_scope(&mut self, scope: &str) -> Result<&mut Self, AfbError> {
        let len = str_to_array(
            scope,
            &mut self.payload.ServiceScope.characters,
            cglue::iso2_ServiceScope_CHARACTER_SIZE,
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
            ).ok()
        };
        scope
    }

    pub fn decode(payload: cglue::iso2_ChargeServiceType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::iso2_ChargeServiceType {
        self.payload
    }
}

pub struct ServiceDiscoveryRequest {
    payload: cglue::iso2_ServiceDiscoveryReqType,
}

impl ServiceDiscoveryRequest {
    pub fn new() -> Self {
        let payload = unsafe { mem::zeroed::<cglue::iso2_ServiceDiscoveryReqType>() };
        Self { payload }
    }

    pub fn set_scope(&mut self, scope: &str) -> Result<&mut Self, AfbError> {
        let len = str_to_array(
            scope,
            &mut self.payload.ServiceScope.characters,
            cglue::iso2_ServiceScope_CHARACTER_SIZE,
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

    pub fn decode(payload: cglue::iso2_ServiceDiscoveryReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.ServiceDiscoveryReq = self.payload;
            exi_body.set_ServiceDiscoveryReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct ServiceDiscoveryResponse {
    payload: cglue::iso2_ServiceDiscoveryResType,
}

impl ServiceDiscoveryResponse {
    pub fn new(rcode: ResponseCode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ServiceDiscoveryResType>() };
        payload.ResponseCode = rcode as u32;
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn add_transfer(&mut self, mode: EngyTransfertMode) -> Result<&mut Self, AfbError> {
        let transfer_mode = &mut self
            .payload
            .ChargeService
            .SupportedEnergyTransferMode
            .EnergyTransferMode;

        let idx = transfer_mode.arrayLen;
        if idx == cglue::iso2_EnergyTransferModeType_6_ARRAY_SIZE as u16 {
            return afb_error!(
                "iso2-svc-discovery",
                "fail to add energy transfert mode (array full)"
            );
        }

        transfer_mode.array[idx as usize] = mode as u32;
        transfer_mode.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_transfers(&self) -> Result<Vec<EngyTransfertMode>, AfbError> {
        let mut response = Vec::new();
        let transfer_mode = &self
            .payload
            .ChargeService
            .SupportedEnergyTransferMode
            .EnergyTransferMode;

        for idx in 0..transfer_mode.arrayLen {
            let transfer = EngyTransfertMode::from_u32(transfer_mode.array[idx as usize]);
            response.push(transfer.clone())
        }
        Ok(response)
    }

    pub fn set_charging(&mut self, charging: &ServiceCharging) -> &mut Self {
        self.payload.ChargeService = charging.encode();
        self.payload.ChargeService.set_ServiceName_isUsed(1);
        self
    }

    pub fn get_charging(&self) -> Option<ServiceCharging> {
        if self.payload.ChargeService.ServiceName_isUsed() == 0 {
            None
        } else {
            Some(ServiceCharging::decode(self.payload.ChargeService))
        }
    }

    pub fn add_payment(&mut self, payment: PaymentOption) -> Result<&mut Self, AfbError> {
        let idx = self.payload.PaymentOptionList.PaymentOption.arrayLen;
        if idx == cglue::iso2_paymentOptionType_2_ARRAY_SIZE as u16 {
            return afb_error!("iso2-svc-discovery", "fail to add payment (array full)");
        }
        self.payload.PaymentOptionList.PaymentOption.array[idx as usize] = payment as u32;
        self.payload.PaymentOptionList.PaymentOption.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_payments(&self) -> Vec<PaymentOption> {
        let mut payments = Vec::new();
        for idx in 0..self.payload.PaymentOptionList.PaymentOption.arrayLen {
            let payment = PaymentOption::from_u32(
                self.payload.PaymentOptionList.PaymentOption.array[idx as usize],
            );
            payments.push(payment);
        }
        payments
    }

    pub fn add_service(&mut self, service: &ServiceOther) -> Result<&mut Self, AfbError> {
        let idx = self.payload.ServiceList.Service.arrayLen;
        if idx == cglue::iso2_ServiceType_8_ARRAY_SIZE as u16 {
            return afb_error!("iso2-svc-discovery", "fail to add service (array full)");
        }

        // update service directly into payload
        self.payload.ServiceList.Service.array[idx as usize] = service.encode();
        self.payload.ServiceList.Service.arrayLen = idx + 1;
        self.payload.set_ServiceList_isUsed(1);

        Ok(self)
    }

    pub fn get_services(&self) -> Result<Vec<ServiceOther>, AfbError> {
        let mut services = Vec::new();
        if self.payload.ServiceList_isUsed() != 0 {
            for idx in 0..self.payload.ServiceList.Service.arrayLen {
                let service = self.payload.ServiceList.Service.array[idx as usize];
                services.push(ServiceOther::decode(service))

            }
        }
        Ok(services)
    }

    pub fn decode(payload: cglue::iso2_ServiceDiscoveryResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.ServiceDiscoveryRes = self.payload;
            exi_body.set_ServiceDiscoveryRes_isUsed(1);
            exi_body
        };
        body
    }
}
