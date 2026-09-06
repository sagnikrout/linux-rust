//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/keyctl.h
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
// keyctl.h: keyctl command IDs
//
// Copyright (C) 2004, 2008 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// special process keyring shortcut IDs

// request-key default keyrings

pub const KEY_REQKEY_DEFL_DEFAULT: c_int = 0;
pub const KEY_REQKEY_DEFL_THREAD_KEYRING: c_int = 1;
pub const KEY_REQKEY_DEFL_PROCESS_KEYRING: c_int = 2;
pub const KEY_REQKEY_DEFL_SESSION_KEYRING: c_int = 3;
pub const KEY_REQKEY_DEFL_USER_KEYRING: c_int = 4;
pub const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: c_int = 5;
pub const KEY_REQKEY_DEFL_GROUP_KEYRING: c_int = 6;
pub const KEY_REQKEY_DEFL_REQUESTOR_KEYRING: c_int = 7;
// keyctl commands

// keyctl structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyctl_dh_params {

    pub private: __s32,

    pub priv: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyctl_kdf_params {
    pub hashname: *mut char __user,
    pub otherinfo: *mut char __user,
    pub otherinfolen: __u32,
    pub __spare: [__u32; 8],
}

pub const KEYCTL_SUPPORTS_ENCRYPT: c_uint = 0x01;
pub const KEYCTL_SUPPORTS_DECRYPT: c_uint = 0x02;
pub const KEYCTL_SUPPORTS_SIGN: c_uint = 0x04;
pub const KEYCTL_SUPPORTS_VERIFY: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyctl_pkey_query {
    pub /: *mut *mut __u32 supported_ops; / Which ops are supported,
    pub /: *mut *mut __u32 key_size; / Size of the key in bits,
    pub /: *mut *mut __u16 max_data_size; / Maximum size of raw data to sign in bytes,
    pub /: *mut *mut __u16 max_sig_size; / Maximum size of signature in bytes,
    pub /: *mut *mut __u16 max_enc_size; / Maximum size of encrypted blob in bytes,
    pub /: *mut *mut __u16 max_dec_size; / Maximum size of decrypted blob in bytes,
    pub __spare: [__u32; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyctl_pkey_params {
    pub /: *mut *mut __s32 key_id; / Serial no. of public key to use,
    pub /: *mut *mut __u32 in_len; / Input data size,
    pub /: *mut *mut __u32 out_len; / Output buffer size (encrypt/decrypt/sign),
    pub /: *mut *mut __u32 in2_len; / 2nd input data size (verify),
}

pub const KEYCTL_MOVE_EXCL: c_uint = 0x00000001 /* Do not displace from the to-keyring */;
//
// Capabilities flags.  The capabilities list is an array of 8-bit integers;
// each integer can carry up to 8 flags.
//
pub const KEYCTL_CAPS0_CAPABILITIES: c_uint = 0x01 /* KEYCTL_CAPABILITIES supported */;
pub const KEYCTL_CAPS0_PERSISTENT_KEYRINGS: c_uint = 0x02 /* Persistent keyrings enabled */;
pub const KEYCTL_CAPS0_DIFFIE_HELLMAN: c_uint = 0x04 /* Diffie-Hellman computation enabled */;
pub const KEYCTL_CAPS0_PUBLIC_KEY: c_uint = 0x08 /* Public key ops enabled */;
pub const KEYCTL_CAPS0_BIG_KEY: c_uint = 0x10 /* big_key-type enabled */;
pub const KEYCTL_CAPS0_INVALIDATE: c_uint = 0x20 /* KEYCTL_INVALIDATE supported */;
pub const KEYCTL_CAPS0_RESTRICT_KEYRING: c_uint = 0x40 /* KEYCTL_RESTRICT_KEYRING supported */;
pub const KEYCTL_CAPS0_MOVE: c_uint = 0x80 /* KEYCTL_MOVE supported */;
pub const KEYCTL_CAPS1_NS_KEYRING_NAME: c_uint = 0x01 /* Keyring names are per-user_namespace */;
pub const KEYCTL_CAPS1_NS_KEY_TAG: c_uint = 0x02 /* Key indexing can include a namespace tag */;
pub const KEYCTL_CAPS1_NOTIFICATIONS: c_uint = 0x04 /* Keys generate watchable notifications */;
