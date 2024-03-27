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

pub struct PaymentServiceOpt {
    service_id: u16,
    param_id: Option<i16>,
}

pub struct PaymentSelectionRequest {
    payload: cglue::iso2_PaymentServiceSelectionReqType,
}

impl PaymentSelectionRequest {
    pub fn new(payment: PaymentOption) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PaymentServiceSelectionReqType>() };
        payload.SelectedPaymentOption = payment as u32;
        Self { payload }
    }

    pub fn get_option(&self) -> PaymentOption {
        PaymentOption::from_u32(self.payload.SelectedPaymentOption)
    }

    pub fn add_service(&mut self, service: &PaymentServiceOpt) -> Result<&mut Self, AfbError> {
        let idx = self.payload.SelectedServiceList.SelectedService.arrayLen;
        if idx == cglue::iso2_SelectedServiceType_16_ARRAY_SIZE as u16 {
            return afb_error!("iso2-payment-option", "fail to add service (array full)");
        }

        let slot = &mut self.payload.SelectedServiceList.SelectedService.array[idx as usize];
        slot.ServiceID = service.service_id;
        if let Some(param_id) = service.param_id {
            slot.ParameterSetID = param_id;
            slot.set_ParameterSetID_isUsed(1)
        }
        self.payload.SelectedServiceList.SelectedService.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_services(&self) -> Vec<PaymentServiceOpt> {
        let mut response = Vec::new();
        for idx in 0..self.payload.SelectedServiceList.SelectedService.arrayLen {
            let slot = &self.payload.SelectedServiceList.SelectedService.array[idx as usize];

            let service = PaymentServiceOpt {
                service_id: slot.ServiceID,
                param_id: if slot.ParameterSetID_isUsed() == 0 {
                    None
                } else {
                    Some(slot.ParameterSetID)
                },
            };
            response.push(service);
        }
        response
    }

    pub fn decode(payload: cglue::iso2_PaymentServiceSelectionReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.PaymentServiceSelectionReq = self.payload;
            exi_body.set_PaymentServiceSelectionReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct PaymentSelectionResponse {
    payload: cglue::iso2_PaymentServiceSelectionResType,
}

impl PaymentSelectionResponse {
    pub fn new(code: ResponseCode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PaymentServiceSelectionResType>() };
        payload.ResponseCode = code as u32;
        Self { payload }
    }

    pub fn get_code (&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn decode(payload: cglue::iso2_PaymentServiceSelectionResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.PaymentServiceSelectionRes = self.payload;
            exi_body.set_PaymentServiceSelectionRes_isUsed(1);
            exi_body
        };
        body
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }
}
