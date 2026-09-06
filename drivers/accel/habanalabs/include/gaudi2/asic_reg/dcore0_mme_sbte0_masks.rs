//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_mme_sbte0_masks.h
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
// DCORE0_MME_SBTE0
// (Prototype: SB)
//
// DCORE0_MME_SBTE0_MAX_SIZE
pub const DCORE0_MME_SBTE0_MAX_SIZE_DATA_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_MAX_SIZE_DATA_MASK: c_uint = 0xFFFF;
pub const DCORE0_MME_SBTE0_MAX_SIZE_MD_SHIFT: c_int = 16;
pub const DCORE0_MME_SBTE0_MAX_SIZE_MD_MASK: c_uint = 0xFFFF0000;
// DCORE0_MME_SBTE0_FORCE_MISS
pub const DCORE0_MME_SBTE0_FORCE_MISS_R_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_FORCE_MISS_R_MASK: c_uint = 0x1;
// DCORE0_MME_SBTE0_MAX
pub const DCORE0_MME_SBTE0_MAX_OS_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_MAX_OS_MASK: c_uint = 0xFFFF;
// DCORE0_MME_SBTE0_RL
pub const DCORE0_MME_SBTE0_RL_SATURATION_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_RL_SATURATION_MASK: c_uint = 0xFF;
pub const DCORE0_MME_SBTE0_RL_TIMEOUT_SHIFT: c_int = 8;
pub const DCORE0_MME_SBTE0_RL_TIMEOUT_MASK: c_uint = 0xFF00;
pub const DCORE0_MME_SBTE0_RL_RATE_LIMITER_EN_SHIFT: c_int = 16;
pub const DCORE0_MME_SBTE0_RL_RATE_LIMITER_EN_MASK: c_uint = 0x10000;
// DCORE0_MME_SBTE0_SB_STALL
pub const DCORE0_MME_SBTE0_SB_STALL_R_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_SB_STALL_R_MASK: c_uint = 0x1;
// DCORE0_MME_SBTE0_INTR
pub const DCORE0_MME_SBTE0_INTR_I0_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_INTR_I0_MASK: c_uint = 0x1;
// DCORE0_MME_SBTE0_ARUSER
pub const DCORE0_MME_SBTE0_ARUSER_ASID_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_ARUSER_ASID_MASK: c_uint = 0x3FF;
pub const DCORE0_MME_SBTE0_ARUSER_MMBP_SHIFT: c_int = 10;
pub const DCORE0_MME_SBTE0_ARUSER_MMBP_MASK: c_uint = 0x400;
pub const DCORE0_MME_SBTE0_ARUSER_DUMMY_SHIFT: c_int = 11;
pub const DCORE0_MME_SBTE0_ARUSER_DUMMY_MASK: c_uint = 0xFFFFF800;
// DCORE0_MME_SBTE0_ARCACHE
pub const DCORE0_MME_SBTE0_ARCACHE_N_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_ARCACHE_N_MASK: c_uint = 0xF;
// DCORE0_MME_SBTE0_STATUS
pub const DCORE0_MME_SBTE0_STATUS_DROP_CNT_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_STATUS_DROP_CNT_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_MME_SBTE0_PRTN
pub const DCORE0_MME_SBTE0_PRTN_CLK_EN_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_PRTN_CLK_EN_MASK: c_uint = 0x1;
// DCORE0_MME_SBTE0_CFG_SB_INFLIGHTS
pub const DCORE0_MME_SBTE0_CFG_SB_INFLIGHTS_W_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_CFG_SB_INFLIGHTS_W_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_MME_SBTE0_PROT
pub const DCORE0_MME_SBTE0_PROT_W_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_PROT_W_MASK: c_uint = 0x7;
// DCORE0_MME_SBTE0_INTR_MASK
pub const DCORE0_MME_SBTE0_INTR_MASK_W_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_INTR_MASK_W_MASK: c_uint = 0x1;
// DCORE0_MME_SBTE0_ARUSER_MSB
pub const DCORE0_MME_SBTE0_ARUSER_MSB_VAL_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_ARUSER_MSB_VAL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_MME_SBTE0_CFG_SB_OCCUPIENCY
pub const DCORE0_MME_SBTE0_CFG_SB_OCCUPIENCY_VAL_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_CFG_SB_OCCUPIENCY_VAL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_MME_SBTE0_ENABLE_CGATE
pub const DCORE0_MME_SBTE0_ENABLE_CGATE_TE_EN_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_ENABLE_CGATE_TE_EN_MASK: c_uint = 0x1;
pub const DCORE0_MME_SBTE0_ENABLE_CGATE_SB_EN_SHIFT: c_int = 4;
pub const DCORE0_MME_SBTE0_ENABLE_CGATE_SB_EN_MASK: c_uint = 0x10;
// DCORE0_MME_SBTE0_INTF_VLD_DBG
pub const DCORE0_MME_SBTE0_INTF_VLD_DBG_VLD_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_INTF_VLD_DBG_VLD_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_MME_SBTE0_INTF_RDY_DBG
pub const DCORE0_MME_SBTE0_INTF_RDY_DBG_RDY_SHIFT: c_int = 0;
pub const DCORE0_MME_SBTE0_INTF_RDY_DBG_RDY_MASK: c_uint = 0xFFFFFFFF;
