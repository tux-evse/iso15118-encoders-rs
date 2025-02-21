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
 */

use crate::prelude::*;

pub trait PkiSignature {
    fn pki_sign_check(
        &self,
        tagid: iso2_exi::MessageTagId,
        challenge: &[u8],
        pub_key: &PkiPubKey,
    ) -> Result<(), AfbError>;

    fn pki_sign_sign(
        &mut self,
        tagid: iso2_exi::MessageTagId,
        priv_key: &PkiPrivKey,
    ) -> Result<(), AfbError>;
}

pub struct PkiConfig {
    pki: GnuPkiConfig,
}

impl PkiConfig {
    pub fn new(ca_trust: Option<&str>, ca_format: &str) -> Result<&'static Self, AfbError> {
        let format = GnuPkiCertFormat::from_label(ca_format)?;
        let pki = GnuPkiConfig::new(ca_trust, format)?;

        let handle = Box::new(PkiConfig { pki });
        Ok(Box::leak(handle))
    }

    pub fn set_cert_key(
        &self,
        cert_path: &str,
        key_path: &str,
        ca_format: &str,
        key_pin: Option<&str>,
    ) -> Result<&Self, AfbError> {
        let format = GnuPkiCertFormat::from_label(ca_format)?;
        self.pki
            .set_cert_key(cert_path, key_path, format, key_pin)?;
        Ok(self)
    }

    pub fn get_private_key(&self) -> Result<PkiPrivKey, AfbError> {
        self.pki.get_private_key(0)
    }

    #[track_caller]
    pub fn get_public_key(&self) -> Result<PkiPubKey, AfbError> {
        let certs = self.pki.get_cert(0)?;
        let public = certs.get_public_key()?;
        Ok(public)
    }

    ///
    /// Get the contract chain as JSON from:
    /// - the contract cert (the has to be added before through e.g. set_cert_key)
    /// - MO Sub CA1 and MO Sub CA 2 certificates
    ///
    /// Certificates are looked for in the trust list (ca_trust argument in ::new())
    ///
    /// The returned JSON follows this structure:
    /// {
    ///   "emaid": "${emaid}",
    ///   "chain": {
    ///     "cert": "${contract}",
    ///     "sub_certs": [
    ///       "${mo_sub1}",
    ///       "${mo_sub2}"
    ///   ]
    /// }
    pub fn get_contract_chain_as_json(&self) -> Result<JsoncObj, AfbError> {
        // look for the contract certificate in the trust list
        let contract_cert: GnuPkiCerts = self.pki.get_cert(0)?;
        let contract_emaid = contract_cert.get_cn();
        let contract_issuer = contract_cert.get_issuer_cn();

        let mut sub_ca1_cert = None;
        for cert in self.pki.get_trusted_ca() {
            if cert.get_cn() == contract_issuer {
                sub_ca1_cert = Some(cert);
                break;
            }
        }
        if sub_ca1_cert.is_none() {
            return afb_error!(
                "get-contract-chain-as-json",
                "Cannot find a certificate issued by {} in the trust store",
                contract_issuer
            );
        }

        let sub_ca1_cert = sub_ca1_cert.unwrap();

        let sub_ca1_issuer = sub_ca1_cert.get_issuer_cn();
        let mut sub_ca2_cert = None;
        for cert in self.pki.get_trusted_ca() {
            if cert.get_cn() == sub_ca1_issuer {
                sub_ca2_cert = Some(cert);
                break;
            }
        }
        if sub_ca2_cert.is_none() {
            return afb_error!(
                "get-contract-chain-as-json",
                "Cannot find a certificate issued by {} in the trust store",
                sub_ca1_issuer
            );
        }

        let contract_cert = contract_cert.export(GnuPkiCertFormat::DER)?.b64encode()?;

        let sub1 = sub_ca1_cert.export(GnuPkiCertFormat::DER)?.b64encode()?;
        let sub2 = sub_ca2_cert
            .unwrap()
            .export(GnuPkiCertFormat::DER)?
            .b64encode()?;

        let sub_certs = JsoncObj::array();
        sub_certs.append(&sub1.to_string()?)?;
        sub_certs.append(&sub2.to_string()?)?;
        let chain_json = JsoncObj::new();
        chain_json.add("cert", &contract_cert.to_string()?)?;
        chain_json.add("sub_certs", sub_certs)?;

        let json = JsoncObj::new();
        json.add("emaid", &contract_emaid)?;
        json.add("chain", chain_json)?;

        Ok(json)
    }

    #[track_caller]
    pub fn check_cert(&self, cert_list: &mut GnuPkiCerts) -> Result<PkiPubKey, AfbError> {
        self.pki.check_cert(cert_list, GnuPkiVerifFlag::DEFAULT)
    }

    #[track_caller]
    pub fn from_jsonc(jtls: JsoncObj) -> Result<&'static Self, AfbError> {
        let cert_format = jtls.default("format", "pem")?; // iso15118-2 x.509v3 DER format
        let ca_trust = jtls.optional::<&str>("ca_trust")?;

        if let Some(value) = ca_trust {
            if value.len() == 0 {
                return afb_error!("pki-config-from-jsonc", "ca_trust when define should > 0");
            }
        }
        let pki = PkiConfig::new(ca_trust, cert_format)?;

        let cert_path = jtls.optional::<&str>("certs")?;
        if let Some(path) = cert_path {
            let priv_key = jtls.get::<&str>("key")?;
            let priv_pin = jtls.optional::<&str>("pin")?;
            pki.set_cert_key(path, priv_key, cert_format, priv_pin)?;
        }
        Ok(pki)
    }
}
