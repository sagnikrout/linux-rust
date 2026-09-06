//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/csiostor/csio_defs.h
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
// This file is part of the Chelsio FCoE driver for Linux.
//
// Copyright (c) 2008-2012 Chelsio Communications, Inc. All rights reserved.
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

pub const CSIO_INVALID_IDX: c_uint = 0xFFFFFFFF;

pub const CSIO_DID_MASK: c_uint = 0xFFFFFF;
pub const CSIO_WORD_TO_BYTE: c_int = 4;

extern "C" {
    pub fn readl(32: addr) + ((u64)readl(addr + 4) <<) -> return;
}

// State machine
// State machine evets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_ln_ev {
    CSIO_LNE_NONE = (uint32_t)0,
    CSIO_LNE_LINKUP,
    CSIO_LNE_FAB_INIT_DONE,
    CSIO_LNE_LINK_DOWN,
    CSIO_LNE_DOWN_LINK,
    CSIO_LNE_LOGO,
    CSIO_LNE_CLOSE,
    CSIO_LNE_MAX_EVENT,
}

extern "C" {
    pub fn void(ln: *mut *mut csio_sm_state_t)(struct csio_lnode, evt: csio_ln_ev) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_sm {
    pub sm_list: list_head,
    pub sm_state: csio_sm_state_t,
}

// Macro flag: #define CSIO_DB_ASSERT(__c)

