//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_arm.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2015-2017 Netronome Systems, Inc.
//
// nfp_arm.h
// Definitions for ARM-based registers and memory spaces
//

pub const NFP_ARM_IM: c_uint = 0x200000;
pub const NFP_ARM_EM: c_uint = 0x300000;
pub const NFP_ARM_GCSR: c_uint = 0x400000;
pub const NFP_ARM_MPCORE: c_uint = 0x800000;
pub const NFP_ARM_PL310: c_uint = 0xa00000;
// Register Type: BulkBARConfig

// Register Type: ExpansionBARConfig

// Register Type: ExplicitBARConfig0_Reg

// Register Type: ExplicitBARConfig1_Reg

// Register Type: ExplicitBARConfig2_Reg

// Register Type: PostedCommandSignal

// Register Type: MPCoreBaseAddress
pub const NFP_ARM_GCSR_MPCORE_BASE: c_uint = 0x00e0;

// Register Type: PL310BaseAddress
pub const NFP_ARM_GCSR_PL310_BASE: c_uint = 0x00e4;

// Register Type: MPCoreConfig
pub const NFP_ARM_GCSR_MP0_CFG: c_uint = 0x00e8;

// Register Type: MPCoreIDCacheDataError
pub const NFP_ARM_GCSR_MP0_CACHE_ERR: c_uint = 0x00ec;

// Register Type: ARMDFT
pub const NFP_ARM_GCSR_DFT: c_uint = 0x0100;

// Gasket CSRs
// NOTE: These cannot be remapped, and are always at this location.
//

// BAR CSRs
//
pub const NFP_ARM_GCSR_BULK_BITS: c_int = 11;
pub const NFP_ARM_GCSR_EXPA_BITS: c_int = 15;
pub const NFP_ARM_GCSR_EXPL_BITS: c_int = 18;

// MP Core CSRs

// PL320 CSRs

