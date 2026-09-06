//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/chpid.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright IBM Corp. 2007, 2012
// Author(s): Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_path_desc_fmt0 {
    pub flags: u8,
    pub lsn: u8,
    pub desc: u8,
    pub chpid: u8,
    pub swla: u8,
    pub zeroes: u8,
    pub chla: u8,
    pub chpp: u8,
    pub __packed: },
    pub chp_id)): memset(chpid, 0, sizeof(struct,
    pub b->cssid): return (a->id == b->id) && (a->cssid ==,
    pub 0: chpid->id =,
    pub __MAX_CSSID): return (chpid->cssid <=,

    pub chp_id_next(c)): for (chp_id_init(c); chp_id_is_valid(c);,
