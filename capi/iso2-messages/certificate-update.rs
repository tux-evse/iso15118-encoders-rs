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

pub struct CertificateUpdateRequest {
    payload: cglue::iso2_CertificateUpdateReqType,
}

impl CertificateUpdateRequest {
    pub fn new(id: &str, contract_chain: CertificateChainType,  emaid: &str, root_certs: CertificateRootList) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_CertificateUpdateReqType>() };

        payload.Id.charactersLen = str_to_array(
            id,
            &mut payload.Id.characters,
            cglue::iso2_Id_CHARACTER_SIZE,
        )?;

        payload.eMAID.charactersLen = str_to_array(
            emaid,
            &mut payload.eMAID.characters,
            cglue::iso2_eMAID_CHARACTER_SIZE,
        )?;

        payload.ContractSignatureCertChain = contract_chain.encode();
        payload.ListOfRootCertificateIDs = root_certs.encode();

        Ok(Self { payload })
    }

    pub fn decode(payload: cglue::iso2_CertificateUpdateReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.CertificateUpdateReq = self.payload;
            exi_body.set_CertificateUpdateReq_isUsed(1);
            exi_body
        };
        body
    }

    pub fn get_id(&self) -> Result<&str, AfbError> {
        array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen)
    }

    pub fn get_contract_chain(&self) -> CertificateChainType{
        CertificateChainType::decode(self.payload.ContractSignatureCertChain)
    }

    pub fn get_emaid(&self) -> Result<&str, AfbError> {
        array_to_str(&self.payload.eMAID.characters, self.payload.eMAID.charactersLen)
    }

    pub fn get_root_certs(&self) -> CertificateRootList {
        CertificateRootList::decode(self.payload.ListOfRootCertificateIDs)
    }
}


pub struct CertificateUpdateResponse {
    payload: cglue::iso2_CertificateUpdateResType,
}

impl CertificateUpdateResponse {
    pub fn new(
        code: ResponseCode,
        provisioning_chain: CertificateChainType,
        contract_chain: CertificateChainType,
        private_key: PrivateKeyType,
        public_key: DhPublicKeyType,
        emaid: EmaidType,
    ) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_CertificateUpdateResType>() };
        payload.ResponseCode = code as u32;
        payload.SAProvisioningCertificateChain = provisioning_chain.encode();
        payload.ContractSignatureCertChain = contract_chain.encode();
        payload.ContractSignatureEncryptedPrivateKey = private_key.encode();
        payload.DHpublickey= public_key.encode();
        payload.eMAID= emaid.encode();
        Self { payload }
    }

    pub fn decode(payload: cglue::iso2_CertificateUpdateResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.CertificateUpdateRes = self.payload;
            exi_body.set_CertificateUpdateRes_isUsed(1);
            exi_body
        };
        body
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_provisioning_chain(&self) -> CertificateChainType{
        CertificateChainType::decode(self.payload.SAProvisioningCertificateChain)
    }

    pub fn get_contract_chain(&self) -> CertificateChainType{
        CertificateChainType::decode(self.payload.ContractSignatureCertChain)
    }

    pub fn get_private_key(&self) -> PrivateKeyType {
        PrivateKeyType::decode(self.payload.ContractSignatureEncryptedPrivateKey)
    }

    pub fn get_public_key(&self) -> DhPublicKeyType {
        DhPublicKeyType::decode(self.payload.DHpublickey)
    }

    pub fn get_emaid(&self) -> EmaidType {
        EmaidType::decode(self.payload.eMAID)
    }

}
