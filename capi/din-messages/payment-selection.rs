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

pub struct SelectedService {
    payload: cglue::din_SelectedServiceType,
}

impl SelectedService {
    pub fn new(service_id: u16) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_SelectedServiceType>() };
        payload.ServiceID = service_id;
        Self { payload }
    }

    pub fn get_service_id(&self) -> u16 {
        self.payload.ServiceID
    }

    pub fn set_param_id(&mut self, param_id: i16) -> &mut Self {
        self.payload.ParameterSetID = param_id;
        self.payload.set_ParameterSetID_isUsed(1);
        self
    }

    pub fn get_param_id(&self) -> Option<i16> {
        if self.payload.ParameterSetID_isUsed() == 0 {
            None
        } else {
            Some(self.payload.ParameterSetID)
        }
    }

    pub fn decode(payload: cglue::din_SelectedServiceType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_SelectedServiceType {
        self.payload
    }
}

pub struct PaymentSelectionRequest {
    payload: cglue::din_ServicePaymentSelectionReqType,
}

impl PaymentSelectionRequest {
    pub fn new(payment: PaymentOption) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ServicePaymentSelectionReqType>() };
        payload.SelectedPaymentOption = payment as u32;
        Self { payload }
    }

    pub fn get_option(&self) -> PaymentOption {
        PaymentOption::from_u32(self.payload.SelectedPaymentOption)
    }

    pub fn add_service(&mut self, service: &SelectedService) -> Result<&mut Self, AfbError> {
        let idx = self.payload.SelectedServiceList.SelectedService.arrayLen;
        if idx == cglue::din_SelectedServiceType_16_ARRAY_SIZE as u16 {
            return afb_error!("din-payment-option", "fail to add service (array full)");
        }

        self.payload.SelectedServiceList.SelectedService.array[idx as usize] = service.encode();
        self.payload.SelectedServiceList.SelectedService.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_services(&self) -> Vec<SelectedService> {
        let mut response = Vec::new();
        for idx in 0..self.payload.SelectedServiceList.SelectedService.arrayLen {
            response.push(SelectedService::decode(
                self.payload.SelectedServiceList.SelectedService.array[idx as usize],
            ));
        }
        response
    }

    pub fn decode(payload: cglue::din_ServicePaymentSelectionReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.ServicePaymentSelectionReq = self.payload;
            exi_body.set_ServicePaymentSelectionReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct PaymentSelectionResponse {
    payload: cglue::din_ServicePaymentSelectionResType,
}

impl PaymentSelectionResponse {
    pub fn new(code: ResponseCode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ServicePaymentSelectionResType>() };
        payload.ResponseCode = code as u32;
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn decode(payload: cglue::din_ServicePaymentSelectionResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.ServicePaymentSelectionRes = self.payload;
            exi_body.set_ServicePaymentSelectionRes_isUsed(1);
            exi_body
        };
        body
    }
}
