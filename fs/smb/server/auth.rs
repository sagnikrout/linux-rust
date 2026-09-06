//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/auth.h
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
// Copyright (C) 2018 Samsung Electronics Co., Ltd.
//

pub const AUTH_GSS_LENGTH: c_int = 96;
pub const AUTH_GSS_PADDING: c_int = 0;

pub const AUTH_GSS_LENGTH: c_int = 74;
pub const AUTH_GSS_PADDING: c_int = 6;

//
// Size of the ntlm client response
//
pub const CIFS_AUTH_RESP_SIZE: c_int = 24;
pub const CIFS_SMB1_SIGNATURE_SIZE: c_int = 8;
pub const CIFS_SMB1_SESSKEY_SIZE: c_int = 16;
pub const KSMBD_AUTH_NTLMSSP: c_uint = 0x0001;
pub const KSMBD_AUTH_KRB5: c_uint = 0x0002;
pub const KSMBD_AUTH_MSKRB5: c_uint = 0x0004;
pub const KSMBD_AUTH_KRB5U2U: c_uint = 0x0008;
extern "C" {
    pub fn ksmbd_copy_gss_neg_header(buf: *mut c_void);
}
