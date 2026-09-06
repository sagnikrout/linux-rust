//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/gss_krb5.h
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


//
// Adapted from MIT Kerberos 5-1.2.1 lib/include/krb5.h,
// lib/gssapi/krb5/gssapiP_krb5.h, and others
//
// Copyright (c) 2000-2008 The Regents of the University of Michigan.
// All rights reserved.
//
// Andy Adamson   <andros@umich.edu>
// Bruce Fields   <bfields@umich.edu>
//
// Copyright 1995 by the Massachusetts Institute of Technology.
// All Rights Reserved.
//
// Export of this software from the United States of America may
// require a specific license from the United States Government.
// It is the responsibility of any person or organization contemplating
// export to obtain such a license before exporting.
//
// WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
// distribute this software and its documentation for any purpose and
// without fee is hereby granted, provided that the above copyright
// notice appear in all copies and that both that copyright notice and
// this permission notice appear in supporting documentation, and that
// the name of M.I.T. not be used in advertising or publicity pertaining
// to distribution of the software without specific, written prior
// permission.  Furthermore if you modify this software you must label
// your software as modified software and not distribute it in such a
// fashion that it might be confused with the original M.I.T. software.
// M.I.T. makes no representations about the suitability of
// this software for any purpose.  It is provided "as is" without express
// or implied warranty.
//

// Maximum key length (in bytes) for the supported crypto algorithms

// Maximum checksum function output for the supported enctypes

// Maximum blocksize for the supported crypto algorithms

// The length of the Kerberos GSS token header

pub const KG2_TOK_MIC: c_uint = 0x0404;
pub const KG2_TOK_WRAP: c_uint = 0x0504;
pub const KG2_TOKEN_FLAG_SENTBYACCEPTOR: c_uint = 0x01;
pub const KG2_TOKEN_FLAG_SEALED: c_uint = 0x02;
pub const KG2_TOKEN_FLAG_ACCEPTORSUBKEY: c_uint = 0x04;
// from rfc4121

