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

#[derive(Clone, Debug)]
pub struct CertificateData {
    issuer_name: String,
    serial_number: i32,
}

impl CertificateData {
    pub fn get_issuer(&self) -> String {
        self.issuer_name.clone()
    }
    pub fn get_serial(&self) -> i32 {
        self.serial_number
    }
}

pub struct CertificateRootList {
    payload: cglue::iso2_ListOfRootCertificateIDsType,
}

impl CertificateRootList {
    pub fn new(issuer_name: &str, serial_number: i32) -> Result<Self, AfbError> {
        let payload = unsafe { mem::zeroed::<cglue::iso2_ListOfRootCertificateIDsType>() };
        let mut certificate = payload.RootCertificateID.array[0];
        certificate.X509IssuerName.charactersLen = str_to_array(
            issuer_name,
            &mut certificate.X509IssuerName.characters,
            cglue::iso2_X509IssuerName_CHARACTER_SIZE,
        )?;
        certificate.X509SerialNumber = serial_number;
        Ok(Self { payload })
    }

    pub fn add_cert(
        &mut self,
        issuer_name: &str,
        serial_number: i32,
    ) -> Result<&mut Self, AfbError> {
        let idx = self.payload.RootCertificateID.arrayLen;
        if idx == cglue::iso2_X509IssuerSerialType_5_ARRAY_SIZE as u16 {
            return afb_error!("cert-list-add", "reach max:{} root certificate", idx);
        }
        let mut certificate = self.payload.RootCertificateID.array[idx as usize];
        certificate.X509IssuerName.charactersLen = str_to_array(
            issuer_name,
            &mut certificate.X509IssuerName.characters,
            cglue::iso2_X509IssuerName_CHARACTER_SIZE,
        )?;
        certificate.X509SerialNumber = serial_number;
        self.payload.RootCertificateID.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_certs(&self) -> Result<Vec<CertificateData>, AfbError> {
        let mut certs = Vec::new();
        for idx in 0..self.payload.RootCertificateID.arrayLen {
            let data = self.payload.RootCertificateID.array[idx as usize];
            let cert = CertificateData {
                issuer_name: array_to_str(
                    &data.X509IssuerName.characters,
                    data.X509IssuerName.charactersLen,
                )?.to_string(),
                serial_number: data.X509SerialNumber,
            };
            certs.push(cert)
        }
        Ok(certs)
    }

    pub fn get_len(&self) -> usize {
        self.payload.RootCertificateID.arrayLen as usize
    }

    pub fn decode(payload: cglue::iso2_ListOfRootCertificateIDsType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_ListOfRootCertificateIDsType {
        self.payload
    }
}


pub struct CertificateChainType {
    payload: cglue::iso2_CertificateChainType,
}

impl CertificateChainType {
    pub fn new(cert_id: &str, cert_data: &[u8]) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_CertificateChainType>() };
        payload.Certificate.bytesLen = bytes_to_array(
            cert_data,
            &mut payload.Certificate.bytes,
            cglue::iso2_certificateType_BYTES_SIZE,
        )?;

        payload.Id.charactersLen = str_to_array(
            cert_id,
            &mut payload.Id.characters,
            cglue::iso2_Id_CHARACTER_SIZE,
        )?;
        payload.set_Id_isUsed(1);

        Ok(Self{payload})
    }

    pub fn get_id(&self) -> Option<&str> {
        if self.payload.Id_isUsed() == 0 {
            None
        } else {
            array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen).ok()
        }
    }

    pub fn get_cert(&self) -> &[u8] {
        array_to_bytes(&self.payload.Certificate.bytes, self.payload.Certificate.bytesLen)
    }

    pub fn add_subcert(&mut self, cert: &[u8]) -> Result<&mut Self, AfbError> {
        let idx = self.payload.SubCertificates.Certificate.arrayLen;
        if idx == cglue::iso2_certificateType_4_ARRAY_SIZE as u16 {
            return afb_error!("cert-chain-add", "max:{} sub certificates reached", idx);
        }
        let subcert = &mut self.payload.SubCertificates.Certificate.array[idx as usize];
        subcert.bytesLen = bytes_to_array(
            cert,
            &mut subcert.bytes,
            cglue::iso2_certificateType_BYTES_SIZE,
        )?;
        self.payload.SubCertificates.Certificate.arrayLen= idx +1;
        self.payload.set_SubCertificates_isUsed(1);
        Ok(self)
    }

    pub fn get_subcerts(&self) -> Vec<&[u8]> {
        let mut subcerts = Vec::new();
        for idx in 0..self.payload.SubCertificates.Certificate.arrayLen {
            let subcert = &self.payload.SubCertificates.Certificate.array[idx as usize];
            //subcerts.push(array_to_bytes(&subcert.bytes, subcert.bytesLen).to_vec());
            subcerts.push(array_to_bytes(&subcert.bytes, subcert.bytesLen));
        }
        subcerts
    }

    pub fn decode(payload: cglue::iso2_CertificateChainType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_CertificateChainType {
        self.payload
    }
}

