//! Automatically rewritten from C to Rust
//! Source: security/keys/encrypted-keys/masterkey_trusted.c
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
// Copyright (C) 2010 IBM Corporation
// Copyright (C) 2010 Politecnico di Torino, Italy
// TORSEC group -- https://security.polito.it
//
// Authors:
// Mimi Zohar <zohar@us.ibm.com>
// Roberto Sassu <roberto.sassu@polito.it>
//
// See Documentation/security/keys/trusted-encrypted.rst
//

//
// request_trusted_key - request the trusted key
//
// Trusted keys are sealed to PCRs and other metadata. Although userspace
// manages both trusted/encrypted key-types, like the encrypted key type
// data, trusted key type data is not visible decrypted from userspace.
//
    struct key *request_trusted_key(const char *trusted_desc,
    const u8 **master_key, size_t *master_keylen)
    {
    struct trusted_key_payload *tpayload;
    struct key *tkey;
    tkey = request_key(&key_type_trusted, trusted_desc, core::ptr::null_mut());
    if (IS_ERR(tkey))
    goto error;
    down_read(&tkey.sem);
    tpayload = tkey.payload.data[0];
// master_key = tpayload->key;
// master_keylen = tpayload->key_len;
    error:
    return tkey;
    }
