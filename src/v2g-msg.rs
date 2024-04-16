/*
 * Copyright (C) 2015-2022 IoT.bzh Pionix, Chargebyte and Everest contributors
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Rust largely inspired from Everest C++ git@github.com:/EVerest/libiso15118.git
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 */

use crate::prelude::v2g::*;

pub const V2G_PROTOCOLS_SUPPORTED_LIST: [&SupportedAppProtocolConf; 3] = [
    &SupportedAppProtocolConf {
        name: "urn:iso:15118:20:2022:MsgDef",
        tag_id: SupportedAppProtocolTagId::Iso20,
        major: 2,
        minor: 0,
    },
    &SupportedAppProtocolConf {
        tag_id: SupportedAppProtocolTagId::Iso2,
        name: "urn:iso:15118:2:2013:MsgDef",
        major: 2,
        minor: 0,
    },
    &SupportedAppProtocolConf {
        tag_id: SupportedAppProtocolTagId::Din,
        name: "urn:din:70121:2012:MsgDef",
        major: 2,
        minor: 0,
    }
];
