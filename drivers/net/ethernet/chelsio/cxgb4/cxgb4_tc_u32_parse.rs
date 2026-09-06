//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4_tc_u32_parse.h
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
// This file is part of the Chelsio T4 Ethernet driver for Linux.
//
// Copyright (c) 2016 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_match_field {
    pub /: *mut *mut int off; / Offset from the beginning of the header to match,
// Fill the value/mask pair in the spec if matched
    pub mask): *mut *mut *mut int (val)(struct ch_filter_specification f, __be32 val, __be32,
}

// IPv4 match fields
// IPv6 match fields
// TCP/UDP match
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_next_header {
// Offset, shift, and mask added to beginning of the header
// to get to next header.  Useful when using a header
// field's value to jump to next header such as IHL field
// in IPv4 header.
//
    pub sel: tc_u32_sel_hdr,
    pub key: tc_u32_key,
// location of jump to make
    pub jump: *const cxgb4_match_field,
}

// Accept a rule with a jump to transport layer header based on IHL field in
// IPv4 header.
//
// TCP Jump
// UDP Jump
// Accept a rule with a jump directly past the 40 Bytes of IPv6 fixed header
// to get to transport layer header.
//
// TCP Jump
// UDP Jump
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_link {
    pub /: *const *const *const cxgb4_match_field match_field; / Next header,
    pub /: *mut *mut ch_filter_specification fs; / Match spec associated with link,
    pub /: *mut *mut u32 link_handle; / Knode handle associated with the link,
    pub /: *mut *mut *mut unsigned long tid_map; / Bitmap for filter tids,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_tc_u32_table {
    pub /: *mut *mut unsigned int size; / number of entries in table,
    pub /: *mut *mut cxgb4_link table[] __counted_by(size); / Jump table,
}
