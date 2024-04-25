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
            "(value:{}, multiplier:{} unit:{})",
            value, multiplier, unit
        )
    }
}

impl PhysicalValue {
    pub fn new(value: i16, multiplier: i8, unit: PhysicalUnit) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PhysicalValueType>() };
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
pub enum ParamValue {
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Text(String),
    PhyValue(PhysicalValue),
}

#[derive(Clone, Debug)]
pub struct ParamTuple {
    name: String,
    value: ParamValue,
}

impl ParamTuple {
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_value(&self) -> &ParamValue {
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
        prm_value: ParamValue,
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
            ParamValue::Bool(data) => {
                if data {
                    param.byteValue = 1;
                } else {
                    param.byteValue = 0;
                }
                param.set_boolValue_isUsed(1);
            }
            ParamValue::Int8(data) => {
                param.byteValue = data;
                param.set_byteValue_isUsed(1);
            }
            ParamValue::Int16(data) => {
                param.shortValue = data;
                param.set_shortValue_isUsed(1);
            }
            ParamValue::Int32(data) => {
                param.intValue = data;
                param.set_intValue_isUsed(1);
            }
            ParamValue::Text(data) => {
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
            ParamValue::PhyValue(data) => {
                param.physicalValue = data.payload;
                param.set_physicalValue_isUsed(1);
            }
        };

        let idx = self.payload.Parameter.arrayLen;
        self.payload.Parameter.array[idx as usize] = param;
        self.payload.Parameter.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_params(&self) -> Result<Vec<ParamTuple>, AfbError> {
        let mut params = Vec::new();

        for idx in 0..self.payload.Parameter.arrayLen {
            let param = &self.payload.Parameter.array[idx as usize];
            let name = array_to_str(&param.Name.characters, param.Name.charactersLen)?.to_string();

            let value = if param.byteValue_isUsed() != 0 {
                let value = if param.boolValue == 0 { false } else { true };
                ParamValue::Bool(value)
            } else if param.byteValue_isUsed() != 0 {
                ParamValue::Int8(param.byteValue)
            } else if param.shortValue_isUsed() != 0 {
                ParamValue::Int16(param.shortValue)
            } else if param.intValue_isUsed() != 0 {
                ParamValue::Int32(param.intValue)
            } else if param.stringValue_isUsed() != 0 {
                ParamValue::Text(
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
                    PhysicalUnit::from_u32(param.physicalValue.Unit),
                );
                ParamValue::PhyValue(phys_value)
            } else {
                return afb_error!("param-set-param", "invalid param type");
            };

            params.push(ParamTuple { name, value });
        }
        Ok(params)
    }
}
