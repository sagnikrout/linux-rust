//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/crw.h
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
// Data definitions for channel report processing
// Copyright IBM Corp. 2000, 2009
// Author(s): Ingo Adlung <adlung@de.ibm.com>,
// Martin Schwidefsky <schwidefsky@de.ibm.com>,
// Cornelia Huck <cornelia.huck@de.ibm.com>,
//

//
// Channel Report Word
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crw {
    pub /: *mut *mut __u32 res1 : 1; / reserved zero,
    pub /: *mut *mut __u32 slct : 1; / solicited,
    pub /: *mut *mut __u32 oflw : 1; / overflow,
    pub /: *mut *mut __u32 chn : 1; / chained,
    pub /: *mut *mut __u32 rsc : 4; / reporting source code,
    pub /: *mut *mut __u32 anc : 1; / ancillary report,
    pub /: *mut *mut __u32 res2 : 1; / reserved zero,
    pub /: *mut *mut __u32 erc : 6; / error-recovery code,
    pub /: *mut *mut __u32 rsid : 16; / reporting-source ID,
// C attribute field omitted
    pub int): *mut *mut *mut *mut typedef void (crw_handler_t)(struct crw , struct crw ,,
    pub handler): extern int crw_register_handler(int rsc, crw_handler_t,
    pub rsc): extern void crw_unregister_handler(int,
    pub crw_handle_channel_report(void): extern void,
    pub crw_wait_for_channel_report(void): c_void,
pub const NR_RSCS: c_int = 16;
pub const CRW_RSC_MONITOR: c_uint = 0x2  /* monitoring facility */;
pub const CRW_RSC_SCH: c_uint = 0x3  /* subchannel */;
pub const CRW_RSC_CPATH: c_uint = 0x4  /* channel path */;
pub const CRW_RSC_CONFIG: c_uint = 0x9  /* configuration-alert facility */;
pub const CRW_RSC_CSS: c_uint = 0xB  /* channel subsystem */;
pub const CRW_ERC_EVENT: c_uint = 0x00 /* event information pending */;
pub const CRW_ERC_AVAIL: c_uint = 0x01 /* available */;
pub const CRW_ERC_INIT: c_uint = 0x02 /* initialized */;
pub const CRW_ERC_TERROR: c_uint = 0x03 /* temporary error */;
pub const CRW_ERC_IPARM: c_uint = 0x04 /* installed parm initialized */;
pub const CRW_ERC_TERM: c_uint = 0x05 /* terminal */;
pub const CRW_ERC_PERRN: c_uint = 0x06 /* perm. error, fac. not init */;
pub const CRW_ERC_PERRI: c_uint = 0x07 /* perm. error, facility init */;
pub const CRW_ERC_PMOD: c_uint = 0x08 /* installed parameters modified */;
