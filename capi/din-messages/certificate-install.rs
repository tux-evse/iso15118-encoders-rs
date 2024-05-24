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

pub struct CertificateInstallRequest {
    payload: cglue::din_CertificateInstallationReqType,
}

impl CertificateInstallRequest {
    pub fn new(
        provisioning: &[u8],
        certs_list: &CertificateRootList,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_CertificateInstallationReqType>() };

        payload.OEMProvisioningCert.bytesLen = bytes_to_array(
            provisioning,
            &mut payload.OEMProvisioningCert.bytes,
            cglue::din_certificateType_BYTES_SIZE,
        )?;

        payload.ListOfRootCertificateIDs = certs_list.encode();

        Ok(Self { payload })
    }

    pub fn set_id(&mut self, id: &str) -> Result<&mut Self, AfbError> {
        self.payload.Id.charactersLen = str_to_array(
            id,
            &mut self.payload.Id.characters,
            cglue::din_Id_CHARACTER_SIZE,
        )?;
        self.payload.set_Id_isUsed(1);
        Ok(self)
    }

    pub fn get_id(&self) -> Result<Option<&str>, AfbError> {
        let response= if self.payload.Id_isUsed() == 0 {
            None
        } else {
            Some(array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen)?)
        };
        Ok(response)
    }

    pub fn get_provisioning(&self) -> &[u8] {
        array_to_bytes(
            &self.payload.OEMProvisioningCert.bytes,
            self.payload.OEMProvisioningCert.bytesLen,
        )
    }

    pub fn get_certs_list(&self) -> CertificateRootList {
        CertificateRootList::decode(self.payload.ListOfRootCertificateIDs)
    }

    pub fn decode(payload: cglue::din_CertificateInstallationReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.CertificateInstallationReq = self.payload;
            exi_body.set_CertificateInstallationReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct CertificateInstallResponse {
    payload: cglue::din_CertificateInstallationResType,
}

impl CertificateInstallResponse {
    pub fn new(
        code: ResponseCode,
        id_ref: &str,
        contract_id: &str,
        contract_chain: &CertificateChainType,
        dh_params: &[u8],
        contract_signature: &[u8],
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_CertificateInstallationResType>() };
        payload.ResponseCode = code as u32;
        payload.Id.charactersLen = str_to_array(
            id_ref,
            &mut payload.Id.characters,
            cglue::din_Id_CHARACTER_SIZE,
        )?;
        payload.ContractID.charactersLen = str_to_array(
            contract_id,
            &mut payload.ContractID.characters,
            cglue::din_Id_CHARACTER_SIZE,
        )?;
        payload.ContractID.charactersLen = str_to_array(
            id_ref,
            &mut payload.ContractID.characters,
            cglue::din_Id_CHARACTER_SIZE,
        )?;
        payload.ContractSignatureCertChain = contract_chain.encode();

        payload.DHParams.bytesLen = bytes_to_array(
            dh_params,
            &mut payload.DHParams.bytes,
            cglue::din_Id_CHARACTER_SIZE,
        )?;

        payload.ContractSignatureEncryptedPrivateKey.bytesLen = bytes_to_array(
            contract_signature,
            &mut payload.ContractSignatureEncryptedPrivateKey.bytes,
            cglue::din_Id_CHARACTER_SIZE,
        )?;

        Ok(Self { payload })
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_id(&self) -> Result<&str, AfbError> {
        array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen)
    }

    pub fn get_contract_id(&self) -> Result<&str, AfbError> {
        array_to_str(&self.payload.ContractID.characters, self.payload.ContractID.charactersLen)
    }

    pub fn get_contract_chain(&self) -> CertificateChainType {
        CertificateChainType::decode(self.payload.ContractSignatureCertChain)
    }

    pub fn get_public_key(&self) -> &[u8] {
        array_to_bytes(&self.payload.DHParams.bytes, self.payload.DHParams.bytesLen)
    }

    pub fn get_contract_signature(&self) -> &[u8] {
        array_to_bytes(&self.payload.ContractSignatureEncryptedPrivateKey.bytes, self.payload.ContractSignatureEncryptedPrivateKey.bytesLen)
    }

    pub fn decode(payload: cglue::din_CertificateInstallationResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.CertificateInstallationRes = self.payload;
            exi_body.set_CertificateInstallationRes_isUsed(1);
            exi_body
        };
        body
    }
}
