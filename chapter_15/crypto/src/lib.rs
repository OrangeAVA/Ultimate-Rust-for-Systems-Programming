use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use cbc::Encryptor;
use wasm_bindgen::prelude::*;

type Aes128CbcEnc = Encryptor<aes::Aes128>;

#[wasm_bindgen]
pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let iv = [0u8; 16];
    let mut buf = Vec::from(data);
    let cipher = Aes128CbcEnc::new(key.into(), (&iv).into());
    cipher.encrypt_padded_vec_mut::<Pkcs7>(&mut buf)
}
