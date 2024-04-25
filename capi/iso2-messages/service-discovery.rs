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
    id: u16,
    name: String,
    scope: String,
    category: ServiceCategory,
    isfree: bool,
}
impl ServiceOther {
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

#[derive(Debug, Clone)]
pub struct ServiceCharging {
    name: String,
    scope: String,
    isfree: bool,
}

impl ServiceCharging {
    pub fn new(name: &str, scope: &str, isfree: bool) -> Self {
        Self {
            name: name.to_string(),
            scope: scope.to_string(),
            isfree,
        }
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
    pub fn new(code: ResponseCode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ServiceDiscoveryResType>() };
        payload.ResponseCode = code as u32;
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

    pub fn set_charging(&mut self, charging: &ServiceCharging) -> Result<&mut Self, AfbError> {
        let len = str_to_array(
            charging.name.as_str(),
            &mut self.payload.ChargeService.ServiceName.characters,
            cglue::iso2_ServiceName_CHARACTER_SIZE,
        )?;
        if len > 0 {
            self.payload.ChargeService.ServiceName.charactersLen = len;
            self.payload.ChargeService.set_ServiceName_isUsed(1)
        };

        let len = str_to_array(
            charging.scope.as_str(),
            &mut self.payload.ChargeService.ServiceScope.characters,
            cglue::iso2_ServiceScope_CHARACTER_SIZE,
        )?;
        if len > 0 {
            self.payload.ChargeService.ServiceScope.charactersLen = len;
            self.payload.ChargeService.set_ServiceScope_isUsed(1)
        };

        if charging.isfree {
            self.payload.ChargeService.FreeService = 1;
        }
        Ok(self)
    }

    pub fn get_charging(&self) -> Result<ServiceCharging, AfbError> {
        let name = if self.payload.ChargeService.ServiceName_isUsed() == 0 {
            ""
        } else {
            array_to_str(
                &self.payload.ChargeService.ServiceName.characters,
                self.payload.ChargeService.ServiceName.charactersLen,
            )?
        };

        let scope = if self.payload.ChargeService.ServiceScope_isUsed() == 0 {
            ""
        } else {
            array_to_str(
                &self.payload.ChargeService.ServiceScope.characters,
                self.payload.ChargeService.ServiceScope.charactersLen,
            )?
        };

        let isfree = if self.payload.ChargeService.FreeService == 0 {
            false
        } else {
            true
        };

        Ok(ServiceCharging {
            name: name.to_string(),
            scope: scope.to_string(),
            isfree,
        })
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
        let svc = &mut self.payload.ServiceList.Service.array[idx as usize];
        svc.ServiceID = service.id;
        svc.ServiceCategory = service.category.clone() as u32;
        let len = str_to_array(
            service.name.as_str(),
            &mut svc.ServiceName.characters,
            cglue::iso2_ServiceName_CHARACTER_SIZE,
        )?;
        if len > 0 {
            svc.ServiceName.charactersLen = len;
            svc.set_ServiceName_isUsed(1)
        };

        let len = str_to_array(
            service.scope.as_str(),
            &mut svc.ServiceScope.characters,
            cglue::iso2_ServiceScope_CHARACTER_SIZE,
        )?;
        if len > 0 {
            svc.ServiceScope.charactersLen = len;
            svc.set_ServiceScope_isUsed(1)
        };

        if service.isfree {
            svc.FreeService = 1;
        }

        // add service in list service array
        self.payload.ServiceList.Service.arrayLen = idx + 1;
        self.payload.set_ServiceList_isUsed(1);

        Ok(self)
    }

    pub fn get_services(&self) -> Result<Vec<ServiceOther>, AfbError> {
        let mut services = Vec::new();
        if self.payload.ServiceList_isUsed() != 0 {
            for idx in 0..self.payload.ServiceList.Service.arrayLen {
                let service = &self.payload.ServiceList.Service.array[idx as usize];

                let id = service.ServiceID;

                let name = if service.ServiceName_isUsed() == 0 {
                    ""
                } else {
                    array_to_str(
                        &service.ServiceName.characters,
                        service.ServiceName.charactersLen,
                    )?
                };

                let scope = if service.ServiceScope_isUsed() == 0 {
                    ""
                } else {
                    array_to_str(
                        &service.ServiceScope.characters,
                        service.ServiceScope.charactersLen,
                    )?
                };

                let category = ServiceCategory::from_u32(service.ServiceCategory);

                let isfree = if service.FreeService == 0 {
                    false
                } else {
                    true
                };

                services.push(ServiceOther {
                    id,
                    name: name.to_string(),
                    scope: scope.to_string(),
                    category,
                    isfree,
                })
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
