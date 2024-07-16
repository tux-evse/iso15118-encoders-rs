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

pub struct DcEvChargeParam {
    payload: cglue::din_DC_EVChargeParameterType,
}

impl DcEvChargeParam {
    #[track_caller]
    pub fn new(
        status: &DcEvStatusType,
        max_voltage: &PhysicalValue,
        max_current: &PhysicalValue,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_DC_EVChargeParameterType>() };

        if max_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "dc-ev-charge-param",
                "expect: PhysicalUnit::Ampere get:{}",
                max_current.get_unit()
            );
        }

        if max_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "dc-ev-charge-param",
                "expect: PhysicalUnit::Volt get:{}",
                max_voltage.get_unit()
            );
        }

        payload.DC_EVStatus = status.encode();
        payload.EVMaximumCurrentLimit = max_current.encode();
        payload.EVMaximumVoltageLimit = max_voltage.encode();
        Ok(Self { payload })
    }

    pub fn get_status(&self) -> DcEvStatusType {
        DcEvStatusType::decode(self.payload.DC_EVStatus)
    }

    pub fn get_max_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVMaximumVoltageLimit)
    }

    pub fn get_max_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVMaximumCurrentLimit)
    }

    #[track_caller]
    pub fn set_max_power(&mut self, power_limit: &PhysicalValue) -> Result<&mut Self, AfbError> {
        if power_limit.get_unit() != PhysicalUnit::Watt {
            return afb_error!(
                "dc-ev-charge-param",
                "expect: PhysicalUnit::Watt get:{}",
                power_limit.get_unit()
            );
        }
        self.payload.EVMaximumPowerLimit = power_limit.encode();
        self.payload.set_EVMaximumPowerLimit_isUsed(1);
        Ok(self)
    }

    pub fn get_max_power(&self) -> Option<PhysicalValue> {
        if self.payload.EVMaximumPowerLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVMaximumPowerLimit))
        }
    }

    #[track_caller]
    pub fn set_energy_capacity(
        &mut self,
        power_limit: &PhysicalValue,
    ) -> Result<&mut Self, AfbError> {
        if power_limit.get_unit() != PhysicalUnit::Wh {
            return afb_error!(
                "dc-ev-charge-param",
                "expect: PhysicalUnit::Wh get:{}",
                power_limit.get_unit()
            );
        }
        self.payload.EVEnergyCapacity = power_limit.encode();
        self.payload.set_EVEnergyCapacity_isUsed(1);
        Ok(self)
    }

    pub fn get_energy_capacity(&self) -> Option<PhysicalValue> {
        if self.payload.EVEnergyCapacity_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVEnergyCapacity))
        }
    }

    pub fn set_energy_request(
        &mut self,
        power_limit: &PhysicalValue,
    ) -> Result<&mut Self, AfbError> {
        if power_limit.get_unit() != PhysicalUnit::Wh {
            return afb_error!(
                "dc-ev-charge-param",
                "expect: PhysicalUnit::Wh get:{}",
                power_limit.get_unit()
            );
        }
        self.payload.EVEnergyRequest = power_limit.encode();
        self.payload.set_EVEnergyRequest_isUsed(1);
        Ok(self)
    }

    pub fn get_energy_request(&self) -> Option<PhysicalValue> {
        if self.payload.EVEnergyRequest_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVEnergyRequest))
        }
    }

    pub fn set_bulk_soc(&mut self, bulk_soc: i8) -> &mut Self {
        self.payload.BulkSOC = bulk_soc;
        self.payload.set_BulkSOC_isUsed(1);
        self
    }

    pub fn get_bulk_soc(&self) -> Option<i8> {
        if self.payload.BulkSOC_isUsed() == 0 {
            None
        } else {
            Some(self.payload.BulkSOC)
        }
    }

    pub fn set_full_soc(&mut self, full_soc: i8) -> &mut Self {
        self.payload.FullSOC = full_soc;
        self.payload.set_FullSOC_isUsed(1);
        self
    }

    pub fn get_full_soc(&self) -> Option<i8> {
        if self.payload.FullSOC_isUsed() == 0 {
            None
        } else {
            Some(self.payload.FullSOC)
        }
    }

    pub fn decode(payload: cglue::din_DC_EVChargeParameterType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_DC_EVChargeParameterType {
        self.payload
    }
}

