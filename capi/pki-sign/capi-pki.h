/*
 * Copyright 2023 - 2022 Pionix GmbH, IoT.bzh and Contributors to EVerest
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
 * Interface file extracted from: https://github.com/EVerest/libiso15118
 * Generate Rust structures for ISO15118-2/20 messages.
 *
 */

#include <errno.h>
#include <string.h>
#include <unistd.h>

#include <gnutls/abstract.h>
#include <gnutls/gnutls.h>
#include <gnutls/x509.h>
#include <gnutls/crypto.h>
#include <gnutls/pkcs11.h>

const int C_GNUTLS_E_CERTIFICATE_ERROR=GNUTLS_E_CERTIFICATE_ERROR;
const uint C_GNUTLS_VERIFY_ALLOW_X509_V1_CA_CRT= GNUTLS_VERIFY_ALLOW_X509_V1_CA_CRT;
const uint C_GNUTLS_CERT_INVALID=GNUTLS_CERT_INVALID;
const uint C_GNUTLS_CERT_SIGNER_NOT_FOUND=GNUTLS_CERT_SIGNER_NOT_FOUND;
const uint C_GNUTLS_CERT_REVOKED=GNUTLS_CERT_REVOKED;
const uint C_GNUTLS_CERT_EXPIRED=GNUTLS_CERT_EXPIRED;
const uint C_GNUTLS_CERT_NOT_ACTIVATED=GNUTLS_CERT_NOT_ACTIVATED;
const uint C_GNUTLS_CRT_X509=GNUTLS_CRT_X509;
const uint C_GNUTLS_X509_FMT_DER= GNUTLS_X509_FMT_DER;
const uint C_GNUTLS_X509_FMT_PEM= GNUTLS_X509_FMT_PEM;
const int C_GNUTLS_E_REQUESTED_DATA_NOT_AVAILABLE = GNUTLS_E_REQUESTED_DATA_NOT_AVAILABLE;
const int C_GNUTLS_PRIVKEY_IMPORT_AUTO_RELEASE = GNUTLS_PRIVKEY_IMPORT_AUTO_RELEASE;
