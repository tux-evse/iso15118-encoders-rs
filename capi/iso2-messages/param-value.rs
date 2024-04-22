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
use std::fmt;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Isp2PhysicalUnit {
    Hour = cglue::iso2_unitSymbolType_iso2_unitSymbolType_h,
    Minute = cglue::iso2_unitSymbolType_iso2_unitSymbolType_m,
    Second = cglue::iso2_unitSymbolType_iso2_unitSymbolType_s,
    Ampere = cglue::iso2_unitSymbolType_iso2_unitSymbolType_A,
    Volt = cglue::iso2_unitSymbolType_iso2_unitSymbolType_V,
    Watt = cglue::iso2_unitSymbolType_iso2_unitSymbolType_W,
    Wh = cglue::iso2_unitSymbolType_iso2_unitSymbolType_Wh,
    Percent= 9999,
}
impl Isp2PhysicalUnit {
    pub fn from_u32(value: u32) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[derive(Clone, Copy)]
pub struct PhysicalValue {
    payload: cglue::iso2_PhysicalValueType,
}
impl fmt::Debug for PhysicalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.get_value();
        let multiplier = self.get_multiplier();
        let unit = self.get_unit();
        write!(
            f,
            "(value:{}, multiplier:{} unit:{:?})",
            value, multiplier, unit
        )
    }
}

impl PhysicalValue {
    pub fn new(value: i16, multiplier: i8, unit: Isp2PhysicalUnit) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PhysicalValueType>() };
        payload.Multiplier = multiplier;
        payload.Unit = unit as u32;
        payload.Value = value;
        Self { payload }
    }

    pub fn get_unit(&self) -> Isp2PhysicalUnit {
        Isp2PhysicalUnit::from_u32(self.payload.Unit)
    }

    pub fn get_multiplier(&self) -> i8 {
        self.payload.Multiplier
    }

    pub fn get_value(&self) -> i16 {
        self.payload.Value
    }

    pub fn decode(payload: cglue::iso2_PhysicalValueType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::iso2_PhysicalValueType {
        self.payload
    }
}

#[derive(Clone, Debug)]
pub enum Iso2ParamValue {
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Text(String),
    PhyValue(PhysicalValue),
}

#[derive(Clone, Debug)]
pub struct Iso2ParamTuple {
    name: String,
    value: Iso2ParamValue,
}

impl Iso2ParamTuple {
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_value(&self) -> &Iso2ParamValue {
        &self.value
    }
}

#[derive(Clone)]
pub struct ParamSet {
    payload: cglue::iso2_ParameterSetType,
}

impl ParamSet {
    pub fn new(param_id: i16) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ParameterSetType>() };
        payload.ParameterSetID = param_id;
        Self { payload }
    }

    pub fn decode(payload: cglue::iso2_ParameterSetType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_ParameterSetType {
        self.payload
    }

    pub fn get_id(&self) -> i16 {
        self.payload.ParameterSetID
    }

    pub fn add_param(
        &mut self,
        prm_name: &str,
        prm_value: Iso2ParamValue,
    ) -> Result<&mut Self, AfbError> {
        let mut param = unsafe { mem::zeroed::<cglue::iso2_ParameterType>() };

        if self.payload.Parameter.arrayLen >= cglue::iso2_ParameterType_16_ARRAY_SIZE as u16 {
            return afb_error!(
                "iso2-param-set",
                "fail to add param (too many params max:{})",
                cglue::iso2_ParameterType_16_ARRAY_SIZE
            );
        }

        param.Name.charactersLen = str_to_array(
            prm_name,
            &mut param.Name.characters,
            cglue::iso2_Name_CHARACTER_SIZE,
        )?;
        match prm_value {
            Iso2ParamValue::Bool(data) => {
                if data {
                    param.byteValue = 1;
                } else {
                    param.byteValue = 0;
                }
                param.set_boolValue_isUsed(1);
            }
            Iso2ParamValue::Int8(data) => {
                param.byteValue = data;
                param.set_byteValue_isUsed(1);
            }
            Iso2ParamValue::Int16(data) => {
                param.shortValue = data;
                param.set_shortValue_isUsed(1);
            }
            Iso2ParamValue::Int32(data) => {
                param.intValue = data;
                param.set_intValue_isUsed(1);
            }
            Iso2ParamValue::Text(data) => {
                let len = str_to_array(
                    data.as_str(),
                    &mut param.stringValue.characters,
                    cglue::iso2_stringValue_CHARACTER_SIZE,
                )?;
                if len > 0 {
                    param.stringValue.charactersLen = len;
                    param.set_stringValue_isUsed(1);
                }
            }
            Iso2ParamValue::PhyValue(data) => {
                param.physicalValue = data.payload;
                param.set_physicalValue_isUsed(1);
            }
        };

        let idx = self.payload.Parameter.arrayLen;
        self.payload.Parameter.array[idx as usize] = param;
        self.payload.Parameter.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_params(&self) -> Result<Vec<Iso2ParamTuple>, AfbError> {
        let mut params = Vec::new();

        for idx in 0..self.payload.Parameter.arrayLen {
            let param = &self.payload.Parameter.array[idx as usize];
            let name = array_to_str(&param.Name.characters, param.Name.charactersLen)?.to_string();

            let value = if param.byteValue_isUsed() != 0 {
                let value = if param.boolValue == 0 { false } else { true };
                Iso2ParamValue::Bool(value)
            } else if param.byteValue_isUsed() != 0 {
                Iso2ParamValue::Int8(param.byteValue)
            } else if param.shortValue_isUsed() != 0 {
                Iso2ParamValue::Int16(param.shortValue)
            } else if param.intValue_isUsed() != 0 {
                Iso2ParamValue::Int32(param.intValue)
            } else if param.stringValue_isUsed() != 0 {
                Iso2ParamValue::Text(
                    array_to_str(
                        &param.stringValue.characters,
                        param.stringValue.charactersLen,
                    )?
                    .to_string(),
                )
            } else if param.physicalValue_isUsed() != 0 {
                let phys_value = PhysicalValue::new(
                    param.physicalValue.Value,
                    param.physicalValue.Multiplier,
                    Isp2PhysicalUnit::from_u32(param.physicalValue.Unit),
                );
                Iso2ParamValue::PhyValue(phys_value)
            } else {
                return afb_error!("param-set-param", "invalid param type");
            };

            params.push(Iso2ParamTuple { name, value });
        }
        Ok(params)
    }
}
