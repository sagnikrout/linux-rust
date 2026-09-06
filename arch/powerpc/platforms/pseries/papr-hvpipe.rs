//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/pseries/papr-hvpipe.h
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


// SPDX-License-Identifier: GPL-2.0-only
pub const HVPIPE_HMC_ID_MASK: c_uint = 0x02000000 /*02-HMC,00-reserved and HMC ID */;
pub const HVPIPE_MAX_WRITE_BUFFER_SIZE: c_int = 4048;
//
// hvpipe specific RTAS return values
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hvpipe_migrate_action {
    HVPIPE_SUSPEND,
    HVPIPE_RESUME,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvpipe_source_info {
    pub /: *mut *mut list_head list; / list of sources,
    pub srcID: u32,
    pub hvpipe_status: u32,
    pub /: *mut *mut wait_queue_head_t recv_wqh; / wake up poll() waitq,
}

//
// Source ID Format 0xCCRRQQQQ
// CC = indicating value is source type (ex: 0x02 for HMC)
// RR = 0x00 (reserved)
// QQQQ = 0x0000 – 0xFFFF indicating the source index indetifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvpipe_event_buf {
    pub /: *mut *mut __be32 srcID; / Source ID,
    pub /: *mut *mut u8 event_type; / 0x01 for hvpipe message available,
// from specified src ID
// 0x02 for loss of pipe connection
// with specified src ID
}

extern "C" {
    pub fn hvpipe_migration_handler(action: c_int);
}
