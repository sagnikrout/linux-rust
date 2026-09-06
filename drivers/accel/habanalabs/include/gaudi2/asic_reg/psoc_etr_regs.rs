//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/psoc_etr_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PSOC_ETR
// (Prototype: ETR)
//
pub const mmPSOC_ETR_RSZ: c_uint = 0x6C44004;
pub const mmPSOC_ETR_STS: c_uint = 0x6C4400C;
pub const mmPSOC_ETR_RRD: c_uint = 0x6C44010;
pub const mmPSOC_ETR_RRP: c_uint = 0x6C44014;
pub const mmPSOC_ETR_RWP: c_uint = 0x6C44018;
pub const mmPSOC_ETR_TRG: c_uint = 0x6C4401C;
pub const mmPSOC_ETR_CTL: c_uint = 0x6C44020;
pub const mmPSOC_ETR_RWD: c_uint = 0x6C44024;
pub const mmPSOC_ETR_MODE: c_uint = 0x6C44028;
pub const mmPSOC_ETR_LBUFLEVEL: c_uint = 0x6C4402C;
pub const mmPSOC_ETR_CBUFLEVEL: c_uint = 0x6C44030;
pub const mmPSOC_ETR_BUFWM: c_uint = 0x6C44034;
pub const mmPSOC_ETR_RRPHI: c_uint = 0x6C44038;
pub const mmPSOC_ETR_RWPHI: c_uint = 0x6C4403C;
pub const mmPSOC_ETR_AXICTL: c_uint = 0x6C44110;
pub const mmPSOC_ETR_DBALO: c_uint = 0x6C44118;
pub const mmPSOC_ETR_DBAHI: c_uint = 0x6C4411C;
pub const mmPSOC_ETR_FFSR: c_uint = 0x6C44300;
pub const mmPSOC_ETR_FFCR: c_uint = 0x6C44304;
pub const mmPSOC_ETR_PSCR: c_uint = 0x6C44308;
pub const mmPSOC_ETR_ITMISCOP0: c_uint = 0x6C44EE0;
pub const mmPSOC_ETR_ITTRFLIN: c_uint = 0x6C44EE8;
pub const mmPSOC_ETR_ITATBDATA0: c_uint = 0x6C44EEC;
pub const mmPSOC_ETR_ITATBCTR2: c_uint = 0x6C44EF0;
pub const mmPSOC_ETR_ITATBCTR1: c_uint = 0x6C44EF4;
pub const mmPSOC_ETR_ITATBCTR0: c_uint = 0x6C44EF8;
pub const mmPSOC_ETR_ITCTRL: c_uint = 0x6C44F00;
pub const mmPSOC_ETR_CLAIMSET: c_uint = 0x6C44FA0;
pub const mmPSOC_ETR_CLAIMCLR: c_uint = 0x6C44FA4;
pub const mmPSOC_ETR_LAR: c_uint = 0x6C44FB0;
pub const mmPSOC_ETR_LSR: c_uint = 0x6C44FB4;
pub const mmPSOC_ETR_AUTHSTATUS: c_uint = 0x6C44FB8;
pub const mmPSOC_ETR_DEVID: c_uint = 0x6C44FC8;
pub const mmPSOC_ETR_DEVTYPE: c_uint = 0x6C44FCC;
pub const mmPSOC_ETR_PERIPHID4: c_uint = 0x6C44FD0;
pub const mmPSOC_ETR_PERIPHID5: c_uint = 0x6C44FD4;
pub const mmPSOC_ETR_PERIPHID6: c_uint = 0x6C44FD8;
pub const mmPSOC_ETR_PERIPHID7: c_uint = 0x6C44FDC;
pub const mmPSOC_ETR_PERIPHID0: c_uint = 0x6C44FE0;
pub const mmPSOC_ETR_PERIPHID1: c_uint = 0x6C44FE4;
pub const mmPSOC_ETR_PERIPHID2: c_uint = 0x6C44FE8;
pub const mmPSOC_ETR_PERIPHID3: c_uint = 0x6C44FEC;
pub const mmPSOC_ETR_COMPID0: c_uint = 0x6C44FF0;
pub const mmPSOC_ETR_COMPID1: c_uint = 0x6C44FF4;
pub const mmPSOC_ETR_COMPID2: c_uint = 0x6C44FF8;
pub const mmPSOC_ETR_COMPID3: c_uint = 0x6C44FFC;
