//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_crypto.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// This header is BSD licensed so anyone can use the definitions to implement
// compatible drivers/servers.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of IBM nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
// USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT
// OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

pub const VIRTIO_CRYPTO_SERVICE_CIPHER: c_int = 0;
pub const VIRTIO_CRYPTO_SERVICE_HASH: c_int = 1;
pub const VIRTIO_CRYPTO_SERVICE_MAC: c_int = 2;
pub const VIRTIO_CRYPTO_SERVICE_AEAD: c_int = 3;
pub const VIRTIO_CRYPTO_SERVICE_AKCIPHER: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_ctrl_header {

    pub opcode: __le32,
    pub algo: __le32,
    pub flag: __le32,
// data virtqueue id
    pub queue_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_cipher_session_para {
pub const VIRTIO_CRYPTO_NO_CIPHER: c_int = 0;
pub const VIRTIO_CRYPTO_CIPHER_ARC4: c_int = 1;
pub const VIRTIO_CRYPTO_CIPHER_AES_ECB: c_int = 2;
pub const VIRTIO_CRYPTO_CIPHER_AES_CBC: c_int = 3;
pub const VIRTIO_CRYPTO_CIPHER_AES_CTR: c_int = 4;
pub const VIRTIO_CRYPTO_CIPHER_DES_ECB: c_int = 5;
pub const VIRTIO_CRYPTO_CIPHER_DES_CBC: c_int = 6;
pub const VIRTIO_CRYPTO_CIPHER_3DES_ECB: c_int = 7;
pub const VIRTIO_CRYPTO_CIPHER_3DES_CBC: c_int = 8;
pub const VIRTIO_CRYPTO_CIPHER_3DES_CTR: c_int = 9;
pub const VIRTIO_CRYPTO_CIPHER_KASUMI_F8: c_int = 10;
pub const VIRTIO_CRYPTO_CIPHER_SNOW3G_UEA2: c_int = 11;
pub const VIRTIO_CRYPTO_CIPHER_AES_F8: c_int = 12;
pub const VIRTIO_CRYPTO_CIPHER_AES_XTS: c_int = 13;
pub const VIRTIO_CRYPTO_CIPHER_ZUC_EEA3: c_int = 14;
    pub algo: __le32,
// length of key
    pub keylen: __le32,
pub const VIRTIO_CRYPTO_OP_ENCRYPT: c_int = 1;
pub const VIRTIO_CRYPTO_OP_DECRYPT: c_int = 2;
// encrypt or decrypt
    pub op: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_session_input {
// Device-writable part
    pub session_id: __le64,
    pub status: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_cipher_session_req {
    pub para: virtio_crypto_cipher_session_para,
    pub padding: [__u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_hash_session_para {
pub const VIRTIO_CRYPTO_NO_HASH: c_int = 0;
pub const VIRTIO_CRYPTO_HASH_MD5: c_int = 1;
pub const VIRTIO_CRYPTO_HASH_SHA1: c_int = 2;
pub const VIRTIO_CRYPTO_HASH_SHA_224: c_int = 3;
pub const VIRTIO_CRYPTO_HASH_SHA_256: c_int = 4;
pub const VIRTIO_CRYPTO_HASH_SHA_384: c_int = 5;
pub const VIRTIO_CRYPTO_HASH_SHA_512: c_int = 6;
pub const VIRTIO_CRYPTO_HASH_SHA3_224: c_int = 7;
pub const VIRTIO_CRYPTO_HASH_SHA3_256: c_int = 8;
pub const VIRTIO_CRYPTO_HASH_SHA3_384: c_int = 9;
pub const VIRTIO_CRYPTO_HASH_SHA3_512: c_int = 10;
pub const VIRTIO_CRYPTO_HASH_SHA3_SHAKE128: c_int = 11;
pub const VIRTIO_CRYPTO_HASH_SHA3_SHAKE256: c_int = 12;
    pub algo: __le32,
// hash result length
    pub hash_result_len: __le32,
    pub padding: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_hash_create_session_req {
    pub para: virtio_crypto_hash_session_para,
    pub padding: [__u8; 40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_mac_session_para {
pub const VIRTIO_CRYPTO_NO_MAC: c_int = 0;
pub const VIRTIO_CRYPTO_MAC_HMAC_MD5: c_int = 1;
pub const VIRTIO_CRYPTO_MAC_HMAC_SHA1: c_int = 2;
pub const VIRTIO_CRYPTO_MAC_HMAC_SHA_224: c_int = 3;
pub const VIRTIO_CRYPTO_MAC_HMAC_SHA_256: c_int = 4;
pub const VIRTIO_CRYPTO_MAC_HMAC_SHA_384: c_int = 5;
pub const VIRTIO_CRYPTO_MAC_HMAC_SHA_512: c_int = 6;
pub const VIRTIO_CRYPTO_MAC_CMAC_3DES: c_int = 25;
pub const VIRTIO_CRYPTO_MAC_CMAC_AES: c_int = 26;
pub const VIRTIO_CRYPTO_MAC_KASUMI_F9: c_int = 27;
pub const VIRTIO_CRYPTO_MAC_SNOW3G_UIA2: c_int = 28;
pub const VIRTIO_CRYPTO_MAC_GMAC_AES: c_int = 41;
pub const VIRTIO_CRYPTO_MAC_GMAC_TWOFISH: c_int = 42;
pub const VIRTIO_CRYPTO_MAC_CBCMAC_AES: c_int = 49;
pub const VIRTIO_CRYPTO_MAC_CBCMAC_KASUMI_F9: c_int = 50;
pub const VIRTIO_CRYPTO_MAC_XCBC_AES: c_int = 53;
    pub algo: __le32,
// hash result length
    pub hash_result_len: __le32,
// length of authenticated key
    pub auth_key_len: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_mac_create_session_req {
    pub para: virtio_crypto_mac_session_para,
    pub padding: [__u8; 40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_aead_session_para {
pub const VIRTIO_CRYPTO_NO_AEAD: c_int = 0;
pub const VIRTIO_CRYPTO_AEAD_GCM: c_int = 1;
pub const VIRTIO_CRYPTO_AEAD_CCM: c_int = 2;
pub const VIRTIO_CRYPTO_AEAD_CHACHA20_POLY1305: c_int = 3;
    pub algo: __le32,
// length of key
    pub key_len: __le32,
// hash result length
    pub hash_result_len: __le32,
// length of the additional authenticated data (AAD) in bytes
    pub aad_len: __le32,
// encrypt or decrypt, See above VIRTIO_CRYPTO_OP_*
    pub op: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_aead_create_session_req {
    pub para: virtio_crypto_aead_session_para,
    pub padding: [__u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_rsa_session_para {
pub const VIRTIO_CRYPTO_RSA_RAW_PADDING: c_int = 0;
pub const VIRTIO_CRYPTO_RSA_PKCS1_PADDING: c_int = 1;
    pub padding_algo: __le32,
pub const VIRTIO_CRYPTO_RSA_NO_HASH: c_int = 0;
pub const VIRTIO_CRYPTO_RSA_MD2: c_int = 1;
pub const VIRTIO_CRYPTO_RSA_MD3: c_int = 2;
pub const VIRTIO_CRYPTO_RSA_MD4: c_int = 3;
pub const VIRTIO_CRYPTO_RSA_MD5: c_int = 4;
pub const VIRTIO_CRYPTO_RSA_SHA1: c_int = 5;
pub const VIRTIO_CRYPTO_RSA_SHA256: c_int = 6;
pub const VIRTIO_CRYPTO_RSA_SHA384: c_int = 7;
pub const VIRTIO_CRYPTO_RSA_SHA512: c_int = 8;
pub const VIRTIO_CRYPTO_RSA_SHA224: c_int = 9;
    pub hash_algo: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_ecdsa_session_para {
pub const VIRTIO_CRYPTO_CURVE_UNKNOWN: c_int = 0;
pub const VIRTIO_CRYPTO_CURVE_NIST_P192: c_int = 1;
pub const VIRTIO_CRYPTO_CURVE_NIST_P224: c_int = 2;
pub const VIRTIO_CRYPTO_CURVE_NIST_P256: c_int = 3;
pub const VIRTIO_CRYPTO_CURVE_NIST_P384: c_int = 4;
pub const VIRTIO_CRYPTO_CURVE_NIST_P521: c_int = 5;
    pub curve_id: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_akcipher_session_para {
pub const VIRTIO_CRYPTO_NO_AKCIPHER: c_int = 0;
pub const VIRTIO_CRYPTO_AKCIPHER_RSA: c_int = 1;
pub const VIRTIO_CRYPTO_AKCIPHER_DSA: c_int = 2;
pub const VIRTIO_CRYPTO_AKCIPHER_ECDSA: c_int = 3;
    pub algo: __le32,
pub const VIRTIO_CRYPTO_AKCIPHER_KEY_TYPE_PUBLIC: c_int = 1;
pub const VIRTIO_CRYPTO_AKCIPHER_KEY_TYPE_PRIVATE: c_int = 2;
    pub keytype: __le32,
    pub keylen: __le32,
    pub rsa: virtio_crypto_rsa_session_para,
    pub ecdsa: virtio_crypto_ecdsa_session_para,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_akcipher_create_session_req {
    pub para: virtio_crypto_akcipher_session_para,
    pub padding: [__u8; 36],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_alg_chain_session_para {
pub const VIRTIO_CRYPTO_SYM_ALG_CHAIN_ORDER_HASH_THEN_CIPHER: c_int = 1;
pub const VIRTIO_CRYPTO_SYM_ALG_CHAIN_ORDER_CIPHER_THEN_HASH: c_int = 2;
    pub alg_chain_order: __le32,
// Plain hash
pub const VIRTIO_CRYPTO_SYM_HASH_MODE_PLAIN: c_int = 1;
// Authenticated hash (mac)
pub const VIRTIO_CRYPTO_SYM_HASH_MODE_AUTH: c_int = 2;
// Nested hash
pub const VIRTIO_CRYPTO_SYM_HASH_MODE_NESTED: c_int = 3;
    pub hash_mode: __le32,
    pub cipher_param: virtio_crypto_cipher_session_para,
    pub hash_param: virtio_crypto_hash_session_para,
    pub mac_param: virtio_crypto_mac_session_para,
    pub padding: [__u8; 16],
    pub u: },
// length of the additional authenticated data (AAD) in bytes
    pub aad_len: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_alg_chain_session_req {
    pub para: virtio_crypto_alg_chain_session_para,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_sym_create_session_req {
    pub cipher: virtio_crypto_cipher_session_req,
    pub chain: virtio_crypto_alg_chain_session_req,
    pub padding: [__u8; 48],
    pub u: },
// Device-readable part
// No operation
pub const VIRTIO_CRYPTO_SYM_OP_NONE: c_int = 0;
// Cipher only operation on the data
pub const VIRTIO_CRYPTO_SYM_OP_CIPHER: c_int = 1;
//
// Chain any cipher with any hash or mac operation. The order
// depends on the value of alg_chain_order param
//
pub const VIRTIO_CRYPTO_SYM_OP_ALGORITHM_CHAINING: c_int = 2;
    pub op_type: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_destroy_session_req {
// Device-readable part
    pub session_id: __le64,
    pub padding: [__u8; 48],
}

// The request of the control virtqueue's packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_op_ctrl_req {
    pub header: virtio_crypto_ctrl_header,
    pub padding: [__u8; 56],
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_op_header {

// akcipher sign/verify opcodes are deprecated

    pub opcode: __le32,
// algo should be service-specific algorithms
    pub algo: __le32,
// session_id should be service-specific algorithms
    pub session_id: __le64,
// control flag to control the request
    pub flag: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_cipher_para {
//
// Byte Length of valid IV/Counter
//
// For block ciphers in CBC or F8 mode, or for Kasumi in F8 mode, or for
// SNOW3G in UEA2 mode, this is the length of the IV (which
// must be the same as the block length of the cipher).
// For block ciphers in CTR mode, this is the length of the counter
// (which must be the same as the block length of the cipher).
// For AES-XTS, this is the 128bit tweak, i, from IEEE Std 1619-2007.
//
// The IV/Counter will be updated after every partial cryptographic
// operation.
//
    pub iv_len: __le32,
// length of source data
    pub src_data_len: __le32,
// length of dst data
    pub dst_data_len: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_hash_para {
// length of source data
    pub src_data_len: __le32,
// hash result length
    pub hash_result_len: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_mac_para {
    pub hash: virtio_crypto_hash_para,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_aead_para {
//
// Byte Length of valid IV data pointed to by the below iv_addr
// parameter.
//
// For GCM mode, this is either 12 (for 96-bit IVs) or 16, in which
// case iv_addr points to J0.
// For CCM mode, this is the length of the nonce, which can be in the
// range 7 to 13 inclusive.
//
    pub iv_len: __le32,
// length of additional auth data
    pub aad_len: __le32,
// length of source data
    pub src_data_len: __le32,
// length of dst data
    pub dst_data_len: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_cipher_data_req {
// Device-readable part
    pub para: virtio_crypto_cipher_para,
    pub padding: [__u8; 24],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_hash_data_req {
// Device-readable part
    pub para: virtio_crypto_hash_para,
    pub padding: [__u8; 40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_mac_data_req {
// Device-readable part
    pub para: virtio_crypto_mac_para,
    pub padding: [__u8; 40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_alg_chain_data_para {
    pub iv_len: __le32,
// Length of source data
    pub src_data_len: __le32,
// Length of destination data
    pub dst_data_len: __le32,
// Starting point for cipher processing in source data
    pub cipher_start_src_offset: __le32,
// Length of the source data that the cipher will be computed on
    pub len_to_cipher: __le32,
// Starting point for hash processing in source data
    pub hash_start_src_offset: __le32,
// Length of the source data that the hash will be computed on
    pub len_to_hash: __le32,
// Length of the additional auth data
    pub aad_len: __le32,
// Length of the hash result
    pub hash_result_len: __le32,
    pub reserved: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_alg_chain_data_req {
// Device-readable part
    pub para: virtio_crypto_alg_chain_data_para,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_sym_data_req {
    pub cipher: virtio_crypto_cipher_data_req,
    pub chain: virtio_crypto_alg_chain_data_req,
    pub padding: [__u8; 40],
    pub u: },
// See above VIRTIO_CRYPTO_SYM_OP_*
    pub op_type: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_aead_data_req {
// Device-readable part
    pub para: virtio_crypto_aead_para,
    pub padding: [__u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_akcipher_para {
    pub src_data_len: __le32,
    pub dst_data_len: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_akcipher_data_req {
    pub para: virtio_crypto_akcipher_para,
    pub padding: [__u8; 40],
}

// The request of the data virtqueue's packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_op_data_req {
    pub header: virtio_crypto_op_header,
    pub sym_req: virtio_crypto_sym_data_req,
    pub hash_req: virtio_crypto_hash_data_req,
    pub mac_req: virtio_crypto_mac_data_req,
    pub aead_req: virtio_crypto_aead_data_req,
    pub akcipher_req: virtio_crypto_akcipher_data_req,
    pub padding: [__u8; 48],
    pub u: },
}

pub const VIRTIO_CRYPTO_OK: c_int = 0;
pub const VIRTIO_CRYPTO_ERR: c_int = 1;
pub const VIRTIO_CRYPTO_BADMSG: c_int = 2;
pub const VIRTIO_CRYPTO_NOTSUPP: c_int = 3;

// The accelerator hardware is ready

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_config {
// See VIRTIO_CRYPTO_OP_* above
    pub status: __le32,
//
// Maximum number of data queue
//
    pub max_dataqueues: __le32,
//
// Specifies the services mask which the device support,
// see VIRTIO_CRYPTO_SERVICE_* above
//
    pub crypto_services: __le32,
// Detailed algorithms mask
    pub cipher_algo_l: __le32,
    pub cipher_algo_h: __le32,
    pub hash_algo: __le32,
    pub mac_algo_l: __le32,
    pub mac_algo_h: __le32,
    pub aead_algo: __le32,
// Maximum length of cipher key
    pub max_cipher_key_len: __le32,
// Maximum length of authenticated key
    pub max_auth_key_len: __le32,
    pub akcipher_algo: __le32,
// Maximum size of each crypto request's content
    pub max_size: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_crypto_inhdr {
// See VIRTIO_CRYPTO_* above
    pub status: __u8,
}
