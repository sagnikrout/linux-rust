//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_tpc0_eml_stm_regs.h
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
// DCORE0_TPC0_EML_STM
// (Prototype: STM)
//
pub const mmDCORE0_TPC0_EML_STM_STMDMASTARTR: c_uint = 0x3C04;
pub const mmDCORE0_TPC0_EML_STM_STMDMASTOPR: c_uint = 0x3C08;
pub const mmDCORE0_TPC0_EML_STM_STMDMASTATR: c_uint = 0x3C0C;
pub const mmDCORE0_TPC0_EML_STM_STMDMACTLR: c_uint = 0x3C10;
pub const mmDCORE0_TPC0_EML_STM_STMDMAIDR: c_uint = 0x3CFC;
pub const mmDCORE0_TPC0_EML_STM_STMHEER: c_uint = 0x3D00;
pub const mmDCORE0_TPC0_EML_STM_STMHETER: c_uint = 0x3D20;
pub const mmDCORE0_TPC0_EML_STM_STMHEBSR: c_uint = 0x3D60;
pub const mmDCORE0_TPC0_EML_STM_STMHEMCR: c_uint = 0x3D64;
pub const mmDCORE0_TPC0_EML_STM_STMHEEXTMUXR: c_uint = 0x3D68;
pub const mmDCORE0_TPC0_EML_STM_STMHEMASTR: c_uint = 0x3DF4;
pub const mmDCORE0_TPC0_EML_STM_STMHEFEAT1R: c_uint = 0x3DF8;
pub const mmDCORE0_TPC0_EML_STM_STMHEIDR: c_uint = 0x3DFC;
pub const mmDCORE0_TPC0_EML_STM_STMSPER: c_uint = 0x3E00;
pub const mmDCORE0_TPC0_EML_STM_STMSPTER: c_uint = 0x3E20;
pub const mmDCORE0_TPC0_EML_STM_STMSPSCR: c_uint = 0x3E60;
pub const mmDCORE0_TPC0_EML_STM_STMSPMSCR: c_uint = 0x3E64;
pub const mmDCORE0_TPC0_EML_STM_STMSPOVERRIDER: c_uint = 0x3E68;
pub const mmDCORE0_TPC0_EML_STM_STMSPMOVERRIDER: c_uint = 0x3E6C;
pub const mmDCORE0_TPC0_EML_STM_STMSPTRIGCSR: c_uint = 0x3E70;
pub const mmDCORE0_TPC0_EML_STM_STMTCSR: c_uint = 0x3E80;
pub const mmDCORE0_TPC0_EML_STM_STMTSSTIMR: c_uint = 0x3E84;
pub const mmDCORE0_TPC0_EML_STM_STMTSFREQR: c_uint = 0x3E8C;
pub const mmDCORE0_TPC0_EML_STM_STMSYNCR: c_uint = 0x3E90;
pub const mmDCORE0_TPC0_EML_STM_STMAUXCR: c_uint = 0x3E94;
pub const mmDCORE0_TPC0_EML_STM_STMFEAT1R: c_uint = 0x3EA0;
pub const mmDCORE0_TPC0_EML_STM_STMFEAT2R: c_uint = 0x3EA4;
pub const mmDCORE0_TPC0_EML_STM_STMFEAT3R: c_uint = 0x3EA8;
pub const mmDCORE0_TPC0_EML_STM_STMITTRIGGER: c_uint = 0x3EE8;
pub const mmDCORE0_TPC0_EML_STM_STMITATBDATA0: c_uint = 0x3EEC;
pub const mmDCORE0_TPC0_EML_STM_STMITATBCTR2: c_uint = 0x3EF0;
pub const mmDCORE0_TPC0_EML_STM_STMITATBID: c_uint = 0x3EF4;
pub const mmDCORE0_TPC0_EML_STM_STMITATBCTR0: c_uint = 0x3EF8;
pub const mmDCORE0_TPC0_EML_STM_STMITCTRL: c_uint = 0x3F00;
pub const mmDCORE0_TPC0_EML_STM_STMCLAIMSET: c_uint = 0x3FA0;
pub const mmDCORE0_TPC0_EML_STM_STMCLAIMCLR: c_uint = 0x3FA4;
pub const mmDCORE0_TPC0_EML_STM_STMLAR: c_uint = 0x3FB0;
pub const mmDCORE0_TPC0_EML_STM_STMLSR: c_uint = 0x3FB4;
pub const mmDCORE0_TPC0_EML_STM_STMAUTHSTATUS: c_uint = 0x3FB8;
pub const mmDCORE0_TPC0_EML_STM_STMDEVARCH: c_uint = 0x3FBC;
pub const mmDCORE0_TPC0_EML_STM_STMDEVID: c_uint = 0x3FC8;
pub const mmDCORE0_TPC0_EML_STM_STMDEVTYPE: c_uint = 0x3FCC;
pub const mmDCORE0_TPC0_EML_STM_STMPIDR4: c_uint = 0x3FD0;
pub const mmDCORE0_TPC0_EML_STM_STMPIDR5: c_uint = 0x3FD4;
pub const mmDCORE0_TPC0_EML_STM_STMPIDR6: c_uint = 0x3FD8;
pub const mmDCORE0_TPC0_EML_STM_STMPIDR7: c_uint = 0x3FDC;
pub const mmDCORE0_TPC0_EML_STM_STMPIDR0: c_uint = 0x3FE0;
pub const mmDCORE0_TPC0_EML_STM_STMPIDR1: c_uint = 0x3FE4;
pub const mmDCORE0_TPC0_EML_STM_STMPIDR2: c_uint = 0x3FE8;
pub const mmDCORE0_TPC0_EML_STM_STMPIDR3: c_uint = 0x3FEC;
pub const mmDCORE0_TPC0_EML_STM_STMCIDR0: c_uint = 0x3FF0;
pub const mmDCORE0_TPC0_EML_STM_STMCIDR1: c_uint = 0x3FF4;
pub const mmDCORE0_TPC0_EML_STM_STMCIDR2: c_uint = 0x3FF8;
pub const mmDCORE0_TPC0_EML_STM_STMCIDR3: c_uint = 0x3FFC;