pub struct AcEvChargeParam {
    payload: cglue::din_AC_EVChargeParameterType,
}

impl AcEvChargeParam {
    #[track_caller]
    pub fn new(
        ea_mount: &PhysicalValue,
        max_voltage: &PhysicalValue,
        max_current: &PhysicalValue,
        min_current: &PhysicalValue,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_AC_EVChargeParameterType>() };

        if max_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "ac-ev-charge-param",
                "expect: PhysicalUnit::Ampere get:{}",
                max_current.get_unit()
            );
        }

        if min_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "ac-ev-charge-param",
                "expect: PhysicalUnit::Ampere get:{}",
                max_current.get_unit()
            );
        }

        if max_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "ac-ev-charge-param",
                "expect: PhysicalUnit::Volt get:{}",
                max_current.get_unit()
            );
        }

        payload.EAmount = ea_mount.encode();
        payload.EVMaxVoltage = max_voltage.encode();
        payload.EVMaxCurrent = max_current.encode();
        payload.EVMinCurrent = min_current.encode();
        Ok(Self { payload })
    }

    pub fn get_ea_mount(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EAmount)
    }

    pub fn get_max_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVMaxVoltage)
    }

    pub fn get_max_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVMaxCurrent)
    }

    pub fn get_min_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVMinCurrent)
    }

    pub fn decode(payload: cglue::din_AC_EVChargeParameterType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_AC_EVChargeParameterType {
        self.payload
    }
}

pub struct EvChargeParam {
    payload: cglue::din_EVChargeParameterType,
}

impl EvChargeParam {
    #[track_caller]
    pub fn new(_unused: i32) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_EVChargeParameterType>() };
        payload._unused = _unused;
        Self { payload }
    }

    pub fn decode(payload: cglue::din_EVChargeParameterType) -> Self {
        Self { payload: payload }
    }

    pub fn encode(&self) -> cglue::din_EVChargeParameterType {
        self.payload
    }
}

pub struct ParamDiscoveryRequest {
    payload: cglue::din_ChargeParameterDiscoveryReqType,
}

