use core::mem;
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
 * reference:
 *  - https://github.com/SwitchEV/RISE-V2G/blob/master/RISE-V2G-Certificates/signature-creation-testdata/README.md
 *  - iso15118-2[V2G2-275] table:64 "Semantics and type definition for CertificateChainType" (p108)
 *  - iso15118-2[Figure 103] SECC Communication states for AC V2G messaging
 *  - iso15118-2[V2G2-108] The EMAID shall be encoded in the subject of the certificate
 *  - iso15118-2[Table 13] Overview of applied XML based signatures
 *
 */
use ::std::os::raw;
use afbv4::utilv4::*;
use cglue::gnutls_privkey_init;
use std::ffi::CStr;
use std::ffi::CString;
use std::slice;
use std::str;

pub mod cglue {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!("_pki-capi.rs");
}

// fn load_buffer(filename: &str) -> Result<Vec<u8>, AfbError> {
//     let file = match File::open(filename) {
//         Ok(handle) => handle,
//         Err(error) => return afb_error!("pki-open-file", "fail open error:{}", error),
//     };
//     let mut buffer = Vec::new();
//     if let Err(error) = file.read_to_end(&mut buffer) {
//         return afb_error!("pki-read-file", "fail read error:{}", error);
//     }
//     Ok(buffer)
// }

fn gtls_perror(code: i32) -> String {
    let error = unsafe { cglue::gnutls_strerror(code) };
    let cstring = unsafe { CStr::from_ptr(error as *const raw::c_char) };
    let slice: &str = cstring.to_str().unwrap();
    slice.to_owned()
}

fn gtls_free(data: *mut raw::c_void) {
    unsafe {
        if let Some(func) = cglue::gnutls_free {
            (func)(data)
        }
    }
}

pub struct GnuPkiDatum {
    payload: cglue::gnutls_datum_t,
    gtls_owned: bool,
}

impl GnuPkiDatum {
    pub fn new(buffer: &[u8]) -> Self {
        let payload = cglue::gnutls_datum_t {
            data: buffer.as_ptr() as *mut u8,
            size: buffer.len() as u32,
        };
        Self {
            payload,
            gtls_owned: false,
        }
    }

    pub fn to_string(&self) -> Result<String, AfbError> {
        let slice = unsafe { slice::from_raw_parts(self.payload.data, self.payload.size as usize) };
        let text = match str::from_utf8(slice) {
            Ok(value) => value,
            Err(_) => return afb_error!("pki-datum-to_string", "fail converting datum"),
        };
        Ok(text.to_string())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let slice = unsafe { slice::from_raw_parts(self.payload.data, self.payload.size as usize) };
        slice.to_vec()
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.payload.data, self.payload.size as usize) }
    }

    pub fn b64decode(&self) -> Result<Self, AfbError> {
        let payload = unsafe {
            let mut buffer = mem::MaybeUninit::<cglue::gnutls_datum_t>::uninit();
            let status = cglue::gnutls_base64_decode2(&self.payload, buffer.as_mut_ptr());
            if status < 0 {
                return afb_error!("pki-datum-b64decode", "error:{}", gtls_perror(status));
            }
            buffer.assume_init()
        };

        Ok(Self {
            payload,
            gtls_owned: true,
        })
    }

    pub fn b64encode(&self) -> Result<Self, AfbError> {
        let payload = unsafe {
            let mut buffer = mem::MaybeUninit::<cglue::gnutls_datum_t>::uninit();
            let status = cglue::gnutls_base64_encode2(&self.payload, buffer.as_mut_ptr());
            if status < 0 {
                return afb_error!("pki-datum-b64encode", "error:{}", gtls_perror(status));
            }
            buffer.assume_init()
        };

        Ok(Self {
            payload,
            gtls_owned: true,
        })
    }

    pub fn get_payload(&self) -> cglue::gnutls_datum_t {
        self.payload
    }
}

