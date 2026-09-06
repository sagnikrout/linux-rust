//! Automatically rewritten from C Header to Rust Module
//! Source: crypto/asymmetric_keys/verify_pefile.h
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
// PE Binary parser bits
//
// Copyright (C) 2014 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pefile_context {
    pub header_size: unsigned,
    pub image_checksum_offset: unsigned,
    pub cert_dirent_offset: unsigned,
    pub n_data_dirents: unsigned,
    pub n_sections: unsigned,
    pub certs_size: unsigned,
    pub sig_offset: unsigned,
    pub sig_len: unsigned,
    pub secs: *const section_header,
// PKCS#7 MS Individual Code Signing content
    pub /: *const *const *const void digest; / Digest,
    pub /: *mut *mut unsigned digest_len; / Digest length,
    pub /: *const *const *const char digest_algo; / Digest algorithm,
}

//
// mscode_parser.c
//
