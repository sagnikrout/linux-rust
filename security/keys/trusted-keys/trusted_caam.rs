//! Automatically rewritten from C to Rust
//! Source: security/keys/trusted-keys/trusted_caam.c
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
// Copyright (C) 2021 Pengutronix, Ahmad Fatoum <kernel@pengutronix.de>
// Copyright 2025 NXP
//

    static struct caam_blob_priv *blobifier;

    static_assert(MAX_KEY_SIZE + CAAM_BLOB_OVERHEAD <= CAAM_BLOB_MAX_LEN);
    static_assert(MAX_BLOB_SIZE <= CAAM_BLOB_MAX_LEN);
    enum {
    opt_err,
    opt_key_enc_algo,
    };
    static const match_table_t key_tokens = {
    {opt_key_enc_algo, "key_enc_algo=%s"},
    {opt_err, core::ptr::null_mut()}
    };

#[no_mangle]
pub unsafe extern "C" fn dump_options(pkey_info: *const caam_pkey_info) {
    static inline void dump_options(const struct caam_pkey_info *pkey_info)
    {
    if (!trusted_debug)
    return;
    pr_debug("key encryption algo %d\n", pkey_info.key_enc_algo);
    }

#[no_mangle]
pub unsafe extern "C" fn dump_options(pkey_info: *const caam_pkey_info) {
    static inline void dump_options(const struct caam_pkey_info *pkey_info)
    {
    }

    static int get_pkey_options(char *c,
    struct caam_pkey_info *pkey_info)
    {
    substring_t args[MAX_OPT_ARGS];
    let mut token_mask: c_ulong = 0;
    u16 key_enc_algo;
    char *p = c;
    int token;
    int res;
    if (!c)
    return 0;
    while ((p = strsep(&c, " \t"))) {
    if (*p == '\0' || *p == ' ' || *p == '\t')
    continue;
    token = match_token(p, key_tokens, args);
    if (test_and_set_bit(token, &token_mask))
    return -EINVAL;
    switch (token) {
    case opt_key_enc_algo:
    res = kstrtou16(args[0].from, 16, &key_enc_algo);
    if (res < 0)
    return -EINVAL;
    pkey_info.key_enc_algo = key_enc_algo;
    break;
    default:
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_key_pkey(datablob: *mut c_char) -> bool {
    static bool is_key_pkey(char **datablob)
    {
    char *c = core::ptr::null_mut();
    do {
// Second argument onwards,
// determine if tied to HW
//
    c = strsep(datablob, " \t");
    if (c && (strcmp(c, "pk") == 0))
    return true;
    } while (c);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn trusted_caam_seal(p: *mut trusted_key_payload, datablob: *mut c_char) -> c_int {
    static int trusted_caam_seal(struct trusted_key_payload *p, char *datablob)
    {
    int ret;
    struct caam_blob_info info = {
    .input  = p.key,  .input_len   = p.key_len,
    .output = p.blob, .output_len  = MAX_BLOB_SIZE,
    .key_mod = KEYMOD, .key_mod_len = sizeof(KEYMOD) - 1,
    };
//
// If it is to be treated as protected key,
// read next arguments too.
//
    if (is_key_pkey(&datablob)) {
    info.pkey_info.plain_key_sz = p.key_len;
    info.pkey_info.is_pkey = 1;
    ret = get_pkey_options(datablob, &info.pkey_info);
    if (ret < 0)
    return 0;
    dump_options(&info.pkey_info);
    }
    ret = caam_encap_blob(blobifier, &info);
    if (ret)
    return ret;
    p.blob_len = info.output_len;
    if (info.pkey_info.is_pkey) {
    p.key_len = p.blob_len + sizeof(struct caam_pkey_info);
    memcpy(p.key, &info.pkey_info, sizeof(struct caam_pkey_info));
    memcpy(p.key + sizeof(struct caam_pkey_info), p.blob, p.blob_len);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trusted_caam_unseal(p: *mut trusted_key_payload, datablob: *mut c_char) -> c_int {
    static int trusted_caam_unseal(struct trusted_key_payload *p, char *datablob)
    {
    int ret;
    struct caam_blob_info info = {
    .input   = p.blob,  .input_len  = p.blob_len,
    .output  = p.key,   .output_len = MAX_KEY_SIZE,
    .key_mod = KEYMOD,  .key_mod_len = sizeof(KEYMOD) - 1,
    };
    if (is_key_pkey(&datablob)) {
    info.pkey_info.plain_key_sz = p.blob_len - CAAM_BLOB_OVERHEAD;
    info.pkey_info.is_pkey = 1;
    ret = get_pkey_options(datablob, &info.pkey_info);
    if (ret < 0)
    return 0;
    dump_options(&info.pkey_info);
    p.key_len = p.blob_len + sizeof(struct caam_pkey_info);
    memcpy(p.key, &info.pkey_info, sizeof(struct caam_pkey_info));
    memcpy(p.key + sizeof(struct caam_pkey_info), p.blob, p.blob_len);
    return 0;
    }
    ret = caam_decap_blob(blobifier, &info);
    if (ret)
    return ret;
    p.key_len = info.output_len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trusted_caam_init() -> c_int {
    static int trusted_caam_init(void)
    {
    int ret;
    blobifier = caam_blob_gen_init();
    if (IS_ERR(blobifier))
    return PTR_ERR(blobifier);
    ret = register_key_type(&key_type_trusted);
    if (ret)
    caam_blob_gen_exit(blobifier);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn trusted_caam_exit() {
    static void trusted_caam_exit(void)
    {
    unregister_key_type(&key_type_trusted);
    caam_blob_gen_exit(blobifier);
    }
    struct trusted_key_ops trusted_key_caam_ops = {
    .migratable = 0, /* non-migratable */
    .init = trusted_caam_init,
    .seal = trusted_caam_seal,
    .unseal = trusted_caam_unseal,
    .exit = trusted_caam_exit,
    };
