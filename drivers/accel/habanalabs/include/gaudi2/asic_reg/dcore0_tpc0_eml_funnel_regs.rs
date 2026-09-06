//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_tpc0_eml_funnel_regs.h
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
// DCORE0_TPC0_EML_FUNNEL
// (Prototype: FUNNEL_2X1)
//
pub const mmDCORE0_TPC0_EML_FUNNEL_CTRL_REG: c_uint = 0x6000;
pub const mmDCORE0_TPC0_EML_FUNNEL_PRIORITY_CTRL_REG: c_uint = 0x6004;
pub const mmDCORE0_TPC0_EML_FUNNEL_ITATBDATA0: c_uint = 0x6EEC;
pub const mmDCORE0_TPC0_EML_FUNNEL_ITATBCTR2: c_uint = 0x6EF0;
pub const mmDCORE0_TPC0_EML_FUNNEL_ITATBCTR1: c_uint = 0x6EF4;
pub const mmDCORE0_TPC0_EML_FUNNEL_ITATBCTR0: c_uint = 0x6EF8;
pub const mmDCORE0_TPC0_EML_FUNNEL_ITCTRL: c_uint = 0x6F00;
pub const mmDCORE0_TPC0_EML_FUNNEL_CLAIMSET: c_uint = 0x6FA0;
pub const mmDCORE0_TPC0_EML_FUNNEL_CLAIMCLR: c_uint = 0x6FA4;
pub const mmDCORE0_TPC0_EML_FUNNEL_LOCKACCESS: c_uint = 0x6FB0;
pub const mmDCORE0_TPC0_EML_FUNNEL_LOCKSTATUS: c_uint = 0x6FB4;
pub const mmDCORE0_TPC0_EML_FUNNEL_AUTHSTATUS: c_uint = 0x6FB8;
pub const mmDCORE0_TPC0_EML_FUNNEL_DEVID: c_uint = 0x6FC8;
pub const mmDCORE0_TPC0_EML_FUNNEL_DEVTYPE: c_uint = 0x6FCC;
pub const mmDCORE0_TPC0_EML_FUNNEL_PIDR4: c_uint = 0x6FD0;
pub const mmDCORE0_TPC0_EML_FUNNEL_PERIPHID5: c_uint = 0x6FD4;
pub const mmDCORE0_TPC0_EML_FUNNEL_PERIPHID6: c_uint = 0x6FD8;
pub const mmDCORE0_TPC0_EML_FUNNEL_PERIPHID7: c_uint = 0x6FDC;
pub const mmDCORE0_TPC0_EML_FUNNEL_PIDR0: c_uint = 0x6FE0;
pub const mmDCORE0_TPC0_EML_FUNNEL_PIDR1: c_uint = 0x6FE4;
pub const mmDCORE0_TPC0_EML_FUNNEL_PIDR2: c_uint = 0x6FE8;
pub const mmDCORE0_TPC0_EML_FUNNEL_PIDR3: c_uint = 0x6FEC;
pub const mmDCORE0_TPC0_EML_FUNNEL_CID0: c_uint = 0x6FF0;
pub const mmDCORE0_TPC0_EML_FUNNEL_CID1: c_uint = 0x6FF4;
pub const mmDCORE0_TPC0_EML_FUNNEL_CID2: c_uint = 0x6FF8;
pub const mmDCORE0_TPC0_EML_FUNNEL_CID3: c_uint = 0x6FFC;
