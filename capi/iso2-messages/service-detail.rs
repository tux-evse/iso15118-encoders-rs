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

pub struct ServiceDetailRequest {
    payload: cglue::iso2_ServiceDetailReqType,
}

impl ServiceDetailRequest {
    pub fn new(service_id: u16) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ServiceDetailReqType>() };
        payload.ServiceID = service_id;
        Self { payload }
    }

    pub fn decode(payload: cglue::iso2_ServiceDetailReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.ServiceDetailReq = self.payload;
            exi_body.set_ServiceDetailReq_isUsed(1);
            exi_body
        };
        body
    }

    pub fn get_id(&self) -> u16 {
        self.payload.ServiceID
    }
}

pub struct ServiceDetailResponse {
    payload: cglue::iso2_ServiceDetailResType,
}

impl ServiceDetailResponse {
    pub fn new(service_id: u16, code: ResponseCode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ServiceDetailResType>() };
        payload.ServiceID = service_id;
        payload.ResponseCode = code as u32;
        Self { payload }
    }

    pub fn get_id(&self) -> u16 {
        self.payload.ServiceID
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn add_pset(&mut self, prm_set: &ParamSet) -> Result<&mut Self, AfbError> {
        let idx = self.payload.ServiceParameterList.ParameterSet.arrayLen;
        if idx == cglue::iso2_ParameterSetType_5_ARRAY_SIZE as u16 {
            return afb_error!("service-detail-param", "param set array full");
        }
        self.payload.ServiceParameterList.ParameterSet.array[idx as usize] = prm_set.encode();
        self.payload.ServiceParameterList.ParameterSet.arrayLen = idx + 1;
        self.payload.set_ServiceParameterList_isUsed(1);
        Ok(self)
    }

    pub fn get_psets(&self) -> Vec<ParamSet> {
        let mut params = Vec::new();
        if self.payload.ServiceParameterList_isUsed() != 0 {
            for idx in 0..self.payload.ServiceParameterList.ParameterSet.arrayLen {
                params.push(ParamSet::decode(
                    self.payload.ServiceParameterList.ParameterSet.array[idx as usize],
                ))
            }
        }
        params
    }

    pub fn decode(payload: cglue::iso2_ServiceDetailResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.ServiceDetailRes = self.payload;
            exi_body.set_ServiceDetailRes_isUsed(1);
            exi_body
        };
        body
    }
}
