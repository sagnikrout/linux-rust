//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pdma1_qm_axuser_nonsecured_regs.h
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
// PDMA1_QM_AXUSER_NONSECURED
// (Prototype: AXUSER)
//
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_ASID: c_uint = 0x4C9AB80;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_MMU_BP: c_uint = 0x4C9AB84;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_STRONG_ORDER: c_uint = 0x4C9AB88;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_NO_SNOOP: c_uint = 0x4C9AB8C;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_WR_REDUCTION: c_uint = 0x4C9AB90;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_RD_ATOMIC: c_uint = 0x4C9AB94;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_QOS: c_uint = 0x4C9AB98;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_RSVD: c_uint = 0x4C9AB9C;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_EMEM_CPAGE: c_uint = 0x4C9ABA0;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_CORE: c_uint = 0x4C9ABA4;
pub const mmPDMA1_QM_AXUSER_NONSECURED_E2E_COORD: c_uint = 0x4C9ABA8;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_WR_OVRD_LO: c_uint = 0x4C9ABB0;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_WR_OVRD_HI: c_uint = 0x4C9ABB4;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_RD_OVRD_LO: c_uint = 0x4C9ABB8;
pub const mmPDMA1_QM_AXUSER_NONSECURED_HB_RD_OVRD_HI: c_uint = 0x4C9ABBC;
pub const mmPDMA1_QM_AXUSER_NONSECURED_LB_COORD: c_uint = 0x4C9ABC0;
pub const mmPDMA1_QM_AXUSER_NONSECURED_LB_LOCK: c_uint = 0x4C9ABC4;
pub const mmPDMA1_QM_AXUSER_NONSECURED_LB_RSVD: c_uint = 0x4C9ABC8;
pub const mmPDMA1_QM_AXUSER_NONSECURED_LB_OVRD: c_uint = 0x4C9ABCC;
