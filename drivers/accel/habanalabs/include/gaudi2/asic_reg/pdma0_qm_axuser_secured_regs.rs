//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pdma0_qm_axuser_secured_regs.h
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
// PDMA0_QM_AXUSER_SECURED
// (Prototype: AXUSER)
//
pub const mmPDMA0_QM_AXUSER_SECURED_HB_ASID: c_uint = 0x4C8AB00;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_MMU_BP: c_uint = 0x4C8AB04;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_STRONG_ORDER: c_uint = 0x4C8AB08;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_NO_SNOOP: c_uint = 0x4C8AB0C;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_WR_REDUCTION: c_uint = 0x4C8AB10;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_RD_ATOMIC: c_uint = 0x4C8AB14;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_QOS: c_uint = 0x4C8AB18;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_RSVD: c_uint = 0x4C8AB1C;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_EMEM_CPAGE: c_uint = 0x4C8AB20;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_CORE: c_uint = 0x4C8AB24;
pub const mmPDMA0_QM_AXUSER_SECURED_E2E_COORD: c_uint = 0x4C8AB28;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_WR_OVRD_LO: c_uint = 0x4C8AB30;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_WR_OVRD_HI: c_uint = 0x4C8AB34;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_RD_OVRD_LO: c_uint = 0x4C8AB38;
pub const mmPDMA0_QM_AXUSER_SECURED_HB_RD_OVRD_HI: c_uint = 0x4C8AB3C;
pub const mmPDMA0_QM_AXUSER_SECURED_LB_COORD: c_uint = 0x4C8AB40;
pub const mmPDMA0_QM_AXUSER_SECURED_LB_LOCK: c_uint = 0x4C8AB44;
pub const mmPDMA0_QM_AXUSER_SECURED_LB_RSVD: c_uint = 0x4C8AB48;
pub const mmPDMA0_QM_AXUSER_SECURED_LB_OVRD: c_uint = 0x4C8AB4C;