impl ParamDiscoveryRequest {
    pub fn new(ev_request_transfer: EvRequestTransfertMode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ChargeParameterDiscoveryReqType>() };
        payload.EVRequestedEnergyTransferType = ev_request_transfer as u32;
        Self { payload }
    }

    pub fn get_transfert_energy_mode(&self) -> EvRequestTransfertMode {
        EvRequestTransfertMode::from_u32(self.payload.EVRequestedEnergyTransferType)
    }

    #[track_caller]
    pub fn set_ac_charge_param(
        &mut self,
        ac_params: &AcEvChargeParam,
    ) -> Result<&mut Self, AfbError> {
        if self.payload.DC_EVChargeParameter_isUsed() != 0
            || self.payload.EVChargeParameter_isUsed() != 0
        {
            return afb_error!(
                "param-discovery-request",
                "fail set_ac_charge_param because dc already set"
            );
        }

        match EvRequestTransfertMode::from_u32(self.payload.EVRequestedEnergyTransferType) {
            EvRequestTransfertMode::AcSinglePhase => {}
            EvRequestTransfertMode::AcTreePhase => {}
            _ => {
                return afb_error!(
                    "param-discovery-request",
                    "set_ac_charge_param incompatible with current RequestedEnergyTransferMode"
                )
            }
        }
        self.payload.AC_EVChargeParameter = ac_params.encode();
        self.payload.set_AC_EVChargeParameter_isUsed(1);
        Ok(self)
    }

    pub fn get_ac_charge_param(&self) -> Option<AcEvChargeParam> {
        if self.payload.AC_EVChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(AcEvChargeParam::decode(self.payload.AC_EVChargeParameter))
        }
    }

    #[track_caller]
    pub fn set_dc_charge_param(
        &mut self,
        dc_params: &DcEvChargeParam,
    ) -> Result<&mut Self, AfbError> {
        if self.payload.AC_EVChargeParameter_isUsed() != 0
            || self.payload.EVChargeParameter_isUsed() != 0
        {
            return afb_error!(
                "param-discovery-request",
                "fail set_dc_charge_param because ac already set"
            );
        }
        match EvRequestTransfertMode::from_u32(self.payload.EVRequestedEnergyTransferType) {
            EvRequestTransfertMode::DcBasic => {}
            EvRequestTransfertMode::DcExtended => {}
            EvRequestTransfertMode::DcCombo => {}
            EvRequestTransfertMode::DcUnique => {}
            _ => {
                return afb_error!(
                    "param-discovery-request",
                    "set_dc_charge_param incompatible with current RequestedEnergyTransferMode"
                )
            }
        }

        self.payload.DC_EVChargeParameter = dc_params.encode();
        self.payload.set_DC_EVChargeParameter_isUsed(1);
        Ok(self)
    }

    pub fn get_dc_charge_param(&self) -> Option<DcEvChargeParam> {
        if self.payload.DC_EVChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(DcEvChargeParam::decode(self.payload.DC_EVChargeParameter))
        }
    }

    pub fn get_ev_charge_param(&self) -> Option<EvChargeParam> {
        if self.payload.EVChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(EvChargeParam::decode(self.payload.EVChargeParameter))
        }
    }

    #[track_caller]
    pub fn set_ev_charge_param(
        &mut self,
        charge_params: &EvChargeParam,
    ) -> Result<&mut Self, AfbError> {
        if self.payload.DC_EVChargeParameter_isUsed() != 0
            || self.payload.AC_EVChargeParameter_isUsed() != 0
        {
            return afb_error!(
                "param-discovery-request",
                "fail set_charge_param bacause ac|dc already set"
            );
        }
        self.payload.EVChargeParameter = charge_params.encode();
        self.payload.set_EVChargeParameter_isUsed(1);
        Ok(self)
    }

    pub fn decode(payload: cglue::din_ChargeParameterDiscoveryReqType) -> Self {
        Self { payload: payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.ChargeParameterDiscoveryReq = self.payload;
            exi_body.set_ChargeParameterDiscoveryReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct SalesTariff {
    payload: cglue::din_SalesTariffType,
}

impl SalesTariff {
    pub fn new(name_id: &str, tariff_id: i16, price_levels: u8) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_SalesTariffType>() };
        payload.SalesTariffID = tariff_id;
        payload.Id.charactersLen = str_to_array(
            name_id,
            &mut payload.Id.characters,
            cglue::din_Id_CHARACTER_SIZE,
        )?;
        payload.NumEPriceLevels = price_levels;
        Ok(Self { payload })
    }

    pub fn get_id(&self) -> Result<&str, AfbError> {
        array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen)
    }

    pub fn get_tariff_id(&self) -> i16 {
        self.payload.SalesTariffID
    }

    pub fn get_price_level(&self) -> u8 {
        self.payload.NumEPriceLevels
    }

    #[track_caller]
    pub fn set_description(&mut self, text: &str) -> Result<&mut Self, AfbError> {
        self.payload.SalesTariffDescription.charactersLen = str_to_array(
            text,
            &mut self.payload.SalesTariffDescription.characters,
            cglue::din_SalesTariffDescription_CHARACTER_SIZE,
        )?;
        if self.payload.SalesTariffDescription.charactersLen > 0 {
            self.payload.set_SalesTariffDescription_isUsed(1);
        }
        Ok(self)
    }

    pub fn get_description(&self) -> Option<&str> {
        if self.payload.SalesTariffDescription_isUsed() == 0 {
            None
        } else {
            // if string not UTF8 return None
            array_to_str(
                &self.payload.SalesTariffDescription.characters,
                self.payload.SalesTariffDescription.charactersLen,
            )
            .ok()
        }
    }

    #[track_caller]
    pub fn add_entry(&mut self, entry: &SaleTariffEntry) -> Result<&mut Self, AfbError> {
        let idx = self.payload.SalesTariffEntry.arrayLen;
        if idx == cglue::din_SalesTariffEntryType_5_ARRAY_SIZE as u16 {
            return afb_error!("din-tarrif-entry", "fail to add tariff entry (array full)");
        }
        self.payload.SalesTariffEntry.array[idx as usize] = entry.encode();
        self.payload.SalesTariffEntry.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_entries(&self) -> Vec<SaleTariffEntry> {
        let mut entries = Vec::new();
        for idx in 0..self.payload.SalesTariffEntry.arrayLen as usize {
            entries.push(SaleTariffEntry::decode(
                self.payload.SalesTariffEntry.array[idx],
            ));
        }
        entries
    }

    pub fn decode(payload: cglue::din_SalesTariffType) -> Self {
        Self { payload: payload }
    }

    pub fn encode(&self) -> cglue::din_SalesTariffType {
        self.payload
    }
}

