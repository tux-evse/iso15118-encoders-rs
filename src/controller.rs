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

use crate::prelude::*;
use std::sync::{Mutex, MutexGuard};

pub struct ControlerConfig {}

pub struct ControlerState {
    pub status: u32,
    pub protocol: SupportedAppProtocolTagId,
    session_id: SessionId,
    evccid: iso2::SessionSetupRequest,
}

pub struct IsoController {
    pub config: ControlerConfig,
    pub data_set: Mutex<ControlerState>,
}

impl IsoController {
    pub fn new() -> Result<Self, AfbError> {
        let state = Mutex::new(ControlerState {
            status: 0,
            protocol: SupportedAppProtocolTagId::Unknown,
            session_id: SessionId::null(),
            evccid: iso2::SessionSetupRequest::empty(),
        });
        let controler = IsoController {
            data_set: state,
            config: ControlerConfig {},
        };
        Ok(controler)
    }

    #[track_caller]
    pub fn lock_handle(&self) -> Result<MutexGuard<'_, ControlerState>, AfbError> {
        let guard = self.data_set.lock().unwrap();
        Ok(guard)
    }

    #[track_caller]
    pub fn get_protocol(&self) -> Result<SupportedAppProtocolTagId, AfbError> {
        let data_set = self.lock_handle()?;
        Ok(data_set.protocol.clone())
    }

    #[track_caller]
    pub fn set_protocol(&self, protocol: SupportedAppProtocolTagId) -> Result<(), AfbError> {
        let mut data_set = self.lock_handle()?;
        data_set.protocol = protocol;
        Ok(())
    }

    pub fn handle_exi_doc(
        &self,
        stream: &ExiStream,
        lock: &mut MutexGuard<RawStream>,
    ) -> Result<(), AfbError> {
        match self.get_protocol()? {
            SupportedAppProtocolTagId::Unknown => {
                // wait for supported_protocols list from EV
                let selected_proto = SupportedAppProtocolMsg::decode_msg(lock)?
                    .match_protocol()
                    .put_response(lock)?;

                self.set_protocol(selected_proto)?;
            }
            SupportedAppProtocolTagId::Iso2 => {
                use iso2::*;
                let message = Iso2Payload::decode(lock)?;
                let mut data_set = self.lock_handle()?;
                match message.get_payload() {
                    Iso2MessageBody::SessionSetupReq(request) => {
                        data_set.evccid = request.clone();

                        afb_log_msg!(
                            Debug,
                            None,
                            "SessionSetupReq evccid:[{}]",
                            dump_buffer(request.get_id())
                        );

                        // check if we are facing a new session
                        let session_id = message.get_session();
                        let status = if !data_set.session_id.equal(session_id) {
                            data_set.session_id = session_id.clone();
                            ResponseCode::NewSession
                        } else {
                            ResponseCode::Ok
                        };

                        // Fulup TBD this should comme from config
                        let evse_id = "tux-evse-001";
                        let body = SessionSetupResponse::new(evse_id, status)?.encode();

                        Iso2MessageExi::encode_to_stream(lock, &body, &data_set.session_id)?;
                    } //end SessionSetupReq

                    Iso2MessageBody::ServiceDiscoveryReq(request) => {
                        let scope = match request.get_scope() {
                            Some(value) => value.to_string(),
                            None => "no-scope-defined".to_string(),
                        };

                        afb_log_msg!(Debug, None, "DiscoverySvcReq optional scope:[{}]", scope);
                        let charging= ServiceCharging::new("Tux-Evse", "IoT-bzh", false);
                        let service = ServiceOther::new(56, "LTE", "Network", ServiceCategory::Internet, true);
                        let transfer= EngyTransfertMode::AcSinglePhase;

                        let body = ServiceDiscoveryResponse::new(ResponseCode::Ok)
                            .add_payment(PaymentOption::Contract)?
                            .set_charging(&charging)?
                            .add_service(&service)?
                            .add_transfer(transfer)?
                            .encode();

                        Iso2MessageExi::encode_to_stream(lock, &body, &data_set.session_id)?;
                    } // end DiscoverySvcReq

                    Iso2MessageBody::ServiceDetailReq(request) => {
                        afb_log_msg!(
                            Debug,
                            None,
                            "ServiceDetailReq service_id:{}",
                            request.get_id()
                        );

                        let mut param_set_1 = ParamSet::new(1);
                        param_set_1.add_param("prm_1", Iso2ParamValue::Int16(123))?;
                        param_set_1
                            .add_param("prm_2", Iso2ParamValue::Text("snoopy".to_string()))?;
                        param_set_1.add_param(
                            "prm_3",
                            Iso2ParamValue::PhyValue(PhysicalValue::new(
                                240,
                                1,
                                Isp2PhysicalUnit::Volt,
                            )),
                        )?;

                        let body =
                            ServiceDetailResponse::new(request.get_id(), ResponseCode::Ok)
                                .add_pset(&param_set_1)?
                                .encode();
                        Iso2MessageExi::encode_to_stream(lock, &body, &data_set.session_id)?;
                    } // end ServiceDetailReq

                    _ => {
                        return afb_error!(
                            "mgr_handle-exi",
                            "unsupported iso2 message:[{}]",
                            stream.dump_buffer(lock, ExiDump::Everything)?
                        )
                    }
                };
            }

            // unexpected request coming from EV
            _ => return afb_error!("controller-handle-exi", "unsupported exi document type"),
        }
        Ok(())
    }
}
