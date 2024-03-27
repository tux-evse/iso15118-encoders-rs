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
 * Reference:
 */

use crate::prelude::*;

pub struct Iso2Payload {
    payload: Iso2MessageExi,
}

impl Iso2Payload {
    pub fn decode(stream_lock: &RawStream) -> Result<Self, AfbError> {
        let payload = Iso2MessageExi::decode_from_stream(stream_lock)?;
        stream_lock.reset();
        Ok(Iso2Payload { payload })
    }

    pub fn get_payload(&self) -> &Iso2MessageBody {
        &self.payload.body
    }

    pub fn get_session(&self) -> &SessionId {
        &self.payload.get_session()
    }

    // Fulup TBD
    // get notification
    // get signature

}