pub struct RelativeTimeInterval {
    payload: cglue::din_RelativeTimeIntervalType,
}

impl RelativeTimeInterval {
    pub fn new(start: u32) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_RelativeTimeIntervalType>() };
        payload.start = start;
        Self { payload }
    }

    pub fn get_start(&self) -> u32 {
        self.payload.start
    }

    pub fn set_duration(&mut self, duration: u32) -> &mut Self {
        self.payload.duration = duration;
        self.payload.set_duration_isUsed(1);
        self
    }

    pub fn get_duration(&self) -> Option<u32> {
        if self.payload.duration_isUsed() == 0 {
            None
        } else {
            Some(self.payload.duration)
        }
    }

    pub fn decode(payload: cglue::din_RelativeTimeIntervalType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_RelativeTimeIntervalType {
        self.payload
    }
}

pub struct PMaxScheduleEntry {
    payload: cglue::din_PMaxScheduleEntryType,
}

impl PMaxScheduleEntry {
    pub fn new(pmax: i16) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_PMaxScheduleEntryType>() };
        payload.PMax = pmax;

        Self { payload }
    }

    pub fn get_pmax(&self) -> i16 {
        self.payload.PMax
    }

    pub fn set_relative_time_interval(
        &mut self,
        relative_time: &RelativeTimeInterval,
    ) -> &mut Self {
        self.payload.RelativeTimeInterval = relative_time.encode();
        self.payload.set_RelativeTimeInterval_isUsed(1);
        self
    }

    pub fn get_relative_time_interval(&self) -> Option<RelativeTimeInterval> {
        if self.payload.RelativeTimeInterval_isUsed() == 0 {
            None
        } else {
            Some(RelativeTimeInterval::decode(
                self.payload.RelativeTimeInterval,
            ))
        }
    }

    pub fn set_time_interval(&mut self, _unused: i32) -> &mut Self {
        self.payload.TimeInterval._unused = _unused;
        self.payload.set_TimeInterval_isUsed(1);
        self
    }

    pub fn get_time_interval(&self) -> Option<i32> {
        if self.payload.TimeInterval_isUsed() == 0 {
            None
        } else {
            Some(self.payload.TimeInterval._unused)
        }
    }

    pub fn decode(payload: cglue::din_PMaxScheduleEntryType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_PMaxScheduleEntryType {
        self.payload
    }
}