impl Drop for GnuPkiDatum {
    fn drop(&mut self) {
        if self.gtls_owned {
            unsafe {
                if let Some(callback) = cglue::gnutls_free {
                    (callback)(self.payload.data as *mut raw::c_void)
                }
            }
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum GnuPkiCertFormat {
    DER = cglue::C_GNUTLS_X509_FMT_DER,
    PEM = cglue::C_GNUTLS_X509_FMT_PEM,
}

impl GnuPkiCertFormat {
    pub fn from_label(label: &str) -> Result<GnuPkiCertFormat, AfbError> {
        let format = match label.to_lowercase().as_str() {
            "der" => GnuPkiCertFormat::DER,
            "pem" => GnuPkiCertFormat::PEM,
            _ => return afb_error!("glu-pki-format", "invalid certificat format:{}", label),
        };
        Ok(format)
    }
}

#[repr(u32)]
pub enum GnuPkiKeyId {
    SHA1 = cglue::gnutls_keyid_flags_t_GNUTLS_KEYID_USE_SHA1,
    SHA256 = cglue::gnutls_keyid_flags_t_GNUTLS_KEYID_USE_SHA256,
    SHA512 = cglue::gnutls_keyid_flags_t_GNUTLS_KEYID_USE_SHA512,
    BEST = cglue::gnutls_keyid_flags_t_GNUTLS_KEYID_USE_BEST_KNOWN,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GnuPkiVerifFlag {
    // Fulup: incomplete list
    DISABLE_CA_SIGN = cglue::gnutls_certificate_verify_flags_GNUTLS_VERIFY_DISABLE_CA_SIGN,
    DO_NOT_ALLOW_SAME = cglue::gnutls_certificate_verify_flags_GNUTLS_VERIFY_DO_NOT_ALLOW_SAME,
    ALLOW_SIGN_WITH_SHA1 =
        cglue::gnutls_certificate_verify_flags_GNUTLS_VERIFY_ALLOW_SIGN_WITH_SHA1,
    DISABLE_CRL_CHECKS = cglue::gnutls_certificate_verify_flags_GNUTLS_VERIFY_DISABLE_CRL_CHECKS,
    DISABLE_TIME_CHECKS = cglue::gnutls_certificate_verify_flags_GNUTLS_VERIFY_DISABLE_TIME_CHECKS,
    DEFAULT = 0,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GnuPkcs11Flag {
    // Fulup: incomplete list
    PUBKEY = cglue::gnutls_pkcs11_obj_flags_GNUTLS_PKCS11_OBJ_FLAG_PUBKEY,
    CRT = cglue::gnutls_pkcs11_obj_flags_GNUTLS_PKCS11_OBJ_FLAG_CRT,
    DISTRUST = cglue::gnutls_pkcs11_obj_flags_GNUTLS_PKCS11_OBJ_FLAG_RETRIEVE_DISTRUSTED,
}

pub struct PkiPubKey {
    payload: cglue::gnutls_pubkey_t,
}

impl PkiPubKey {
    pub fn get_payload(&self) -> cglue::gnutls_pubkey_t {
        self.payload
    }
}

impl Drop for PkiPubKey {
    fn drop(&mut self) {
        unsafe {
            cglue::gnutls_pubkey_deinit(self.payload);
        }
    }
}

pub struct PkiPrivKey {
    payload: cglue::gnutls_privkey_t,
}

impl PkiPrivKey {
    pub fn get_payload(&self) -> cglue::gnutls_privkey_t {
        self.payload
    }
}

impl Drop for PkiPrivKey {
    fn drop(&mut self) {
        unsafe {
            cglue::gnutls_privkey_deinit(self.payload);
        }
    }
}
pub struct GnuPkiCerts {
    payload: cglue::gnutls_x509_crt_t,
    count: u32,
}

impl GnuPkiCerts {
    pub fn new() -> Result<Self, AfbError> {
        let mut cert = mem::MaybeUninit::<cglue::gnutls_x509_crt_t>::uninit();
        let payload = unsafe {
            let status = cglue::gnutls_x509_crt_init(cert.as_mut_ptr());
            if status < 0 {
                return afb_error!(
                    "pki-cert-from-raw",
                    "Fail to build certificate error:{}",
                    gtls_perror(status)
                );
            }
            cert.assume_init()
        };
        Ok(Self { payload, count: 0 })
    }

    pub fn add_raw(
        &mut self,
        cert_data: &[u8],
        cert_format: GnuPkiCertFormat,
    ) -> Result<&mut Self, AfbError> {
        let cert_datum = cglue::gnutls_datum_t {
            data: cert_data.as_ptr() as *mut u8,
            size: cert_data.len() as u32,
        };

        unsafe {
            let status =
                cglue::gnutls_x509_crt_import(self.payload, &cert_datum, cert_format as u32);
            if status < 0 {
                return afb_error!(
                    "pki-cert-add-raw",
                    "Fail to import certificate error:{}",
                    gtls_perror(status)
                );
            }
        };
        self.count += 1;
        Ok(self)
    }

    pub fn add_datum(
        &mut self,
        cert_datum: &GnuPkiDatum,
        cert_format: GnuPkiCertFormat,
    ) -> Result<&mut Self, AfbError> {
        unsafe {
            let status = cglue::gnutls_x509_crt_import(
                self.payload,
                &cert_datum.get_payload(),
                cert_format as u32,
            );
            if status < 0 {
                return afb_error!(
                    "pki-cert-add_datum-raw",
                    "Fail to import certificate error:{}",
                    gtls_perror(status)
                );
            }
        };
        self.count += 1;
        Ok(self)
    }

    pub fn get_dn(&self) -> String {
        let mut buffer = [0 as raw::c_char; 255];
        let mut len = buffer.len();
        let dn = unsafe {
            cglue::gnutls_x509_crt_get_dn(self.payload, buffer.as_mut_ptr(), &mut len);
            let slice = slice::from_raw_parts(&buffer as *const _ as *const u8, len);
            str::from_utf8(slice).unwrap().to_string()
        };
        dn
    }

    pub fn get_cn(&self) -> String {
        let mut buffer = [0 as raw::c_char; 32];
        let mut len = buffer.len();
        let dn = unsafe {
            cglue::gnutls_x509_crt_get_dn_by_oid(
                self.payload,
                cglue::GNUTLS_OID_X520_COMMON_NAME as *const _ as *const raw::c_char,
                0,
                0,
                buffer.as_mut_ptr() as *mut raw::c_void,
                &mut len,
            );
            let slice = slice::from_raw_parts(&buffer as *const _ as *const u8, len);
            str::from_utf8(slice).unwrap().to_string()
        };
        dn
    }

    pub fn get_issuer_cn(&self) -> String {
        let mut buffer = [0 as raw::c_char; 32];
        let mut len = buffer.len();
        let dn = unsafe {
            cglue::gnutls_x509_crt_get_issuer_dn_by_oid(
                self.payload,
                cglue::GNUTLS_OID_X520_COMMON_NAME as *const _ as *const raw::c_char,
                0,
                0,
                buffer.as_mut_ptr() as *mut raw::c_void,
                &mut len,
            );
            let slice = slice::from_raw_parts(&buffer as *const _ as *const u8, len);
            str::from_utf8(slice).unwrap().to_string()
        };
        dn
    }

    pub fn get_public_key(&self) -> Result<PkiPubKey, AfbError> {
        let mut pub_key = mem::MaybeUninit::<cglue::gnutls_pubkey_t>::uninit();
        let status = unsafe { cglue::gnutls_pubkey_init(pub_key.as_mut_ptr()) };
        if status < 0 {
            return afb_error!(
                "gpki-cert-key",
                "fail to initialize public key error:{}",
                gtls_perror(status)
            );
        }
        let pub_key = unsafe { pub_key.assume_init() };
        let status = unsafe { cglue::gnutls_pubkey_import_x509(pub_key, self.payload, 0) };
        if status < 0 {
            return afb_error!(
                "gpki-cert-key",
                "fail to initialize public key error:{}",
                gtls_perror(status)
            );
        }
        Ok(PkiPubKey { payload: pub_key })
    }

    pub fn get_payload(&self) -> cglue::gnutls_x509_crt_t {
        self.payload
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn get_key_id(&self, flag: GnuPkiKeyId) -> Result<Vec<u8>, AfbError> {
        let mut buffer = [0u8; 32];
        let mut len = buffer.len();

        let status = unsafe {
            cglue::gnutls_x509_crt_get_key_id(
                self.payload,
                flag as u32,
                buffer.as_mut_ptr(),
                &mut len,
            )
        };
        if status < 0 {
            return afb_error!(
                "gpki-cert-key",
                "fail to extract keyid from certificate error:{}",
                gtls_perror(status)
            );
        }
        Ok(buffer[0..len].to_vec())
    }

    pub fn export(&self, format: GnuPkiCertFormat) -> Result<GnuPkiDatum, AfbError> {
        let mut buffer = mem::MaybeUninit::<cglue::gnutls_datum_t>::uninit();
        unsafe {
            let status =
                cglue::gnutls_x509_crt_export2(self.payload, format as u32, buffer.as_mut_ptr());
            if status < 0 {
                return afb_error!("gpki-cert-export", "Cannot export: {}", gtls_perror(status));
            }

            Ok(GnuPkiDatum {
                payload: buffer.assume_init(),
                gtls_owned: true,
            })
        }
    }
}

impl Drop for GnuPkiCerts {
    fn drop(&mut self) {
        unsafe {
            cglue::gnutls_x509_crt_deinit(self.payload);
        }
    }
}

pub struct GnuPkiTrustList {
    payload: cglue::gnutls_x509_trust_list_t,
    iter: cglue::gnutls_x509_trust_list_iter_t,
    // whether the struct must be explicitely dropped
    to_deinit: bool,
}

impl GnuPkiTrustList {
    pub fn get_payload(&self) -> cglue::gnutls_x509_trust_list_t {
        self.payload
    }
}

impl Drop for GnuPkiTrustList {
    fn drop(&mut self) {
        if self.to_deinit {
            unsafe {
                cglue::gnutls_x509_trust_list_deinit(self.payload, 1);
            }
        }
    }
}

//
// Implement the Iterator trait, so that we can write "for x in trust_list"
//
impl Iterator for GnuPkiTrustList {
    type Item = GnuPkiCerts;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cert = mem::MaybeUninit::<cglue::gnutls_x509_crt_t>::uninit();
        unsafe {
            match cglue::gnutls_x509_trust_list_iter_get_ca(
                self.payload,
                &mut self.iter,
                cert.as_mut_ptr(),
            ) {
                cglue::C_GNUTLS_E_REQUESTED_DATA_NOT_AVAILABLE => None,
                _ => Some(GnuPkiCerts {
                    payload: cert.assume_init(),
                    count: 1,
                }),
            }
        }
    }
}

#[derive(Clone)]
pub struct GnuPkiConfig {
    payload: cglue::gnutls_certificate_credentials_t,
}
impl GnuPkiConfig {
    pub fn new(ca_trust: Option<&str>, ca_format: GnuPkiCertFormat) -> Result<Self, AfbError> {
        const GNU_TLS_MIN_VER: &str = "3.4.6";

        match CString::new(GNU_TLS_MIN_VER) {
            Ok(value) => value,
            Err(_) => {
                return afb_error!(
                    "gpki-credentials-new",
                    "fail to import iface:{}",
                    GNU_TLS_MIN_VER
                )
            }
        };

        let payload = unsafe {
            let mut cred = mem::MaybeUninit::<cglue::gnutls_certificate_credentials_t>::uninit();
            let status = cglue::gnutls_certificate_allocate_credentials(cred.as_mut_ptr());
            let cred = cred.assume_init();
            if status < 0 {
                return afb_error!(
                    "gpki-credentials-new",
                    "fail to initialize session error:{}",
                    gtls_perror(status)
                );
            }
            cred
        };

        if let Some(ca) = ca_trust {
            let ca_cstr = match CString::new(ca) {
                Ok(str) => str,
                Err(err) => {
                    return afb_error!("gpki-credentials-new", "CString::new failure: {}", err);
                }
            };

            unsafe {
                let status = cglue::gnutls_certificate_set_x509_trust_dir(
                    payload,
                    ca_cstr.as_ptr() as *mut raw::c_char,
                    ca_format as u32,
                );

                if status < 0 {
                    return afb_error!(
                        "gpki-credentials-new",
                        "invalid pki key/certification ca_trust:{} error:{}",
                        ca,
                        gtls_perror(status)
                    );
                }
            }
        }

        Ok(Self { payload })
    }

    pub fn set_public_cert(
        &self,
        cert_path: &str,
        ca_format: GnuPkiCertFormat,
    ) -> Result<&Self, AfbError> {
        let glutls_cert = match CString::new(cert_path) {
            Ok(value) => value,
            Err(_) => {
                return afb_error!("gpki-cert-file", "fail to import pki_certs:{}", cert_path)
            }
        };

        let status = unsafe {
            cglue::gnutls_certificate_set_x509_trust_file(
                self.payload,
                glutls_cert.as_ptr(),
                ca_format as u32,
            )
        };
        if status < 0 {
            return afb_error!(
                "gpki-credentials-set-public",
                "invalid pki certificate:{} error:{}",
                cert_path,
                gtls_perror(status)
            );
        }

        Ok(self)
    }

    pub fn add_trusted_cert_raw(
        &self,
        cert_data: &[u8],
        ca_format: GnuPkiCertFormat,
    ) -> Result<&Self, AfbError> {
        let cert_datum = cglue::gnutls_datum_t {
            data: cert_data.as_ptr() as *mut u8,
            size: cert_data.len() as u32,
        };

        // prepare for gnutls_certificate_verify_peers2()
        let status = unsafe {
            cglue::gnutls_certificate_set_x509_trust_mem(
                self.payload,
                &cert_datum,
                ca_format as u32,
            )
        };
        if status < 0 {
            return afb_error!(
                "gpki-credentials-set-cert",
                "invalid pki certificate:&[u8] error:{}",
                gtls_perror(status)
            );
        }

        Ok(self)
    }

    pub fn set_cert_key(
        &self,
        cert_path: &str,
        key_path: &str,
        ca_format: GnuPkiCertFormat,
        key_pin: Option<&str>,
    ) -> Result<&Self, AfbError> {
        let glutls_key = match CString::new(key_path) {
            Ok(value) => value,
            Err(_) => {
                return afb_error!(
                    "gpki-credentials-set-keys",
                    "fail to import key:{}",
                    key_path
                )
            }
        };

        let glutls_pin = match key_pin {
            None => None,
            Some(pin) => match CString::new(pin) {
                Ok(value) => Some(value),
                Err(_) => {
                    return afb_error!(
                        "gpki-credentials-set-keys",
                        "fail to import tls_pin:{}",
                        pin
                    )
                }
            },
        };

        let glutls_cert = match CString::new(cert_path) {
            Ok(value) => value,
            Err(_) => {
                return afb_error!(
                    "gpki-credentials-set-keys",
                    "fail to import tls_certs:{}",
                    cert_path
                )
            }
        };

        let status = unsafe {
            match glutls_pin {
                None => cglue::gnutls_certificate_set_x509_key_file(
                    self.payload,
                    glutls_cert.as_ptr(),
                    glutls_key.as_ptr(),
                    ca_format as u32,
                ),
                Some(pin) => cglue::gnutls_certificate_set_x509_key_file2(
                    self.payload,
                    glutls_cert.as_ptr(),
                    glutls_key.as_ptr(),
                    ca_format as u32,
                    pin.as_ptr(),
                    cglue::gnutls_pkcs_encrypt_flags_t_GNUTLS_PKCS_PLAIN, // unencrypted key
                ),
            }
        };

        if status < 0 {
            return afb_error!(
                "gpki-credentials-set-keys",
                "invalid pki key:{} cert:{} error:{}",
                key_path,
                cert_path,
                gtls_perror(status)
            );
        }

        Ok(self)
    }

    pub fn get_private_key(&self, index: u32) -> Result<PkiPrivKey, AfbError> {
        let payload = unsafe {
            let mut key_x509 = mem::MaybeUninit::<cglue::gnutls_x509_privkey_t>::uninit();
            let status =
                cglue::gnutls_certificate_get_x509_key(self.payload, index, key_x509.as_mut_ptr());
            if status < 0 {
                return afb_error!(
                    "gpki-credentials-get-private",
                    "fail to retrieve private x509 key from credential store index:{}, error:{}",
                    index,
                    gtls_perror(status)
                );
            }
            let key_x509 = key_x509.assume_init();

            let mut key_private = mem::MaybeUninit::<cglue::gnutls_privkey_t>::uninit();
            let status = gnutls_privkey_init(key_private.as_mut_ptr());
            if status < 0 {
                return afb_error!(
                    "gpki-credentials-get-private",
                    "fail to allocate private key error:{}",
                    gtls_perror(status)
                );
            }
            let key_private = key_private.assume_init();
            let status = cglue::gnutls_privkey_import_x509(
                key_private,
                key_x509,
                cglue::C_GNUTLS_PRIVKEY_IMPORT_AUTO_RELEASE as u32,
            );
            if status < 0 {
                return afb_error!(
                    "gpki-credentials-get-private",
                    "file to import x509 key as generic private key error:{}",
                    gtls_perror(status)
                );
            }

            key_private
        };

        Ok(PkiPrivKey { payload })
    }

    pub fn get_cert(&self, index: u32) -> Result<GnuPkiCerts, AfbError> {
        let list = unsafe {
            let mut buffer = mem::MaybeUninit::<*mut cglue::gnutls_x509_crt_t>::uninit();
            let count = 0;
            let status = cglue::gnutls_certificate_get_x509_crt(
                self.payload,
                index,
                buffer.as_mut_ptr(),
                &count as *const _ as *mut u32,
            );
            if status < 0 {
                return afb_error!(
                    "gpki-credentials-get-trusted",
                    "failed to retrieve cert from config index:{}, error:{}",
                    index,
                    gtls_perror(status)
                );
            }
            GnuPkiCerts {
                // only return the first certificate here
                payload: *buffer.assume_init(),
                count,
            }
        };
        Ok(list)
    }

    pub fn get_trusted_ca(&self) -> GnuPkiTrustList {
        let mut buffer = mem::MaybeUninit::<cglue::gnutls_x509_trust_list_t>::uninit();

        unsafe {
            cglue::gnutls_x509_trust_list_init(buffer.as_mut_ptr(), 0);
            cglue::gnutls_certificate_get_trust_list(self.payload, buffer.as_mut_ptr());
            GnuPkiTrustList {
                payload: buffer.assume_init(),
                iter: core::ptr::null_mut(),
                // internal trust_list, it must not be manually freed
                to_deinit: false,
            }
        }
    }

    pub fn check_cert(
        &self,
        cert_list: &mut GnuPkiCerts,
        flags: GnuPkiVerifFlag,
    ) -> Result<PkiPubKey, AfbError> {
        let trust_list = self.get_trusted_ca();
        let mut voutput = 0u32;

        let status = unsafe {
            cglue::gnutls_x509_trust_list_verify_crt(
                trust_list.get_payload(),
                &mut cert_list.get_payload(),
                cert_list.get_count(),
                flags as u32,
                &mut voutput,
                None,
            )
        };

        if status != 0 {
            let mut datum = mem::MaybeUninit::<cglue::gnutls_datum_t>::uninit();
            let error = unsafe {
                cglue::gnutls_certificate_verification_status_print(
                    voutput,
                    cglue::C_GNUTLS_CRT_X509,
                    datum.as_mut_ptr(),
                    0,
                );
                let datum = datum.assume_init();
                let slice = slice::from_raw_parts(datum.data, datum.size as usize);
                let text = str::from_utf8(slice).unwrap();
                let error = format!("status:{} error:{}", text, gtls_perror(status));
                gtls_free(datum.data as *mut raw::c_void);
                error
            };
            return afb_error!("gpki-credentials-check-cert", error);
        }

        cert_list.get_public_key()
    }
}
