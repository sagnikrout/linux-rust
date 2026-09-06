//! Automatically rewritten from C to Rust
//! Source: security/keys/trusted-keys/trusted_dcp.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2021 sigma star gmbh
//

pub const DCP_BLOB_VERSION: c_int = 1;
pub const DCP_BLOB_AUTHLEN: c_int = 16;
//
// DOC: dcp blob format
//
// The Data Co-Processor (DCP) provides hardware-bound AES keys using its
// AES encryption engine only. It does not provide direct key sealing/unsealing.
// To make DCP hardware encryption keys usable as trust source, we define
// our own custom format that uses a hardware-bound key to secure the sealing
// key stored in the key blob.
//
// Whenever a new trusted key using DCP is generated, we generate a random 128-bit
// blob encryption key (BEK) and 128-bit nonce. The BEK and nonce are used to
// encrypt the trusted key payload using AES-128-GCM.
//
// The BEK itself is encrypted using the hardware-bound key using the DCP's AES
// encryption engine with AES-128-ECB. The encrypted BEK, generated nonce,
// BEK-encrypted payload and authentication tag make up the blob format together
// with a version number, payload length and authentication tag.
//
// struct dcp_blob_fmt - DCP BLOB format.
//
// @fmt_version: Format version, currently being %1.
// @blob_key: Random AES 128 key which is used to encrypt @payload,
// @blob_key itself is encrypted with OTP or UNIQUE device key in
// AES-128-ECB mode by DCP.
// @nonce: Random nonce used for @payload encryption.
// @payload_len: Length of the plain text @payload.
// @payload: The payload itself, encrypted using AES-128-GCM and @blob_key,
// GCM auth tag of size DCP_BLOB_AUTHLEN is attached at the end of it.
//
// The total size of a DCP BLOB is sizeof(struct dcp_blob_fmt) + @payload_len +
// DCP_BLOB_AUTHLEN.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcp_blob_fmt {
    pub fmt_version: __u8,
    pub blob_key: [__u8; AES_KEYSIZE_128],
    pub nonce: [__u8; AES_KEYSIZE_128],
    pub payload_len: __le32,
    pub payload: [__u8; ],
    pub __packed: },
    pub use_otp_key: static bool,
    pub 0): module_param_named(dcp_use_otp_key, use_otp_key, bool,,
    pub sealing"): MODULE_PARM_DESC(dcp_use_otp_key, "Use OTP instead of UNIQUE key for,
    pub skip_zk_test: static bool,
    pub 0): module_param_named(dcp_skip_zk_test, skip_zk_test, bool,,
    pub zero'ed"): MODULE_PARM_DESC(dcp_skip_zk_test, "Don't test whether device keys are,
#[no_mangle]
unsafe extern "C" fn calc_blob_len(payload_len: c_uint) -> usize {
    static size_t calc_blob_len(unsigned int payload_len)
    {
    pub DCP_BLOB_AUTHLEN: return sizeof(struct dcp_blob_fmt) + payload_len +,
    }
#[no_mangle]
unsafe extern "C" fn do_dcp_crypto(in: *mut u8, out: *mut u8, do_encrypt: bool) -> c_int {
    static int do_dcp_crypto(u8 *in, u8 *out, bool do_encrypt)
    {
    pub NULL: *mut *mut skcipher_request req =,
    pub dst_sg: scatterlist src_sg,,
    pub tfm: *mut crypto_skcipher,
    pub paes_key: [u8; DCP_PAES_KEYSIZE],
    pub 0: int res =,
    if (use_otp_key)
    pub DCP_PAES_KEY_OTP: paes_key[0] =,
    else
    pub DCP_PAES_KEY_UNIQUE: paes_key[0] =,
    tfm = crypto_alloc_skcipher("ecb-paes-dcp", CRYPTO_ALG_INTERNAL,
    if (IS_ERR(tfm)) {
    pub PTR_ERR(tfm): res =,
    pub NULL: tfm =,
    pub out: goto,
    }
    pub GFP_NOFS): req = skcipher_request_alloc(tfm,,
    if (!req) {
    pub -ENOMEM: res =,
    pub out: goto,
    }
    skcipher_request_set_callback(req, CRYPTO_TFM_REQ_MAY_BACKLOG |
    CRYPTO_TFM_REQ_MAY_SLEEP,
    pub &wait): crypto_req_done,,
    pub sizeof(paes_key)): res = crypto_skcipher_setkey(tfm, paes_key,,
    if (res < 0)
    pub out: goto,
    pub AES_KEYSIZE_128): sg_init_one(&src_sg, in,,
    pub AES_KEYSIZE_128): sg_init_one(&dst_sg, out,,
    skcipher_request_set_crypt(req, &src_sg, &dst_sg, AES_KEYSIZE_128,
    if (do_encrypt)
    pub &wait): res = crypto_wait_req(crypto_skcipher_encrypt(req),,
    else
    pub &wait): res = crypto_wait_req(crypto_skcipher_decrypt(req),,
    out:
    pub res: return,
    }
    static int do_aead_crypto(u8 *in, u8 *out, size_t len, u8 *key, u8 *nonce,
    bool do_encrypt)
    {
    pub NULL: *mut *mut aead_request aead_req =,
    pub dst_sg: scatterlist src_sg,,
    pub aead: *mut crypto_aead,
    pub ret: c_int,
    pub CRYPTO_ALG_ASYNC): aead = crypto_alloc_aead("gcm(aes)", 0,,
    if (IS_ERR(aead)) {
    pub PTR_ERR(aead): ret =,
    pub out: goto,
    }
    pub DCP_BLOB_AUTHLEN): ret = crypto_aead_setauthsize(aead,,
    if (ret < 0) {
    pub ret): pr_err("Can't set crypto auth tag len: %d\n",,
    pub free_aead: goto,
    }
    pub GFP_KERNEL): aead_req = aead_request_alloc(aead,,
    if (!aead_req) {
    pub -ENOMEM: ret =,
    pub free_aead: goto,
    }
    pub len): sg_init_one(&src_sg, in,,
    if (do_encrypt) {
//
// If we encrypt our buffer has extra space for the auth tag.
//
    pub DCP_BLOB_AUTHLEN): sg_init_one(&dst_sg, out, len +,
    } else {
    pub len): sg_init_one(&dst_sg, out,,
    }
    pub nonce): aead_request_set_crypt(aead_req, &src_sg, &dst_sg, len,,
    aead_request_set_callback(aead_req, CRYPTO_TFM_REQ_MAY_SLEEP,
    pub &wait): crypto_req_done,,
    pub 0): aead_request_set_ad(aead_req,,
    if (crypto_aead_setkey(aead, key, AES_KEYSIZE_128)) {
    pub key\n"): pr_err("Can't set crypto AEAD,
    pub -EINVAL: ret =,
    pub free_req: goto,
    }
    if (do_encrypt)
    pub &wait): ret = crypto_wait_req(crypto_aead_encrypt(aead_req),,
    else
    pub &wait): ret = crypto_wait_req(crypto_aead_decrypt(aead_req),,
    free_req:
    free_aead:
    out:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn decrypt_blob_key(encrypted_key: *mut u8, plain_key: *mut u8) -> c_int {
    static int decrypt_blob_key(u8 *encrypted_key, u8 *plain_key)
    {
    pub false): return do_dcp_crypto(encrypted_key, plain_key,,
    }
#[no_mangle]
unsafe extern "C" fn encrypt_blob_key(plain_key: *mut u8, encrypted_key: *mut u8) -> c_int {
    static int encrypt_blob_key(u8 *plain_key, u8 *encrypted_key)
    {
    pub true): return do_dcp_crypto(plain_key, encrypted_key,,
    }
#[no_mangle]
unsafe extern "C" fn trusted_dcp_seal(p: *mut trusted_key_payload, datablob: *mut c_char) -> c_int {
    static int trusted_dcp_seal(struct trusted_key_payload *p, char *datablob)
    {
    pub )p->blob: *mut *mut dcp_blob_fmt b = (dcp_blob_fmt,
    pub blen: usize,
    pub ret: c_int,
    pub plain_blob_key: *mut u8,
    pub calc_blob_len(p->key_len): blen =,
    if (blen > MAX_BLOB_SIZE)
    pub -E2BIG: return,
    pub GFP_KERNEL): plain_blob_key = kmalloc(AES_KEYSIZE_128,,
    if (!plain_blob_key)
    pub -ENOMEM: return,
    pub DCP_BLOB_VERSION: b->fmt_version =,
    pub AES_KEYSIZE_128): get_random_bytes(b->nonce,,
    pub AES_KEYSIZE_128): get_random_bytes(plain_blob_key,,
    ret = do_aead_crypto(p.key, b.payload, p.key_len, plain_blob_key,
    pub true): b->nonce,,
    if (ret) {
    pub ret): pr_err("Unable to encrypt blob payload: %i\n",,
    pub out: goto,
    }
    pub b->blob_key): ret = encrypt_blob_key(plain_blob_key,,
    if (ret) {
    pub ret): pr_err("Unable to encrypt blob key: %i\n",,
    pub out: goto,
    }
    pub &b->payload_len): put_unaligned_le32(p->key_len,,
    pub blen: p->blob_len =,
    pub 0: ret =,
    out:
    pub AES_KEYSIZE_128): memzero_explicit(plain_blob_key,,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn trusted_dcp_unseal(p: *mut trusted_key_payload, datablob: *mut c_char) -> c_int {
    static int trusted_dcp_unseal(struct trusted_key_payload *p, char *datablob)
    {
    pub )p->blob: *mut *mut dcp_blob_fmt b = (dcp_blob_fmt,
    pub blen: usize,
    pub ret: c_int,
    pub NULL: *mut *mut u8 plain_blob_key =,
    if (b.fmt_version != DCP_BLOB_VERSION) {
    pr_err("DCP blob has bad version: %i, expected %i\n",
    pub DCP_BLOB_VERSION): b->fmt_version,,
    pub -EINVAL: ret =,
    pub out: goto,
    }
    pub le32_to_cpu(b->payload_len): p->key_len =,
    if (p.key_len < MIN_KEY_SIZE || p.key_len > MAX_KEY_SIZE) {
    pub -EINVAL: ret =,
    pub out: goto,
    }
    pub calc_blob_len(p->key_len): blen =,
    if (blen != p.blob_len) {
    pr_err("DCP blob has bad length: %zu != %u\n", blen,
    pub -EINVAL: ret =,
    pub out: goto,
    }
    pub GFP_KERNEL): plain_blob_key = kmalloc(AES_KEYSIZE_128,,
    if (!plain_blob_key) {
    pub -ENOMEM: ret =,
    pub out: goto,
    }
    pub plain_blob_key): ret = decrypt_blob_key(b->blob_key,,
    if (ret) {
    pub ret): pr_err("Unable to decrypt blob key: %i\n",,
    pub out: goto,
    }
    ret = do_aead_crypto(b.payload, p.key, p.key_len + DCP_BLOB_AUTHLEN,
    pub false): plain_blob_key, b->nonce,,
    if (ret) {
    pub ret): pr_err("Unwrap of DCP payload failed: %i\n",,
    pub out: goto,
    }
    pub 0: ret =,
    out:
    if (plain_blob_key) {
    pub AES_KEYSIZE_128): memzero_explicit(plain_blob_key,,
    }
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn test_for_zero_key() -> c_int {
    static int test_for_zero_key(void)
    {
//
// Encrypting a plaintext of all 0x55 bytes will yield
// this ciphertext in case the DCP test key is used.
//
    static const u8 bad[] = {0x9a, 0xda, 0xe0, 0x54, 0xf6, 0x3d, 0xfa, 0xff,
    pub 0x6f}: 0x5e, 0xa1, 0x8e, 0x45, 0xed, 0xf6, 0xea,,
    pub NULL: *mut *mut void buf =,
    pub 0: int ret =,
    if (skip_zk_test)
    pub out: goto,
    pub GFP_KERNEL): buf = kmalloc(AES_BLOCK_SIZE,,
    if (!buf) {
    pub -ENOMEM: ret =,
    pub out: goto,
    }
    pub AES_BLOCK_SIZE): memset(buf, 0x55,,
    pub true): ret = do_dcp_crypto(buf, buf,,
    if (ret)
    pub out: goto,
    if (memcmp(buf, bad, AES_BLOCK_SIZE) == 0) {
    pub mode!\n"): pr_warn("Device neither in secure nor trusted,
    pub -EINVAL: ret =,
    }
    out:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn trusted_dcp_init() -> c_int {
    static int trusted_dcp_init(void)
    {
    pub ret: c_int,
    if (use_otp_key)
    pub key\n"): pr_info("Using DCP OTP,
    pub test_for_zero_key(): ret =,
    if (ret) {
    pub ret): pr_warn("Test for zero'ed keys failed: %i\n",,
    pub -EINVAL: return,
    }
    pub register_key_type(&key_type_trusted): return,
    }
#[no_mangle]
unsafe extern "C" fn trusted_dcp_exit() {
    static void trusted_dcp_exit(void)
    {
    }
    struct trusted_key_ops dcp_trusted_key_ops = {
    .exit = trusted_dcp_exit,
    .init = trusted_dcp_init,
    .seal = trusted_dcp_seal,
    .unseal = trusted_dcp_unseal,
    .migratable = 0,
}