pub struct PMaxSchedule {
    payload: cglue::din_PMaxScheduleType,
}

impl PMaxSchedule {
    pub fn new(id: i16) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_PMaxScheduleType>() };
        payload.PMaxScheduleID = id;
        Self { payload }
    }

    pub fn get_id(&self) -> i16 {
        self.payload.PMaxScheduleID
    }

    pub fn add_entry(&mut self, entry: &PMaxScheduleEntry) -> Result<&mut Self, AfbError> {
        let idx = self.payload.PMaxScheduleEntry.arrayLen;
        if idx == cglue::din_PMaxScheduleEntryType_5_ARRAY_SIZE as u16 {
            return afb_error!("pmax-schedule-add", "entry array full");
        }
        self.payload.PMaxScheduleEntry.array[idx as usize] = entry.encode();
        self.payload.PMaxScheduleEntry.arrayLen = idx + 1;

        Ok(self)
    }

    pub fn get_entries(&self) -> Vec<PMaxScheduleEntry> {
        let mut response = Vec::new();
        for idx in 0..self.payload.PMaxScheduleEntry.arrayLen {
            response.push(PMaxScheduleEntry::decode(
                self.payload.PMaxScheduleEntry.array[idx as usize],
            ))
        }
        response
    }

    pub fn decode(payload: cglue::din_PMaxScheduleType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_PMaxScheduleType {
        self.payload
    }
}

#[repr(u32)]
pub enum CostKind {
    PricePercent = cglue::din_costKindType_din_costKindType_relativePricePercentage,
    RenewGenPercent = cglue::din_costKindType_din_costKindType_RenewableGenerationPercentage,
    CarbonEmission = cglue::din_costKindType_din_costKindType_CarbonDioxideEmission,
}
impl CostKind {
    pub fn from_u32(value: u32) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

pub struct CostType {
    payload: cglue::din_CostType,
}

impl CostType {
    pub fn new(kind: CostKind, amount: u32) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_CostType>() };
        payload.costKind = kind as u32;
        payload.amount = amount;
        Self { payload }
    }

    pub fn set_multiplier(&mut self, multiplier: i8) -> &mut Self {
        self.payload.amountMultiplier = multiplier;
        self.payload.set_amountMultiplier_isUsed(1);
        self
    }

    pub fn get_multiplier(&self) -> Option<i8> {
        if self.payload.amountMultiplier_isUsed() == 0 {
            None
        } else {
            Some(self.payload.amountMultiplier)
        }
    }

    pub fn decode(payload: cglue::din_CostType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_CostType {
        self.payload
    }
}

pub struct ConsumptionCost {
    payload: cglue::din_ConsumptionCostType,
}

impl ConsumptionCost {
    pub fn new(start_value: u32) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ConsumptionCostType>() };
        payload.startValue = start_value;
        Self { payload }
    }

    #[track_caller]
    pub fn set_cost(&mut self, cost: &CostType) -> &mut Self {
        self.payload.Cost = cost.encode();
        self
    }

    pub fn get_costs(&self) -> CostType {
        CostType::decode(self.payload.Cost)
    }

    pub fn decode(payload: cglue::din_ConsumptionCostType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_ConsumptionCostType {
        self.payload
    }
}

pub struct SaleTariffEntry {
    payload: cglue::din_SalesTariffEntryType,
}

