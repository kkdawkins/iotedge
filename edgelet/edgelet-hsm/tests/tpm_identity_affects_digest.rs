// Copyright (c) Microsoft. All rights reserved.

extern crate base64;
extern crate edgelet_core;
extern crate edgelet_hsm;

use std::str;

use edgelet_core::KeyStore;
use edgelet_core::crypto::Sign;
use edgelet_core::crypto::Signature;
use edgelet_core::crypto::SignatureAlgorithm;
use edgelet_hsm::TpmKeyStore;

const TEST_KEY_BASE64: &'static str = "D7PuplFy7vIr0349blOugqCxyfMscyVZDoV9Ii0EFnA=";

// This tests the following:
//  1) A well known identity key K can be installed in the TPM
//  2) The HMACSHA256 digest sign request for a known payload DATA
//     should return a digest whose value would be diferent based on the identity.
#[test]
fn tpm_identity_affects_digest() {
    // arrange
    let key_store = TpmKeyStore::default();

    let decoded_key = base64::decode(TEST_KEY_BASE64).unwrap();
    let decoded_key_str = unsafe { str::from_utf8_unchecked(&decoded_key) };
    let module1_identity: &str = "module1";
    let module2_identity: &str = "module2";

    key_store.activate_key(decoded_key_str).unwrap();

    let key1 = key_store.get(module1_identity, "ignored").unwrap();
    let key2 = key_store.get(module2_identity, "ignored").unwrap();

    let data_to_be_signed = b"I am the very model of a modern major general";

    // act
    let digest1 = key1.sign(SignatureAlgorithm::HMACSHA256, data_to_be_signed)
        .unwrap();
    let digest2 = key2.sign(SignatureAlgorithm::HMACSHA256, data_to_be_signed)
        .unwrap();

    // assert
    assert_ne!(digest1.as_bytes(), digest2.as_bytes());
}