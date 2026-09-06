//! Automatically rewritten from C to Rust
//! Source: crypto/rsa_helper.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// RSA key extract helper
//
// Copyright (c) 2015, Intel Corporation
// Authors: Tadeusz Struk <tadeusz.struk@intel.com>
//

    int rsa_get_n(void *context, size_t hdrlen, unsigned char tag,
    const void *value, size_t vlen)
    {
    struct rsa_key *key = context;
    const u8 *ptr = value;
    let mut n_sz: usize = vlen;
// invalid key provided
    if (!value || !vlen)
    return -EINVAL;
    if (fips_enabled) {
    while (n_sz && !*ptr) {
    ptr++;
    n_sz--;
    }
// In FIPS mode only allow key size 2K and higher
    if (n_sz < 256) {
    pr_err("RSA: key size not allowed in FIPS mode\n");
    return -EINVAL;
    }
    }
    key.n = value;
    key.n_sz = vlen;
    return 0;
    }
    int rsa_get_e(void *context, size_t hdrlen, unsigned char tag,
    const void *value, size_t vlen)
    {
    struct rsa_key *key = context;
// invalid key provided
    if (!value || !key.n_sz || !vlen || vlen > key.n_sz)
    return -EINVAL;
    key.e = value;
    key.e_sz = vlen;
    return 0;
    }
    int rsa_get_d(void *context, size_t hdrlen, unsigned char tag,
    const void *value, size_t vlen)
    {
    struct rsa_key *key = context;
// invalid key provided
    if (!value || !key.n_sz || !vlen || vlen > key.n_sz)
    return -EINVAL;
    key.d = value;
    key.d_sz = vlen;
    return 0;
    }
    int rsa_get_p(void *context, size_t hdrlen, unsigned char tag,
    const void *value, size_t vlen)
    {
    struct rsa_key *key = context;
// invalid key provided
    if (!value || !vlen || vlen > key.n_sz)
    return -EINVAL;
    key.p = value;
    key.p_sz = vlen;
    return 0;
    }
    int rsa_get_q(void *context, size_t hdrlen, unsigned char tag,
    const void *value, size_t vlen)
    {
    struct rsa_key *key = context;
// invalid key provided
    if (!value || !vlen || vlen > key.n_sz)
    return -EINVAL;
    key.q = value;
    key.q_sz = vlen;
    return 0;
    }
    int rsa_get_dp(void *context, size_t hdrlen, unsigned char tag,
    const void *value, size_t vlen)
    {
    struct rsa_key *key = context;
// invalid key provided
    if (!value || !vlen || vlen > key.n_sz)
    return -EINVAL;
    key.dp = value;
    key.dp_sz = vlen;
    return 0;
    }
    int rsa_get_dq(void *context, size_t hdrlen, unsigned char tag,
    const void *value, size_t vlen)
    {
    struct rsa_key *key = context;
// invalid key provided
    if (!value || !vlen || vlen > key.n_sz)
    return -EINVAL;
    key.dq = value;
    key.dq_sz = vlen;
    return 0;
    }
    int rsa_get_qinv(void *context, size_t hdrlen, unsigned char tag,
    const void *value, size_t vlen)
    {
    struct rsa_key *key = context;
// invalid key provided
    if (!value || !vlen || vlen > key.n_sz)
    return -EINVAL;
    key.qinv = value;
    key.qinv_sz = vlen;
    return 0;
    }
//
// rsa_parse_pub_key() - decodes the BER encoded buffer and stores in the
// provided struct rsa_key, pointers to the raw key as is,
// so that the caller can copy it or MPI parse it, etc.
//
// @rsa_key:	struct rsa_key key representation
// @key:	key in BER format
// @key_len:	length of key
//
// Return:	0 on success or error code in case of error
//
    int rsa_parse_pub_key(struct rsa_key *rsa_key, const void *key,
    unsigned int key_len)
    {
    return asn1_ber_decoder(&rsapubkey_decoder, rsa_key, key, key_len);
    }
    EXPORT_SYMBOL_GPL(rsa_parse_pub_key);
//
// rsa_parse_priv_key() - decodes the BER encoded buffer and stores in the
// provided struct rsa_key, pointers to the raw key
// as is, so that the caller can copy it or MPI parse it,
// etc.
//
// @rsa_key:	struct rsa_key key representation
// @key:	key in BER format
// @key_len:	length of key
//
// Return:	0 on success or error code in case of error
//
    int rsa_parse_priv_key(struct rsa_key *rsa_key, const void *key,
    unsigned int key_len)
    {
    return asn1_ber_decoder(&rsaprivkey_decoder, rsa_key, key, key_len);
    }
    EXPORT_SYMBOL_GPL(rsa_parse_priv_key);