impl SaleTariffEntry {
    pub fn new(tariff_level: u8) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_SalesTariffEntryType>() };
        payload.EPriceLevel = tariff_level;
        Self { payload }
    }

    pub fn get_price_level(&self) -> u8 {
        self.payload.EPriceLevel
    }

    pub fn set_start(&mut self, start: u32) -> &mut Self {
        self.payload.set_RelativeTimeInterval_isUsed(1);
        self.payload.RelativeTimeInterval.start = start;
        self
    }

    pub fn get_start(&self) -> Option<u32> {
        if self.payload.RelativeTimeInterval_isUsed() == 0 {
            None
        } else {
            Some(self.payload.RelativeTimeInterval.start)
        }
    }

    pub fn set_duration(&mut self, duration: u32) -> &mut Self {
        self.payload.set_RelativeTimeInterval_isUsed(1);
        self.payload.RelativeTimeInterval.duration = duration;
        self
    }

    pub fn get_duration(&self) -> Option<u32> {
        if self.payload.RelativeTimeInterval_isUsed() == 0 {
            None
        } else {
            Some(self.payload.RelativeTimeInterval.duration)
        }
    }

    #[track_caller]
    pub fn set_comsumption_cost(&mut self, cost: ConsumptionCost) -> Result<&mut Self, AfbError> {
        self.payload.ConsumptionCost = cost.encode();
        Ok(self)
    }

    pub fn get_comsumption_cost(&mut self) -> ConsumptionCost {
        ConsumptionCost::decode(self.payload.ConsumptionCost)
    }

    pub fn decode(payload: cglue::din_SalesTariffEntryType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_SalesTariffEntryType {
        self.payload
    }
}
pub struct SasScheduleTuple {
    payload: cglue::din_SAScheduleTupleType,
}

impl SasScheduleTuple {
    pub fn new(id: i16, pmax_schedule: &PMaxSchedule) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_SAScheduleTupleType>() };
        payload.SAScheduleTupleID = id;
        payload.PMaxSchedule = pmax_schedule.encode();
        Self { payload }
    }

    pub fn get_id(&self) -> i16 {
        self.payload.SAScheduleTupleID
    }

    pub fn get_pmax_schedule(&self) -> PMaxSchedule {
        PMaxSchedule::decode(self.payload.PMaxSchedule)
    }

    pub fn set_tariff(&mut self, tariff: &SalesTariff) -> &mut Self {
        self.payload.SalesTariff = tariff.encode();
        self.payload.set_SalesTariff_isUsed(1);
        self
    }

    pub fn get_tariff(&self) -> Option<SalesTariff> {
        if self.payload.SalesTariff_isUsed() == 0 {
            None
        } else {
            Some(SalesTariff::decode(self.payload.SalesTariff))
        }
    }

    pub fn decode(payload: cglue::din_SAScheduleTupleType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_SAScheduleTupleType {
        self.payload
    }
}

pub struct AcEvseChargeParam {
    payload: cglue::din_AC_EVSEChargeParameterType,
}

impl AcEvseChargeParam {
    pub fn new(
        status: &AcEvseStatusType,
        max_voltage: &PhysicalValue,
        max_current: &PhysicalValue,
        min_current: &PhysicalValue,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_AC_EVSEChargeParameterType>() };

        payload.AC_EVSEStatus = status.encode();

        if max_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "av-evse-charge-param",
                "expect: PhysicalUnit::Volt get:{}",
                max_voltage.get_unit()
            );
        }
        payload.EVSEMaxVoltage = max_voltage.encode();

        if max_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "av-evse-charge-param",
                "expect: PhysicalUnit::Ampere get:{}",
                max_voltage.get_unit()
            );
        }
        payload.EVSEMaxCurrent = max_current.encode();

        if min_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "av-evse-charge-param",
                "expect: PhysicalUnit::Ampere get:{}",
                max_voltage.get_unit()
            );
        }
        payload.EVSEMinCurrent = min_current.encode();

        Ok(Self { payload })
    }

    pub fn get_status(&self) -> AcEvseStatusType {
        AcEvseStatusType::decode(self.payload.AC_EVSEStatus)
    }

    pub fn get_maximum_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMaxVoltage)
    }

    pub fn get_max_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMaxCurrent)
    }

    pub fn get_min_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMinCurrent)
    }

    pub fn decode(payload: cglue::din_AC_EVSEChargeParameterType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_AC_EVSEChargeParameterType {
        self.payload
    }
}

