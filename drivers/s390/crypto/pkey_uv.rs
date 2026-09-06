//! Automatically rewritten from C to Rust
//! Source: drivers/s390/crypto/pkey_uv.c
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


// SPDX-License-Identifier: GPL-2.0
//
// pkey uv specific code
//
// Copyright IBM Corp. 2024
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("IBM Corporation");
    MODULE_DESCRIPTION("s390 protected key UV handler");
//
// One pre-allocated uv_secret_list for use with uv_find_secret()
//
    static struct uv_secret_list *uv_list;
    static DEFINE_MUTEX(uv_list_mutex);
//
// UV secret token struct and defines.
//
pub const TOKVER_UV_SECRET: c_uint = 0x09;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvsecrettoken {
    pub /: *mut *mut u8 type; / 0x00 = TOKTYPE_NON_CCA,
    pub res0: [u8; 3],
    pub /: *mut *mut u8 version; / 0x09 = TOKVER_UV_SECRET,
    pub res1: [u8; 3],
    pub /: *mut *mut u16 secret_type; / one of enum uv_secret_types from uv.h,
    pub /: *mut *mut u16 secret_len; / length in bytes of the secret,
    pub /: *mut *mut u8 secret_id[UV_SECRET_ID_LEN]; / the secret id for this secret,
    pub __packed: },
