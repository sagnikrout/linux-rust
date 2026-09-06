//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/asic_reg/psoc_etr_regs.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PSOC_ETR (Prototype: ETR)
//
pub const mmPSOC_ETR_RSZ: c_uint = 0x2C43004;
pub const mmPSOC_ETR_STS: c_uint = 0x2C4300C;
pub const mmPSOC_ETR_RRD: c_uint = 0x2C43010;
pub const mmPSOC_ETR_RRP: c_uint = 0x2C43014;
pub const mmPSOC_ETR_RWP: c_uint = 0x2C43018;
pub const mmPSOC_ETR_TRG: c_uint = 0x2C4301C;
pub const mmPSOC_ETR_CTL: c_uint = 0x2C43020;
pub const mmPSOC_ETR_RWD: c_uint = 0x2C43024;
pub const mmPSOC_ETR_MODE: c_uint = 0x2C43028;
pub const mmPSOC_ETR_LBUFLEVEL: c_uint = 0x2C4302C;
pub const mmPSOC_ETR_CBUFLEVEL: c_uint = 0x2C43030;
pub const mmPSOC_ETR_BUFWM: c_uint = 0x2C43034;
pub const mmPSOC_ETR_RRPHI: c_uint = 0x2C43038;
pub const mmPSOC_ETR_RWPHI: c_uint = 0x2C4303C;
pub const mmPSOC_ETR_AXICTL: c_uint = 0x2C43110;
pub const mmPSOC_ETR_DBALO: c_uint = 0x2C43118;
pub const mmPSOC_ETR_DBAHI: c_uint = 0x2C4311C;
pub const mmPSOC_ETR_FFSR: c_uint = 0x2C43300;
pub const mmPSOC_ETR_FFCR: c_uint = 0x2C43304;
pub const mmPSOC_ETR_PSCR: c_uint = 0x2C43308;
pub const mmPSOC_ETR_ITMISCOP0: c_uint = 0x2C43EE0;
pub const mmPSOC_ETR_ITTRFLIN: c_uint = 0x2C43EE8;
pub const mmPSOC_ETR_ITATBDATA0: c_uint = 0x2C43EEC;
pub const mmPSOC_ETR_ITATBCTR2: c_uint = 0x2C43EF0;
pub const mmPSOC_ETR_ITATBCTR1: c_uint = 0x2C43EF4;
pub const mmPSOC_ETR_ITATBCTR0: c_uint = 0x2C43EF8;
pub const mmPSOC_ETR_ITCTRL: c_uint = 0x2C43F00;
pub const mmPSOC_ETR_CLAIMSET: c_uint = 0x2C43FA0;
pub const mmPSOC_ETR_CLAIMCLR: c_uint = 0x2C43FA4;
pub const mmPSOC_ETR_LAR: c_uint = 0x2C43FB0;
pub const mmPSOC_ETR_LSR: c_uint = 0x2C43FB4;
pub const mmPSOC_ETR_AUTHSTATUS: c_uint = 0x2C43FB8;
pub const mmPSOC_ETR_DEVID: c_uint = 0x2C43FC8;
pub const mmPSOC_ETR_DEVTYPE: c_uint = 0x2C43FCC;
pub const mmPSOC_ETR_PERIPHID4: c_uint = 0x2C43FD0;
pub const mmPSOC_ETR_PERIPHID5: c_uint = 0x2C43FD4;
pub const mmPSOC_ETR_PERIPHID6: c_uint = 0x2C43FD8;
pub const mmPSOC_ETR_PERIPHID7: c_uint = 0x2C43FDC;
pub const mmPSOC_ETR_PERIPHID0: c_uint = 0x2C43FE0;
pub const mmPSOC_ETR_PERIPHID1: c_uint = 0x2C43FE4;
pub const mmPSOC_ETR_PERIPHID2: c_uint = 0x2C43FE8;
pub const mmPSOC_ETR_PERIPHID3: c_uint = 0x2C43FEC;
pub const mmPSOC_ETR_COMPID0: c_uint = 0x2C43FF0;
pub const mmPSOC_ETR_COMPID1: c_uint = 0x2C43FF4;
pub const mmPSOC_ETR_COMPID2: c_uint = 0x2C43FF8;
pub const mmPSOC_ETR_COMPID3: c_uint = 0x2C43FFC;
