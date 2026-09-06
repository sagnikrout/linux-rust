//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/rvu_reg.h
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
// Marvell RVU Admin Function driver
//
// Copyright (C) 2018 Marvell.
//
// Admin function registers

// Admin function's privileged PF/VF registers

// RVU PF registers

// RVU VF registers

// NPA block's admin function registers

// NIX block's admin function registers

// SSO

// SSOW

// TIM

// CPT

pub const CPT_AF_BAR2_SEL: c_uint = 0x9000000;

pub const CPT_AF_LF_CTL2_SHIFT: c_int = 3;
pub const CPT_AF_LF_SSO_PF_FUNC_SHIFT: c_int = 32;
pub const CPT_LF_CTL: c_uint = 0x10;
pub const CPT_LF_INPROG: c_uint = 0x40;
pub const CPT_LF_Q_SIZE: c_uint = 0x100;
pub const CPT_LF_Q_INST_PTR: c_uint = 0x110;
pub const CPT_LF_Q_GRP_PTR: c_uint = 0x120;
pub const CPT_LF_CTX_FLUSH: c_uint = 0x510;

// NPC

// NDC

// LBK

// APR

pub const APR_LMT_MAP_ENT_DIS_SCH_CMP_SHIFT: c_int = 23;
pub const APR_LMT_MAP_ENT_SCH_ENA_SHIFT: c_int = 22;
pub const APR_LMT_MAP_ENT_DIS_LINE_PREF_SHIFT: c_int = 21;

pub const LMTST_WR_PEND_MAX: c_int = 15;