pub struct DcEvseChargeParam {
    payload: cglue::din_DC_EVSEChargeParameterType,
}

impl DcEvseChargeParam {
    #[track_caller]
    pub fn new(
        status: &DcEvseStatusType,
        max_voltage: &PhysicalValue,
        min_voltage: &PhysicalValue,
        max_current: &PhysicalValue,
        min_current: &PhysicalValue,
        max_power: &PhysicalValue,
        current_ripple: &PhysicalValue,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_DC_EVSEChargeParameterType>() };

        if max_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "dc-charge-param",
                "expect: PhysicalUnit::Volt get:{}",
                max_voltage.get_unit()
            );
        }
        if min_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "dc-charge-param",
                "expect: PhysicalUnit::Volt get:{}",
                min_voltage.get_unit()
            );
        }
        if max_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "pre-charge-req",
                "expect: PhysicalUnit::Ampere get:{}",
                max_current.get_unit()
            );
        }
        if min_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "pre-charge-req",
                "expect: PhysicalUnit::Ampere get:{}",
                min_current.get_unit()
            );
        }
        if max_power.get_unit() != PhysicalUnit::Watt {
            return afb_error!(
                "pre-charge-req",
                "expect: PhysicalUnit::Watt get:{}",
                max_power.get_unit()
            );
        }
        if current_ripple.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "pre-charge-req",
                "expect: PhysicalUnit::Volt get:{}",
                current_ripple.get_unit()
            );
        }

        payload.DC_EVSEStatus = status.encode();
        payload.EVSEMaximumCurrentLimit = max_current.encode();
        payload.EVSEMaximumVoltageLimit = max_voltage.encode();
        payload.EVSEMaximumPowerLimit = max_power.encode();
        payload.EVSEMinimumCurrentLimit = min_current.encode();
        payload.EVSEMinimumVoltageLimit = min_voltage.encode();
        payload.EVSEPeakCurrentRipple = current_ripple.encode();

        Ok(Self { payload })
    }

    pub fn get_status(&self) -> DcEvseStatusType {
        DcEvseStatusType::decode(self.payload.DC_EVSEStatus)
    }

    pub fn get_max_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMaximumVoltageLimit)
    }

    pub fn get_min_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMinimumVoltageLimit)
    }

    pub fn get_max_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMaximumCurrentLimit)
    }

    pub fn get_min_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMinimumCurrentLimit)
    }
    pub fn get_max_power(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMaximumPowerLimit)
    }

    pub fn set_regul_tolerance(&mut self, tolerance: &PhysicalValue) -> Result<&mut Self, AfbError> {
        if tolerance.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "dc-ev-charge-param",
                "regul_tolerance expect: PhysicalUnit::Ampere get:{}",
                tolerance.get_unit()
            );
        }
        self.payload.EVSECurrentRegulationTolerance = tolerance.encode();
        self.payload.set_EVSECurrentRegulationTolerance_isUsed(1);
        Ok(self)
    }

    pub fn get_regul_tolerance(&self) -> Option<PhysicalValue> {
        if self.payload.EVSECurrentRegulationTolerance_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(
                self.payload.EVSECurrentRegulationTolerance,
            ))
        }
    }

    pub fn set_energy_to_deliver(&mut self, energy: &PhysicalValue) -> Result<&mut Self, AfbError> {
        if energy.get_unit() != PhysicalUnit::Wh {
            return afb_error!(
                "dc-ev-charge-param",
                "energy_to_deliver expect: PhysicalUnit::Wh get:{}",
                energy.get_unit()
            );
        }
        self.payload.EVSEEnergyToBeDelivered = energy.encode();
        self.payload.set_EVSEEnergyToBeDelivered_isUsed(1);
        Ok(self)
    }

    pub fn get_energy_to_deliver(&self) -> Option<PhysicalValue> {
        if self.payload.EVSEEnergyToBeDelivered_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVSEEnergyToBeDelivered))
        }
    }

    pub fn get_peak_current_ripple(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEPeakCurrentRipple)
    }

    pub fn decode(payload: cglue::din_DC_EVSEChargeParameterType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_DC_EVSEChargeParameterType {
        self.payload
    }
}

