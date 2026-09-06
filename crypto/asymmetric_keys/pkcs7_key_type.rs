//! Automatically rewritten from C to Rust
//! Source: crypto/asymmetric_keys/pkcs7_key_type.c
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
// Testing module to load key from trusted PKCS#7 message
//
// Copyright (C) 2014 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("PKCS#7 testing key type");
    MODULE_AUTHOR("Red Hat, Inc.");
    static unsigned pkcs7_usage;
    module_param_named(usage, pkcs7_usage, uint, S_IWUSR | S_IRUGO);
    MODULE_PARM_DESC(pkcs7_usage,
    "Usage to specify when verifying the PKCS#7 message");
//
// Retrieve the PKCS#7 message content.
//
    static int pkcs7_view_content(void *ctx, const void *data, size_t len,
    size_t asn1hdrlen)
    {
    struct key_preparsed_payload *prep = ctx;
    const void *saved_prep_data;
    size_t saved_prep_datalen;
    int ret;
    saved_prep_data = prep.data;
    saved_prep_datalen = prep.datalen;
    prep.data = data;
    prep.datalen = len;
    ret = user_preparse(prep);
    prep.data = saved_prep_data;
    prep.datalen = saved_prep_datalen;
    return ret;
    }
//
// Preparse a PKCS#7 wrapped and validated data blob.
//
#[no_mangle]
unsafe extern "C" fn pkcs7_preparse(prep: *mut key_preparsed_payload) -> c_int {
    static int pkcs7_preparse(struct key_preparsed_payload *prep)
    {
    let mut usage: enum key_being_used_for = pkcs7_usage;
    if (usage >= NR__KEY_BEING_USED_FOR) {
    pr_err("Invalid usage type %d\n", usage);
    return -EINVAL;
    }
    return verify_pkcs7_signature(core::ptr::null_mut(), 0,
    prep.data, prep.datalen,
    VERIFY_USE_SECONDARY_KEYRING, usage,
    pkcs7_view_content, prep);
    }
//
// user defined keys take an arbitrary string as the description and an
// arbitrary blob of data as the payload
//
    static struct key_type key_type_pkcs7 = {
    .name			= "pkcs7_test",
    .preparse		= pkcs7_preparse,
    .free_preparse		= user_free_preparse,
    .instantiate		= generic_key_instantiate,
    .revoke			= user_revoke,
    .destroy		= user_destroy,
    .describe		= user_describe,
    .read			= user_read,
    };
//
// Module stuff
//
#[no_mangle]
unsafe extern "C" fn pkcs7_key_init() -> int __init {
    static int __init pkcs7_key_init(void)
    {
    return register_key_type(&key_type_pkcs7);
    }
#[no_mangle]
unsafe extern "C" fn pkcs7_key_cleanup() -> void __exit {
    static void __exit pkcs7_key_cleanup(void)
    {
    unregister_key_type(&key_type_pkcs7);
    }
    module_init(pkcs7_key_init);
    module_exit(pkcs7_key_cleanup);
