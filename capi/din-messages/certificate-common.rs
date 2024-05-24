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

pub struct IssuerSerialType {
    payload: cglue::din_X509IssuerSerialType,
}

impl IssuerSerialType {
    pub fn new(issuer_name: &str, serial_number: i32) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_X509IssuerSerialType>() };

        payload.X509IssuerName.charactersLen = str_to_array(
            issuer_name,
            &mut payload.X509IssuerName.characters,
            cglue::din_X509IssuerName_CHARACTER_SIZE,
        )?;
        payload.X509SerialNumber = serial_number;
        Ok(Self { payload })
    }

    pub fn get_issuer(&self) -> Result<&str, AfbError> {
        array_to_str(
            &self.payload.X509IssuerName.characters,
            self.payload.X509IssuerName.charactersLen,
        )
    }

    pub fn get_serial(&self) -> i32 {
        self.payload.X509SerialNumber
    }

    pub fn decode(payload: cglue::din_X509IssuerSerialType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_X509IssuerSerialType {
        self.payload
    }
}

pub struct CertificateRootList {
    payload: cglue::din_ListOfRootCertificateIDsType,
}

impl CertificateRootList {
    pub fn new(cert: &str) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ListOfRootCertificateIDsType>() };
        let certificate = &mut payload.RootCertificateID.array[0];
        certificate.charactersLen = str_to_array(
            cert,
            &mut certificate.characters,
            cglue::din_RootCertificateID_CHARACTER_SIZE,
        )?;
        payload.RootCertificateID.arrayLen = 1;
        Ok(Self { payload })
    }

    pub fn add_cert(&mut self, cert: &str) -> Result<&mut Self, AfbError> {
        let idx = self.payload.RootCertificateID.arrayLen;
        if idx == cglue::din_rootCertificateIDType_5_ARRAY_SIZE as u16 {
            return afb_error!("cert-list-add", "reach max:{} root certificate", idx);
        }
        let certificate = &mut self.payload.RootCertificateID.array[idx as usize];
        certificate.charactersLen = str_to_array(
            cert,
            &mut certificate.characters,
            cglue::din_X509IssuerName_CHARACTER_SIZE,
        )?;
        self.payload.RootCertificateID.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_certs(&self) -> Result<Vec<String>, AfbError> {
        let mut certs = Vec::new();
        for idx in 0..self.payload.RootCertificateID.arrayLen {
            let data = self.payload.RootCertificateID.array[idx as usize];
            let cert = array_to_str(&data.characters, data.charactersLen)?.to_string();
            certs.push(cert)
        }
        Ok(certs)
    }

    pub fn get_len(&self) -> usize {
        self.payload.RootCertificateID.arrayLen as usize
    }

    pub fn decode(payload: cglue::din_ListOfRootCertificateIDsType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_ListOfRootCertificateIDsType {
        self.payload
    }
}

pub struct CertificateChainType {
    payload: cglue::din_CertificateChainType,
}

impl CertificateChainType {
    pub fn new(cert_data: &[u8]) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_CertificateChainType>() };
        payload.Certificate.bytesLen = bytes_to_array(
            cert_data,
            &mut payload.Certificate.bytes,
            cglue::din_certificateType_BYTES_SIZE,
        )?;

        Ok(Self { payload })
    }

    pub fn get_cert(&self) -> &[u8] {
        array_to_bytes(
            &self.payload.Certificate.bytes,
            self.payload.Certificate.bytesLen,
        )
    }

    pub fn set_subcert(&mut self, cert: &[u8]) -> Result<&mut Self, AfbError> {
        self.payload.SubCertificates.Certificate.bytesLen = bytes_to_array(
            cert,
            &mut self.payload.SubCertificates.Certificate.bytes,
            cglue::din_certificateType_BYTES_SIZE,
        )?;
        self.payload.set_SubCertificates_isUsed(1);
        Ok(self)
    }

    pub fn get_subcert(&self) -> Option<&[u8]> {
        if self.payload.SubCertificates_isUsed() == 0 {
            None
        } else {
            Some(array_to_bytes(
                &self.payload.SubCertificates.Certificate.bytes,
                self.payload.SubCertificates.Certificate.bytesLen,
            ))
        }
    }

    pub fn decode(payload: cglue::din_CertificateChainType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::din_CertificateChainType {
        self.payload
    }
}
