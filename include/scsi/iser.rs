//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/iser.h
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
// Copyright (c) 2015 Mellanox Technologies. All rights reserved.
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
pub const ISER_ZBVA_NOT_SUP: c_uint = 0x80;
pub const ISER_SEND_W_INV_NOT_SUP: c_uint = 0x40;
pub const ISERT_ZBVA_NOT_USED: c_uint = 0x80;
pub const ISERT_SEND_W_INV_NOT_USED: c_uint = 0x40;
pub const ISCSI_CTRL: c_uint = 0x10;
pub const ISER_HELLO: c_uint = 0x20;
pub const ISER_HELLORPLY: c_uint = 0x30;
pub const ISER_VER: c_uint = 0x10;
pub const ISER_WSV: c_uint = 0x08;
pub const ISER_RSV: c_uint = 0x04;
//
// struct iser_cm_hdr - iSER CM header (from iSER Annex A12)
//
// @flags:        flags support (zbva, send_w_inv)
// @rsvd:         reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_cm_hdr {
    pub flags: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
//
// struct iser_ctrl - iSER header of iSCSI control PDU
//
// @flags:        opcode and read/write valid bits
// @rsvd:         reserved
// @write_stag:   write rkey
// @write_va:     write virtual address
// @read_stag:    read rkey
// @read_va:      read virtual address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iser_ctrl {
    pub flags: u8,
    pub rsvd: [u8; 3],
    pub write_stag: __be32,
    pub write_va: __be64,
    pub read_stag: __be32,
    pub read_va: __be64,
    pub __packed: },