pub struct ParamDiscoveryResponse {
    payload: cglue::din_ChargeParameterDiscoveryResType,
}

impl ParamDiscoveryResponse {
    pub fn new(code: ResponseCode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ChargeParameterDiscoveryResType>() };
        payload.ResponseCode = code as u32;
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn set_schedules(&mut self, unused: i32) -> &mut Self {
        self.payload.set_SASchedules_isUsed(1);
        self.payload.SASchedules._unused = unused;
        self
    }

    pub fn get_schedules(&self) -> Option<i32> {
        if self.payload.SASchedules_isUsed() == 0 {
            None
        } else {
            Some(self.payload.SASchedules._unused)
        }
    }

    pub fn set_evse_charge_param(&mut self, unused: i32) -> &mut Self {
        self.payload.set_EVSEChargeParameter_isUsed(1);
        self.payload.SASchedules._unused = unused;
        self
    }

    pub fn get_evse_charge_param(&self) -> Option<i32> {
        if self.payload.EVSEChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(self.payload.SASchedules._unused)
        }
    }

    #[track_caller]
    pub fn add_schedule_tuple(&mut self, tuple: &SasScheduleTuple) -> Result<&mut Self, AfbError> {
        let idx = self.payload.SAScheduleList.SAScheduleTuple.arrayLen;
        if idx == cglue::din_SAScheduleTupleType_5_ARRAY_SIZE as u16 {
            return afb_error!(
                "din-param-disco-res",
                "fail to add schedule_tuple array full"
            );
        }
        self.payload.SAScheduleList.SAScheduleTuple.array[idx as usize] = tuple.encode();
        self.payload.SAScheduleList.SAScheduleTuple.arrayLen = idx + 1;
        self.payload.set_SAScheduleList_isUsed(1);
        Ok(self)
    }

    pub fn get_schedule_tuples(&self) -> Vec<SasScheduleTuple> {
        let mut tuples = Vec::new();
        for idx in 0..self.payload.SAScheduleList.SAScheduleTuple.arrayLen as usize {
            tuples.push(SasScheduleTuple::decode(
                self.payload.SAScheduleList.SAScheduleTuple.array[idx],
            ));
        }
        tuples
    }

    pub fn set_evse_dc_charge_param(&mut self, dc_charge_param: &DcEvseChargeParam) -> &mut Self {
        self.payload.DC_EVSEChargeParameter = dc_charge_param.encode();
        self.payload.set_DC_EVSEChargeParameter_isUsed(1);
        self
    }

    pub fn get_evse_dc_charge_param(&self) -> Option<DcEvseChargeParam> {
        if self.payload.DC_EVSEChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(DcEvseChargeParam::decode(
                self.payload.DC_EVSEChargeParameter,
            ))
        }
    }

    pub fn set_evse_ac_charge_param(&mut self, ac_charge_param: &AcEvseChargeParam) -> &mut Self {
        self.payload.AC_EVSEChargeParameter = ac_charge_param.encode();
        self.payload.set_AC_EVSEChargeParameter_isUsed(1);
        self
    }

    pub fn get_evse_ac_charge_param(&self) -> Option<AcEvseChargeParam> {
        if self.payload.AC_EVSEChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(AcEvseChargeParam::decode(
                self.payload.AC_EVSEChargeParameter,
            ))
        }
    }

    pub fn decode(payload: cglue::din_ChargeParameterDiscoveryResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.ChargeParameterDiscoveryRes = self.payload;
            exi_body.set_ChargeParameterDiscoveryRes_isUsed(1);
            exi_body
        };
        body
    }
}
