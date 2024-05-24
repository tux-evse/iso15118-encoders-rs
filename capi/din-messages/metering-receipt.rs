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
use std::mem;
use super::*;

pub struct MeterInfo {
    payload: cglue::din_MeterInfoType,
}

impl MeterInfo {
    pub fn new(meter_id: &str) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_MeterInfoType>() };
        payload.MeterID.charactersLen = str_to_array(
            meter_id,
            &mut payload.MeterID.characters,
            cglue::din_MeterID_CHARACTER_SIZE,
        )?;
        Ok(MeterInfo { payload })
    }

    pub fn get_id(&self) -> Result<&str, AfbError> {
        let text = array_to_str(
            &self.payload.MeterID.characters,
            self.payload.MeterID.charactersLen,
        )?;
        Ok(text)
    }

    pub fn set_reading(&mut self, reading: &PhysicalValue) -> &mut Self {
        self.payload.MeterReading = reading.encode();
        self.payload.set_MeterReading_isUsed(1);
        self
    }

    pub fn get_reading(&self) -> Option<PhysicalValue> {
        if self.payload.MeterReading_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.MeterReading))
        }
    }

    pub fn set_sig(&mut self, sig_meter: &[u8]) -> Result<&mut Self, AfbError> {
        self.payload.SigMeterReading.bytesLen = bytes_to_array(
            sig_meter,
            &mut self.payload.SigMeterReading.bytes,
            cglue::din_sigMeterReadingType_BYTES_SIZE,
        )?;

        if self.payload.SigMeterReading.bytesLen > 0 {
            self.payload.set_SigMeterReading_isUsed(1);
        }
        Ok(self)
    }

    pub fn get_sig(&self) -> Option<&[u8]> {
        if self.payload.SigMeterReading_isUsed() == 0 {
            None
        } else {
            let bytes = array_to_bytes(
                &self.payload.SigMeterReading.bytes,
                self.payload.SigMeterReading.bytesLen,
            );
            Some(bytes)
        }
    }

    pub fn set_status(&mut self, status: i16) -> &mut Self {
        self.payload.MeterStatus = status;
        self.payload.set_MeterStatus_isUsed(1);
        self
    }

    pub fn get_status(&self) -> Option<i16> {
        if self.payload.MeterStatus_isUsed() == 0 {
            None
        } else {
            Some(self.payload.MeterStatus)
        }
    }

    pub fn set_tmeter(&mut self, tmeter: i64) -> &mut Self {
        self.payload.TMeter = tmeter;
        self.payload.set_TMeter_isUsed(1);
        self
    }

    pub fn get_tmeter(&self) -> Option<i64> {
        if self.payload.TMeter_isUsed() == 0 {
            None
        } else {
            Some(self.payload.TMeter)
        }
    }
    pub fn decode(payload: cglue::din_MeterInfoType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_MeterInfoType {
        self.payload
    }
}

#[derive(Clone)]
pub struct MeteringReceiptRequest {
    payload: cglue::din_MeteringReceiptReqType,
}
impl MeteringReceiptRequest {
    pub fn new(session_id: &[u8], info: &MeterInfo) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_MeteringReceiptReqType>() };

        payload.SessionID.bytesLen = bytes_to_array(
            session_id,
            &mut payload.SessionID.bytes,
            cglue::din_sessionIDType_BYTES_SIZE,
        )?;
        payload.MeterInfo = info.encode();
        Ok(Self { payload })
    }

    pub fn get_session_id(&self) -> &[u8] {
        array_to_bytes(
            &self.payload.SessionID.bytes,
            self.payload.SessionID.bytesLen,
        )
    }

    pub fn get_info(&self) -> MeterInfo {
        MeterInfo::decode(self.payload.MeterInfo)
    }

    pub fn set_id(&mut self, id: &str) -> Result<&mut Self, AfbError> {
        self.payload.Id.charactersLen = str_to_array(
            id,
            &mut self.payload.Id.characters,
            cglue::din_Id_CHARACTER_SIZE,
        )?;
        self.payload.set_Id_isUsed(1);

        Ok(self)
    }

    pub fn get_id(&self) -> Option<&str> {
        if self.payload.Id_isUsed() == 0 {
            None
        } else {
            array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen).ok()
        }
    }

    pub fn set_tupple_id(&mut self, tuple_id: i16) -> &mut Self {
        self.payload.set_SAScheduleTupleID_isUsed(1);
        self.payload.SAScheduleTupleID= tuple_id;
        self
    }

    pub fn get_tuple_id(&self) -> Option<i16> {
        if self.payload.SAScheduleTupleID_isUsed() == 0 {
            None
        } else {
            Some(self.payload.SAScheduleTupleID)
        }
    }

    pub fn decode(payload: cglue::din_MeteringReceiptReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.MeteringReceiptReq = self.payload;
            exi_body.set_MeteringReceiptReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct MeteringReceiptResponse {
    payload: cglue::din_MeteringReceiptResType,
}

impl MeteringReceiptResponse {
    pub fn new(code: ResponseCode, ac_status: &AcEvseStatusType) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_MeteringReceiptResType>() };

        payload.ResponseCode = code as u32;
        payload.AC_EVSEStatus= ac_status.encode();
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }


    pub fn get_ac_evse_status(&self) -> AcEvseStatusType  {
            AcEvseStatusType::decode (self.payload.AC_EVSEStatus)
    }

    pub fn decode(payload: cglue::din_MeteringReceiptResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.MeteringReceiptRes = self.payload;
            exi_body.set_MeteringReceiptRes_isUsed(1);
            exi_body
        };
        body
    }
}