//
// Check key blob for known and supported UV key.
//
#[no_mangle]
unsafe extern "C" fn is_uv_key(key: *const u8, keylen: u32) -> bool {
    static bool is_uv_key(const u8 *key, u32 keylen)
    {
    pub )key: *mut *mut uvsecrettoken t = (uvsecrettoken,
    if (keylen < sizeof(*t))
    pub false: return,
    switch (t.type) {
    case TOKTYPE_NON_CCA:
    switch (t.version) {
    case TOKVER_UV_SECRET:
    switch (t.secret_type) {
    case UV_SECRET_AES_128:
    case UV_SECRET_AES_192:
    case UV_SECRET_AES_256:
    case UV_SECRET_AES_XTS_128:
    case UV_SECRET_AES_XTS_256:
    case UV_SECRET_HMAC_SHA_256:
    case UV_SECRET_HMAC_SHA_512:
    case UV_SECRET_ECDSA_P256:
    case UV_SECRET_ECDSA_P384:
    case UV_SECRET_ECDSA_P521:
    case UV_SECRET_ECDSA_ED25519:
    case UV_SECRET_ECDSA_ED448:
    pub true: return,
    default:
    pub false: return,
    }
    default:
    pub false: return,
    }
    default:
    pub false: return,
    }
    }
#[no_mangle]
unsafe extern "C" fn is_uv_keytype(keytype: enum pkey_key_type) -> bool {
    static bool is_uv_keytype(enum pkey_key_type keytype)
    {
    switch (keytype) {
    case PKEY_TYPE_UVSECRET:
    pub true: return,
    default:
    pub false: return,
    }
    }
    static int get_secret_metadata(const u8 secret_id[UV_SECRET_ID_LEN],
    struct uv_secret_list_item_hdr *secret)
    {
    pub rc: c_int,
    pub sizeof(*uv_list)): *mut memset(uv_list, 0,,
    pub secret): rc = uv_find_secret(secret_id, uv_list,,
    pub rc: return,
    }
    static int retrieve_secret(const u8 secret_id[UV_SECRET_ID_LEN],
    u16 *secret_type, u8 *buf, u32 *buflen)
    {
    pub secret_meta_data: uv_secret_list_item_hdr,
    pub rc: c_int,
    pub &secret_meta_data): rc = get_secret_metadata(secret_id,,
    if (rc)
    pub rc: return,
    if (*buflen < secret_meta_data.length)
    pub -EINVAL: return,
    rc = uv_retrieve_secret(secret_meta_data.index,
    pub secret_meta_data.length): buf,,
    if (rc)
    pub rc: return,
// secret_type = secret_meta_data.type;
// buflen = secret_meta_data.length;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn uv_get_size_and_type(secret_type: u16, pkeysize: *mut u32, pkeytype: *mut u32) -> c_int {
    static int uv_get_size_and_type(u16 secret_type, u32 *pkeysize, u32 *pkeytype)
    {
    pub 0: int rc =,
    switch (secret_type) {
    case UV_SECRET_AES_128:
// pkeysize = 16 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_AES_128;
    case UV_SECRET_AES_192:
// pkeysize = 24 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_AES_192;
    case UV_SECRET_AES_256:
// pkeysize = 32 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_AES_256;
    case UV_SECRET_AES_XTS_128:
// pkeysize = 16 + 16 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_AES_XTS_128;
    case UV_SECRET_AES_XTS_256:
// pkeysize = 32 + 32 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_AES_XTS_256;
    case UV_SECRET_HMAC_SHA_256:
// pkeysize = 64 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_HMAC_512;
    case UV_SECRET_HMAC_SHA_512:
// pkeysize = 128 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_HMAC_1024;
    case UV_SECRET_ECDSA_P256:
// pkeysize = 32 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_ECC_P256;
    case UV_SECRET_ECDSA_P384:
// pkeysize = 48 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_ECC_P384;
    case UV_SECRET_ECDSA_P521:
// pkeysize = 80 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_ECC_P521;
    case UV_SECRET_ECDSA_ED25519:
// pkeysize = 32 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_ECC_ED25519;
    case UV_SECRET_ECDSA_ED448:
// pkeysize = 64 + AES_WK_VP_SIZE;
// pkeytype = PKEY_KEYTYPE_ECC_ED448;
    default:
    pub -EINVAL: rc =,
    }
    pub rc: return,
    }
    static int uv_key2protkey(const struct pkey_apqn *_apqns __always_unused,
    size_t _nr_apqns __always_unused,
    const u8 *key, u32 keylen,
    u8 *protkey, u32 *protkeylen, u32 *keyinfo,
    u32 _xflags __always_unused)
    {
    pub )key: *mut *mut uvsecrettoken t = (uvsecrettoken,
    pub pkeytype: u32 pkeysize,,
    pub secret_type: u16,
    pub rc: c_int,
    pub &pkeytype): rc = uv_get_size_and_type(t->secret_type, &pkeysize,,
    if (rc)
    pub out: goto,
    if (*protkeylen < pkeysize) {
    PKEY_DBF_ERR("%s prot key buffer size too small: %u < %u\n",
    pub pkeysize): *mut *mut __func__, protkeylen,,
    pub -EINVAL: rc =,
    pub out: goto,
    }
    pub protkeylen): rc = retrieve_secret(t->secret_id, &secret_type, protkey,,
    if (rc) {
    PKEY_DBF_ERR("%s retrieve_secret() failed with %d\n",
    pub rc): __func__,,
    pub out: goto,
    }
    if (secret_type != t.secret_type) {
    PKEY_DBF_ERR("%s retrieved secret type %u != expected type %u\n",
    pub t->secret_type): __func__, secret_type,,
    pub -EINVAL: rc =,
    pub out: goto,
    }
    if (keyinfo)
// keyinfo = pkeytype;
    out:
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
    static int uv_verifykey(const u8 *key, u32 keylen,
    u16 *_card __always_unused,
    u16 *_dom __always_unused,
    u32 *keytype, u32 *keybitsize, u32 *flags,
    u32 xflags __always_unused)
    {
    pub )key: *mut *mut uvsecrettoken t = (uvsecrettoken,
    pub secret_meta_data: uv_secret_list_item_hdr,
    pub bitsize: u32 pkeysize, pkeytype,,
    pub rc: c_int,
    pub &pkeytype): rc = uv_get_size_and_type(t->secret_type, &pkeysize,,
    if (rc)
    pub out: goto,
    pub &secret_meta_data): rc = get_secret_metadata(t->secret_id,,
    if (rc)
    pub out: goto,
    if (secret_meta_data.type != t.secret_type) {
    pub -EINVAL: rc =,
    pub out: goto,
    }
// set keytype; keybitsize and flags are not supported
    if (keytype)
// keytype = PKEY_TYPE_UVSECRET;
    if (keybitsize) {
    pub pkey_keytype_to_size(pkeytype): *mut *mut bitsize = 8,
// keybitsize = bitsize ?: PKEY_SIZE_UNKNOWN;
    }
    if (flags)
// flags = pkeytype;
    out:
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
    static struct pkey_handler uv_handler = {
    .module			 = THIS_MODULE,
    .name			 = "PKEY UV handler",
    .is_supported_key	 = is_uv_key,
    .is_supported_keytype	 = is_uv_keytype,
    .key_to_protkey		 = uv_key2protkey,
    .verify_key		 = uv_verifykey,
}

//
// Module init
//
#[no_mangle]
unsafe extern "C" fn pkey_uv_init() -> int __init {
    static int __init pkey_uv_init(void)
    {
    int rc;
    if (!is_prot_virt_guest())
    return -ENODEV;
    if (!test_bit_inv(BIT_UVC_CMD_RETR_SECRET, uv_info.inst_calls_list))
    return -ENODEV;
    uv_list = kmalloc_obj(*uv_list);
    if (!uv_list)
    return -ENOMEM;
    rc = pkey_handler_register(&uv_handler);
    if (rc)
    kfree(uv_list);
    return rc;
    }
//
// Module exit
//
#[no_mangle]
unsafe extern "C" fn pkey_uv_exit() -> void __exit {
    static void __exit pkey_uv_exit(void)
    {
    pkey_handler_unregister(&uv_handler);
    mutex_lock(&uv_list_mutex);
    kvfree(uv_list);
    mutex_unlock(&uv_list_mutex);
    }
    module_cpu_feature_match(S390_CPU_FEATURE_UV, pkey_uv_init);
    module_exit(pkey_uv_exit);
