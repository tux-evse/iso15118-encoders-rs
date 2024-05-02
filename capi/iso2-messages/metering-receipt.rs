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

pub struct MeterInfoType {
    payload: cglue::iso2_MeterInfoType,
}

impl MeterInfoType {
    pub fn new(meter_id: &str) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_MeterInfoType>() };
        payload.MeterID.charactersLen = str_to_array(
            meter_id,
            &mut payload.MeterID.characters,
            cglue::iso2_MeterID_CHARACTER_SIZE,
        )?;
        Ok(MeterInfoType { payload })
    }

    pub fn get_id(&self) -> Result<&str, AfbError> {
        let text = array_to_str(
            &self.payload.MeterID.characters,
            self.payload.MeterID.charactersLen,
        )?;
        Ok(text)
    }

    pub fn set_reading(&mut self, reading: u64) -> &mut Self {
        self.payload.MeterReading = reading;
        self.payload.set_MeterReading_isUsed(1);
        self
    }

    pub fn get_reading(&self) -> Option<u64> {
        if self.payload.MeterReading_isUsed() == 0 {
            None
        } else {
            Some(self.payload.MeterReading)
        }
    }

    pub fn set_sig(&mut self, sig_meter: &[u8]) -> Result<&mut Self, AfbError> {
        self.payload.SigMeterReading.bytesLen = bytes_to_array(
            sig_meter,
            &mut self.payload.SigMeterReading.bytes,
            cglue::iso2_sigMeterReadingType_BYTES_SIZE,
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
    pub fn decode(payload: cglue::iso2_MeterInfoType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_MeterInfoType {
        self.payload
    }
}

#[derive(Clone)]
pub struct MeteringReceiptRequest {
    payload: cglue::iso2_MeteringReceiptReqType,
}
impl MeteringReceiptRequest {
    pub fn new(session_id: &[u8], info: &MeterInfoType) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_MeteringReceiptReqType>() };

        payload.SessionID.bytesLen = bytes_to_array(
            session_id,
            &mut payload.SessionID.bytes,
            cglue::iso2_sessionIDType_BYTES_SIZE,
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

    pub fn get_info(&self) -> MeterInfoType {
        MeterInfoType::decode(self.payload.MeterInfo)
    }

    pub fn set_id(&mut self, id: &str) -> Result<&mut Self, AfbError> {
        self.payload.Id.charactersLen = str_to_array(
            id,
            &mut self.payload.Id.characters,
            cglue::iso2_Id_CHARACTER_SIZE,
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

    pub fn set_tupple_id(&mut self, tuple_id: u8) -> &mut Self {
        self.payload.set_SAScheduleTupleID_isUsed(1);
        self.payload.SAScheduleTupleID= tuple_id;
        self
    }

    pub fn get_tuple_id(&self) -> Option<u8> {
        if self.payload.SAScheduleTupleID_isUsed() == 0 {
            None
        } else {
            Some(self.payload.SAScheduleTupleID)
        }
    }

    pub fn decode(payload: cglue::iso2_MeteringReceiptReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.MeteringReceiptReq = self.payload;
            exi_body.set_MeteringReceiptReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct MeteringReceiptResponse {
    payload: cglue::iso2_MeteringReceiptResType,
}

impl MeteringReceiptResponse {
    pub fn new(code: ResponseCode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_MeteringReceiptResType>() };

        payload.ResponseCode = code as u32;
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn set_ac_evse_status(&mut self, status: &AcEvseStatusType) -> &mut Self {
        self.payload.AC_EVSEStatus = status.encode();
        self.payload.set_AC_EVSEStatus_isUsed(1);
        self
    }

    pub fn get_ac_evse_status(&self) -> Option<AcEvseStatusType>  {
        if  self.payload.AC_EVSEStatus_isUsed() == 0 {
            None
        } else {
            Some(AcEvseStatusType::decode (self.payload.AC_EVSEStatus))
        }
    }

    pub fn set_dc_evse_status(&mut self, status: &DcEvseStatusType) -> &mut Self {
        self.payload.DC_EVSEStatus = status.encode();
        self.payload.set_DC_EVSEStatus_isUsed(1);
        self
    }

    pub fn get_dc_evse_status(&self) -> Option<DcEvseStatusType>  {
        if  self.payload.DC_EVSEStatus_isUsed() == 0 {
            None
        } else {
            Some(DcEvseStatusType::decode (self.payload.DC_EVSEStatus))
        }
    }

    pub fn set_evse_status(&mut self, status: &EvseStatusType) -> &mut Self {
        self.payload.EVSEStatus = status.encode();
        self.payload.set_EVSEStatus_isUsed(1);
        self
    }

    pub fn get_evse_status(&self) -> Option<EvseStatusType>  {
        if  self.payload.EVSEStatus_isUsed() == 0 {
            None
        } else {
            Some(EvseStatusType::decode (self.payload.EVSEStatus))
        }
    }

    pub fn decode(payload: cglue::iso2_MeteringReceiptResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.MeteringReceiptRes = self.payload;
            exi_body.set_MeteringReceiptRes_isUsed(1);
            exi_body
        };
        body
    }
}
