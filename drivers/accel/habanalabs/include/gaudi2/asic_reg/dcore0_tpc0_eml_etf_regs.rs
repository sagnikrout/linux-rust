//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_tpc0_eml_etf_regs.h
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
// DCORE0_TPC0_EML_ETF
// (Prototype: ETF_1KB)
//
pub const mmDCORE0_TPC0_EML_ETF_RSZ: c_uint = 0x2004;
pub const mmDCORE0_TPC0_EML_ETF_STS: c_uint = 0x200C;
pub const mmDCORE0_TPC0_EML_ETF_RRD: c_uint = 0x2010;
pub const mmDCORE0_TPC0_EML_ETF_RRP: c_uint = 0x2014;
pub const mmDCORE0_TPC0_EML_ETF_RWP: c_uint = 0x2018;
pub const mmDCORE0_TPC0_EML_ETF_TRG: c_uint = 0x201C;
pub const mmDCORE0_TPC0_EML_ETF_CTL: c_uint = 0x2020;
pub const mmDCORE0_TPC0_EML_ETF_RWD: c_uint = 0x2024;
pub const mmDCORE0_TPC0_EML_ETF_MODE: c_uint = 0x2028;
pub const mmDCORE0_TPC0_EML_ETF_LBUFLEVEL: c_uint = 0x202C;
pub const mmDCORE0_TPC0_EML_ETF_CBUFLEVEL: c_uint = 0x2030;
pub const mmDCORE0_TPC0_EML_ETF_BUFWM: c_uint = 0x2034;
pub const mmDCORE0_TPC0_EML_ETF_FFSR: c_uint = 0x2300;
pub const mmDCORE0_TPC0_EML_ETF_FFCR: c_uint = 0x2304;
pub const mmDCORE0_TPC0_EML_ETF_PSCR: c_uint = 0x2308;
pub const mmDCORE0_TPC0_EML_ETF_ITATBMDATA0: c_uint = 0x2ED0;
pub const mmDCORE0_TPC0_EML_ETF_ITATBMCTR2: c_uint = 0x2ED4;
pub const mmDCORE0_TPC0_EML_ETF_ITATBMCTR1: c_uint = 0x2ED8;
pub const mmDCORE0_TPC0_EML_ETF_ITATBMCTR0: c_uint = 0x2EDC;
pub const mmDCORE0_TPC0_EML_ETF_ITMISCOP0: c_uint = 0x2EE0;
pub const mmDCORE0_TPC0_EML_ETF_ITTRFLIN: c_uint = 0x2EE8;
pub const mmDCORE0_TPC0_EML_ETF_ITATBDATA0: c_uint = 0x2EEC;
pub const mmDCORE0_TPC0_EML_ETF_ITATBCTR2: c_uint = 0x2EF0;
pub const mmDCORE0_TPC0_EML_ETF_ITATBCTR1: c_uint = 0x2EF4;
pub const mmDCORE0_TPC0_EML_ETF_ITATBCTR0: c_uint = 0x2EF8;
pub const mmDCORE0_TPC0_EML_ETF_ITCTRL: c_uint = 0x2F00;
pub const mmDCORE0_TPC0_EML_ETF_CLAIMSET: c_uint = 0x2FA0;
pub const mmDCORE0_TPC0_EML_ETF_CLAIMCLR: c_uint = 0x2FA4;
pub const mmDCORE0_TPC0_EML_ETF_LAR: c_uint = 0x2FB0;
pub const mmDCORE0_TPC0_EML_ETF_LSR: c_uint = 0x2FB4;
pub const mmDCORE0_TPC0_EML_ETF_AUTHSTATUS: c_uint = 0x2FB8;
pub const mmDCORE0_TPC0_EML_ETF_DEVID: c_uint = 0x2FC8;
pub const mmDCORE0_TPC0_EML_ETF_DEVTYPE: c_uint = 0x2FCC;
pub const mmDCORE0_TPC0_EML_ETF_PERIPHID4: c_uint = 0x2FD0;
pub const mmDCORE0_TPC0_EML_ETF_PERIPHID5: c_uint = 0x2FD4;
pub const mmDCORE0_TPC0_EML_ETF_PERIPHID6: c_uint = 0x2FD8;
pub const mmDCORE0_TPC0_EML_ETF_PERIPHID7: c_uint = 0x2FDC;
pub const mmDCORE0_TPC0_EML_ETF_PERIPHID0: c_uint = 0x2FE0;
pub const mmDCORE0_TPC0_EML_ETF_PERIPHID1: c_uint = 0x2FE4;
pub const mmDCORE0_TPC0_EML_ETF_PERIPHID2: c_uint = 0x2FE8;
pub const mmDCORE0_TPC0_EML_ETF_PERIPHID3: c_uint = 0x2FEC;
pub const mmDCORE0_TPC0_EML_ETF_COMPID0: c_uint = 0x2FF0;
pub const mmDCORE0_TPC0_EML_ETF_COMPID1: c_uint = 0x2FF4;
pub const mmDCORE0_TPC0_EML_ETF_COMPID2: c_uint = 0x2FF8;
pub const mmDCORE0_TPC0_EML_ETF_COMPID3: c_uint = 0x2FFC;
