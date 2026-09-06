//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/xs_wire.h
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


// SPDX-License-Identifier: MIT
//
// Details of the "wire" protocol between Xen Store Daemon and client
// library or guest kernel.
// Copyright (C) 2005 Rusty Russell IBM Corporation
//

// XS_RESTRICT has been removed

// We hand errors as strings, for portability.

// Generally followed by nul-terminated string(s).
// Inter-domain shared memory communications.
pub const XENSTORE_RING_SIZE: c_int = 1024;
pub type XENSTORE_RING_IDX = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenstore_domain_interface {
    pub /: *mut *mut char req[XENSTORE_RING_SIZE]; / Requests to xenstore daemon.,
    pub /: *mut *mut char rsp[XENSTORE_RING_SIZE]; / Replies and async watch events.,
    pub req_prod: XENSTORE_RING_IDX req_cons,,
    pub rsp_prod: XENSTORE_RING_IDX rsp_cons,,
    pub /: *mut *mut uint32_t server_features; / Bitmap of features supported by the server,
    pub connection: u32,
    pub error: u32,
}

// Violating this is very bad.  See docs/misc/xenstore.txt.
pub const XENSTORE_PAYLOAD_MAX: c_int = 4096;
// Violating these just gets you an error back
pub const XENSTORE_ABS_PATH_MAX: c_int = 3072;
pub const XENSTORE_REL_PATH_MAX: c_int = 2048;
// The ability to reconnect a ring
pub const XENSTORE_SERVER_FEATURE_RECONNECTION: c_int = 1;
// The presence of the "error" field in the ring page
pub const XENSTORE_SERVER_FEATURE_ERROR: c_int = 2;
// Valid values for the connection field

// Valid values for the error field

