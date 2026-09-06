//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/module_signature.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Module signature handling.
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

// In stripped ARM and x86-64 modules, ~ is surprisingly rare.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum module_signature_type {
    MODULE_SIGNATURE_TYPE_PKCS7 = 2,	/* Signature in PKCS#7 message */
}

//
// Module signature information block.
//
// The constituents of the signature section are, in order:
//
// - Signer's name
// - Key identifier
// - Signature data
// - Information block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_signature {
    pub /: *mut *mut __u8 algo; / Public-key crypto algorithm [0],
    pub /: *mut *mut __u8 hash; / Digest algorithm [0],
    pub /: *mut *mut __u8 id_type; / Key identifier type [enum module_signature_type],
    pub /: *mut *mut __u8 signer_len; / Length of signer's name [0],
    pub /: *mut *mut __u8 key_id_len; / Length of key identifier [0],
    pub __pad: [__u8; 3],
    pub /: *mut *mut __be32 sig_len; / Length of signature data,
}
