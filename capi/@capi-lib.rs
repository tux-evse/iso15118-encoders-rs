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

// cargo test --package iso15118 --test test-v2g
#[path = "pki-sign/capi-pki.rs"]
mod pki_sign;

#[path = "exi-encoder.rs"]
mod exi_encoder;

#[path = "v2g-messages/@v2g-lib.rs"]
mod v2g_encoder;

#[path = "iso2-messages/@iso2-lib.rs"]
mod iso2_encoder;

#[path = "iso20-messages/@iso20-lib.rs"]
mod iso20_encoder;

#[path = "din-messages/@din-lib.rs"]
mod din_encoder;

//
// Add a get_<$name>(&self) -> &str method in an impl that reads a
// string field made of a [i8] ($string_field) and a length ($string_len_field)
#[macro_export]
macro_rules! string_field_getter {
    ( $( $string_field:ident ).+, $( $string_len_field:ident ).+, $name:ident ) => {
        paste::paste! {
        pub fn [<get_ $name>](&self) -> &str {
                str::from_utf8(unsafe {
                    &*((&self.$( $string_field ).+[0..self.$( $string_len_field ).+ as usize])
                        as *const [i8] as *const [u8])
                })
                .unwrap()
        }
        }
    };
}

//
// Add a set_<$name>(&self, input_str: &str, max_length: usize) -> Result<(), AfbError>
// method in an impl that writes in a string field made of a [i8]
// ($string_field) and a length ($string_len_field). The input string
// cannot be larger than $max_length
#[macro_export]
macro_rules! string_field_setter {
    ( $( $string_field:ident ).+, $( $string_len_field:ident ).+, $name:ident, $max_length:expr ) => {
        paste::paste! {
        pub fn [<set_ $name>](&mut self, input_str: &str) -> Result<(), AfbError> {
            self.$( $string_len_field ).+ = str_to_array(input_str, &mut self.$( $string_field ).+, $max_length)?;
            Ok(())
        }
        }
    };
}

#[macro_export]
macro_rules! string_field_getter_and_setter {
    ( $( $string_field:ident ).+, $( $string_len_field:ident ).+, $name:ident, $max_length:expr ) => {
        $crate::string_field_getter!($( $string_field ).+, $( $string_len_field ).+, $name);
        $crate::string_field_setter!($( $string_field ).+, $( $string_len_field ).+, $name, $max_length);
    };
}

pub mod prelude {
    pub use crate::capi::din_encoder::*;
    pub use crate::capi::exi_encoder::*;
    pub use crate::capi::iso20_encoder::*;
    pub use crate::capi::iso2_encoder::*;
    pub use crate::capi::pki_sign::*;
    pub use crate::capi::v2g_encoder::*;
    pub use afbv4::prelude::*;
}
