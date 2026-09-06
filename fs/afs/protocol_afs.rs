//! Automatically rewritten from C Header to Rust Module
//! Source: fs/afs/protocol_afs.h
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
// AFS protocol bits
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

// AFS3 Fileserver capabilities word 0
pub const AFS3_VICED_CAPABILITY_ERRORTRANS: c_uint = 0x0001 /* Uses UAE errors */;
pub const AFS3_VICED_CAPABILITY_64BITFILES: c_uint = 0x0002 /* FetchData64 & StoreData64 supported */;
pub const AFS3_VICED_CAPABILITY_WRITELOCKACL: c_uint = 0x0004 /* Can lock a file even without lock perm */;
pub const AFS3_VICED_CAPABILITY_SANEACLS: c_uint = 0x0008 /* ACLs reviewed for sanity - don't use */;
