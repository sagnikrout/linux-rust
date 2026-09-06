//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_mme_acc_regs.h
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
// DCORE0_MME_ACC
// (Prototype: ACC)
//
pub const mmDCORE0_MME_ACC_WBC0_AXI: c_uint = 0x40F8000;
pub const mmDCORE0_MME_ACC_WBC1_AXI: c_uint = 0x40F8004;
pub const mmDCORE0_MME_ACC_WBC0_RL: c_uint = 0x40F8008;
pub const mmDCORE0_MME_ACC_WBC1_RL: c_uint = 0x40F800C;
pub const mmDCORE0_MME_ACC_WBC_STALL: c_uint = 0x40F8010;
pub const mmDCORE0_MME_ACC_AWCACHE: c_uint = 0x40F8014;
pub const mmDCORE0_MME_ACC_AWPROT: c_uint = 0x40F8018;
pub const mmDCORE0_MME_ACC_AP_LFSR_POLY: c_uint = 0x40F801C;
pub const mmDCORE0_MME_ACC_AP_LFSR_SEED_WDATA: c_uint = 0x40F8020;
pub const mmDCORE0_MME_ACC_AP_LFSR_SEED_SEL: c_uint = 0x40F8024;
pub const mmDCORE0_MME_ACC_AP_LFSR_SEED_RDATA: c_uint = 0x40F8028;
pub const mmDCORE0_MME_ACC_AP_LFSR_CLOSE_CGATE_DLY: c_uint = 0x40F802C;
pub const mmDCORE0_MME_ACC_WBC_SRC_BP: c_uint = 0x40F8030;
pub const mmDCORE0_MME_ACC_CLK_GATE_EN: c_uint = 0x40F8034;
pub const mmDCORE0_MME_ACC_WBC_INFLIGHTS: c_uint = 0x40F8038;
pub const mmDCORE0_MME_ACC_HBW_CLK_ENABLER_DIS: c_uint = 0x40F803C;
pub const mmDCORE0_MME_ACC_E2E_CRDT_TOP0: c_uint = 0x40F8040;
pub const mmDCORE0_MME_ACC_E2E_CRDT_TOP1: c_uint = 0x40F8044;
pub const mmDCORE0_MME_ACC_INTR_CAUSE: c_uint = 0x40F8048;
pub const mmDCORE0_MME_ACC_INTR_MASK: c_uint = 0x40F804C;
pub const mmDCORE0_MME_ACC_INTR_CLEAR: c_uint = 0x40F8050;
pub const mmDCORE0_MME_ACC_WR_AXI_AGG_COUT0: c_uint = 0x40F8054;
pub const mmDCORE0_MME_ACC_WR_AXI_AGG_COUT1: c_uint = 0x40F8058;
pub const mmDCORE0_MME_ACC_BIST: c_uint = 0x40F805C;
pub const mmDCORE0_MME_ACC_WR_AXI_AGG_2P_BVALID: c_uint = 0x40F8060;
