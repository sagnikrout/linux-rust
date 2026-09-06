//! Automatically rewritten from C to Rust
//! Source: crypto/asymmetric_keys/signature.c
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
// Signature verification with an asymmetric key
//
// See Documentation/crypto/asymmetric-keys.rst
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Destroy a public key signature.
//
#[no_mangle]
pub unsafe extern "C" fn public_key_signature_free(sig: *mut public_key_signature) {
    void public_key_signature_free(struct public_key_signature *sig)
    {
    int i;
    if (sig) {
    for (i = 0; i < ARRAY_SIZE(sig.auth_ids); i++)
    kfree(sig.auth_ids[i]);
    kfree(sig.s);
    if (sig.m_free)
    kfree(sig.m);
    kfree(sig);
    }
    }
    EXPORT_SYMBOL_GPL(public_key_signature_free);
//
// query_asymmetric_key - Get information about an asymmetric key.
// @params: Various parameters.
// @info: Where to put the information.
//
    int query_asymmetric_key(const struct kernel_pkey_params *params,
    struct kernel_pkey_query *info)
    {
    const struct asymmetric_key_subtype *subtype;
    struct key *key = params.key;
    int ret;
    pr_devel("==>%s()\n", __func__);
    if (key.type != &key_type_asymmetric)
    return -EINVAL;
    subtype = asymmetric_key_subtype(key);
    if (!subtype ||
    !key.payload.data[0])
    return -EINVAL;
    if (!subtype.query)
    return -ENOTSUPP;
    ret = subtype.query(params, info);
    pr_devel("<==%s() = %d\n", __func__, ret);
    return ret;
    }
    EXPORT_SYMBOL_GPL(query_asymmetric_key);
//
// verify_signature - Initiate the use of an asymmetric key to verify a signature
// @key: The asymmetric key to verify against
// @sig: The signature to check
//
// Returns 0 if successful or else an error.
//
    int verify_signature(const struct key *key,
    const struct public_key_signature *sig)
    {
    const struct asymmetric_key_subtype *subtype;
    int ret;
    pr_devel("==>%s()\n", __func__);
    if (key.type != &key_type_asymmetric)
    return -EINVAL;
    subtype = asymmetric_key_subtype(key);
    if (!subtype ||
    !key.payload.data[0])
    return -EINVAL;
    if (!subtype.verify_signature)
    return -ENOTSUPP;
    ret = subtype.verify_signature(key, sig);
    pr_devel("<==%s() = %d\n", __func__, ret);
    return ret;
    }
    EXPORT_SYMBOL_GPL(verify_signature);