pub struct PrivateKeyType {
    payload: cglue::iso2_ContractSignatureEncryptedPrivateKeyType,
}

impl PrivateKeyType {
    pub fn new(key_id: &str, data: &[u8]) -> Result<Self, AfbError> {
        let mut payload =
            unsafe { mem::zeroed::<cglue::iso2_ContractSignatureEncryptedPrivateKeyType>() };
        payload.Id.charactersLen = str_to_array(
            key_id,
            &mut payload.Id.characters,
            cglue::iso2_Id_CHARACTER_SIZE,
        )?;
        payload.CONTENT.bytesLen = bytes_to_array(
            data,
            &mut payload.CONTENT.bytes,
            cglue::iso2_ContractSignatureEncryptedPrivateKeyType_BYTES_SIZE,
        )?;
        Ok(Self { payload })
    }

    pub fn get_id(&self) -> Result<&str, AfbError> {
        array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen)
    }

    pub fn get_data(&self) -> &[u8] {
        array_to_bytes(&self.payload.CONTENT.bytes, self.payload.CONTENT.bytesLen)
    }

    pub fn decode(payload: cglue::iso2_ContractSignatureEncryptedPrivateKeyType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_ContractSignatureEncryptedPrivateKeyType {
        self.payload
    }

}

pub struct DhPublicKeyType {
    payload: cglue::iso2_DiffieHellmanPublickeyType,
}

impl DhPublicKeyType {
    pub fn new(key_id: &str, data: &[u8]) -> Result<Self, AfbError> {
        let mut payload =
            unsafe { mem::zeroed::<cglue::iso2_DiffieHellmanPublickeyType>() };
        payload.Id.charactersLen = str_to_array(
            key_id,
            &mut payload.Id.characters,
            cglue::iso2_Id_CHARACTER_SIZE,
        )?;
        payload.CONTENT.bytesLen = bytes_to_array(
            data,
            &mut payload.CONTENT.bytes,
            cglue::iso2_DiffieHellmanPublickeyType_BYTES_SIZE,
        )?;
        Ok(Self { payload })
    }

    pub fn get_id(&self) -> Result<&str, AfbError> {
        array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen)
    }

    pub fn get_data(&self) -> &[u8] {
        array_to_bytes(&self.payload.CONTENT.bytes, self.payload.CONTENT.bytesLen)
    }

    pub fn decode(payload: cglue::iso2_DiffieHellmanPublickeyType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_DiffieHellmanPublickeyType {
        self.payload
    }

}

pub struct EmaidType {
    payload: cglue::iso2_EMAIDType,
}

impl EmaidType {
    pub fn new(key_id: &str, data: &str) -> Result<Self, AfbError> {
        let mut payload =
            unsafe { mem::zeroed::<cglue::iso2_EMAIDType>() };
        payload.Id.charactersLen = str_to_array(
            key_id,
            &mut payload.Id.characters,
            cglue::iso2_Id_CHARACTER_SIZE,
        )?;
        payload.CONTENT.charactersLen = str_to_array(
            data,
            &mut payload.CONTENT.characters,
            cglue::iso2_CONTENT_CHARACTER_SIZE,
        )?;
        Ok(Self { payload })
    }

    pub fn get_id(&self) -> Result<&str, AfbError> {
        array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen)
    }

    pub fn get_data(&self) -> Result<&str,AfbError> {
        array_to_str(&self.payload.CONTENT.characters, self.payload.CONTENT.charactersLen)
    }

    pub fn decode(payload: cglue::iso2_EMAIDType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_EMAIDType {
        self.payload
    }

}

