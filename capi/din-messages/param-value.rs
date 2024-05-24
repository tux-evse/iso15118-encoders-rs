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
use std::fmt;
use std::mem;

#[derive(Clone, Copy)]
pub struct PhysicalValue {
    payload: cglue::din_PhysicalValueType,
}
impl fmt::Debug for PhysicalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.get_value();
        let multiplier = self.get_multiplier();
        let unit = self.get_unit();
        write!(
            f,
            "(value:{}, multiplier:{} unit:{})",
            value, multiplier, unit
        )
    }
}

impl PhysicalValue {
    pub fn new(value: i16, multiplier: i8, unit: PhysicalUnit) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_PhysicalValueType>() };
        payload.Multiplier = multiplier;
        payload.Unit = unit as u32;
        payload.Value = value;
        Self { payload }
    }

    pub fn get_unit(&self) -> PhysicalUnit {
        PhysicalUnit::from_u32(self.payload.Unit)
    }

    pub fn get_multiplier(&self) -> i8 {
        self.payload.Multiplier
    }

    pub fn get_value(&self) -> i16 {
        self.payload.Value
    }

    pub fn decode(payload: cglue::din_PhysicalValueType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::din_PhysicalValueType {
        self.payload
    }
}

#[derive(Clone, Debug)]
pub enum ParamValue {
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Text(String),
    PhyValue(PhysicalValue),
}


pub struct ParamTuple {
    payload: cglue::din_ParameterType,
}

impl ParamTuple {
    pub fn new(prm_name: &str, prm_value: &ParamValue) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ParameterType>() };

        payload.Name.charactersLen = str_to_array(
            prm_name,
            &mut payload.Name.characters,
            cglue::din_Name_CHARACTER_SIZE,
        )?;
        match prm_value {
            ParamValue::Bool(data) => {
                if *data {
                    payload.byteValue = 1;
                } else {
                    payload.byteValue = 0;
                }
                payload.set_boolValue_isUsed(1);
            }
            ParamValue::Int8(data) => {
                payload.byteValue = *data;
                payload.set_byteValue_isUsed(1);
            }
            ParamValue::Int16(data) => {
                payload.shortValue = *data;
                payload.set_shortValue_isUsed(1);
            }
            ParamValue::Int32(data) => {
                payload.intValue = *data;
                payload.set_intValue_isUsed(1);
            }
            ParamValue::Text(data) => {
                let len = str_to_array(
                    data.as_str(),
                    &mut payload.stringValue.characters,
                    cglue::din_stringValue_CHARACTER_SIZE,
                )?;
                if len > 0 {
                    payload.stringValue.charactersLen = len;
                    payload.set_stringValue_isUsed(1);
                }
            }
            ParamValue::PhyValue(data) => {
                payload.physicalValue = data.payload;
                payload.set_physicalValue_isUsed(1);
            }
        };
        Ok(Self { payload })
    }

    pub fn get_name(&self) -> Result<&str, AfbError> {
        array_to_str(
            &self.payload.Name.characters,
            self.payload.Name.charactersLen,
        )
    }
    pub fn get_value(&self) -> Result<ParamValue, AfbError> {
        let value = if self.payload.byteValue_isUsed() != 0 {
            let value = if self.payload.boolValue == 0 {
                false
            } else {
                true
            };
            ParamValue::Bool(value)
        } else if self.payload.byteValue_isUsed() != 0 {
            ParamValue::Int8(self.payload.byteValue)
        } else if self.payload.shortValue_isUsed() != 0 {
            ParamValue::Int16(self.payload.shortValue)
        } else if self.payload.intValue_isUsed() != 0 {
            ParamValue::Int32(self.payload.intValue)
        } else if self.payload.stringValue_isUsed() != 0 {
            ParamValue::Text(
                array_to_str(
                    &self.payload.stringValue.characters,
                    self.payload.stringValue.charactersLen,
                )?
                .to_string(),
            )
        } else if self.payload.physicalValue_isUsed() != 0 {
            let phys_value = PhysicalValue::new(
                self.payload.physicalValue.Value,
                self.payload.physicalValue.Multiplier,
                PhysicalUnit::from_u32(self.payload.physicalValue.Unit),
            );
            ParamValue::PhyValue(phys_value)
        } else {
            return afb_error!("param-set-param", "invalid param type");
        };
        Ok(value)
    }

    pub fn decode(payload: cglue::din_ParameterType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_ParameterType {
        self.payload
    }
}

#[derive(Clone)]
pub struct ParamSet {
    payload: cglue::din_ParameterSetType,
}

impl ParamSet {
    pub fn new(prm_id: i16, param: &ParamTuple) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ParameterSetType>() };
        payload.ParameterSetID = prm_id;

        payload.Parameter= param.encode();
        Self { payload }
    }

    pub fn get_id(&self) -> i16 {
        self.payload.ParameterSetID
    }

    pub fn get_param(&self) -> ParamTuple {
        ParamTuple::decode(self.payload.Parameter)
    }

    pub fn decode(payload: cglue::din_ParameterSetType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_ParameterSetType {
        self.payload
    }
}
