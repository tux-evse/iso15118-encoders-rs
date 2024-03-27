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

#[derive(Clone)]
pub struct BodyBaseElement {
    payload: cglue::iso2_BodyBaseType,
}
impl BodyBaseElement {
    pub fn new()->Self {
        let payload = unsafe { mem::zeroed::<cglue::iso2_BodyBaseType>() };
        Self { payload }
    }

    pub fn decode(payload: cglue::iso2_BodyBaseType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.BodyElement = self.payload;
            exi_body.set_BodyElement_isUsed(1);
            exi_body
        };
        body
    }
}