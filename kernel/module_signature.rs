//! Automatically rewritten from C to Rust
//! Source: kernel/module_signature.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Module signature checker
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// mod_check_sig - check that the given signature is sane
//
// @ms:		Signature to check.
// @file_len:	Size of the file to which @ms is appended.
// @name:	What is being checked. Used for error messages.
//
    int mod_check_sig(const struct module_signature *ms, size_t file_len,
    const char *name)
    {
    if (be32_to_cpu(ms.sig_len) >= file_len - sizeof(*ms))
    return -EBADMSG;
    if (ms.id_type != MODULE_SIGNATURE_TYPE_PKCS7) {
    pr_err("%s: not signed with expected PKCS#7 message\n",
    name);
    return -ENOPKG;
    }
    if (ms.algo != 0 ||
    ms.hash != 0 ||
    ms.signer_len != 0 ||
    ms.key_id_len != 0 ||
    ms.__pad[0] != 0 ||
    ms.__pad[1] != 0 ||
    ms.__pad[2] != 0) {
    pr_err("%s: PKCS#7 signature info has unexpected non-zero params\n",
    name);
    return -EBADMSG;
    }
    return 0;
    }
