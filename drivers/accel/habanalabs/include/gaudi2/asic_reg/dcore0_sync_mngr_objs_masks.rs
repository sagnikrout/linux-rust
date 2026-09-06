//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_sync_mngr_objs_masks.h
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
// DCORE0_SYNC_MNGR_OBJS
// (Prototype: SOB_OBJS)
//
// DCORE0_SYNC_MNGR_OBJS_SOB_OBJ
pub const DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_VAL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_VAL_MASK: c_uint = 0x7FFF;
pub const DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_LONG_SOB_SHIFT: c_int = 24;
pub const DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_LONG_SOB_MASK: c_uint = 0x1000000;
pub const DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_TRACE_EVICT_SHIFT: c_int = 30;
pub const DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_TRACE_EVICT_MASK: c_uint = 0x40000000;
pub const DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_INC_SHIFT: c_int = 31;
pub const DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_INC_MASK: c_uint = 0x80000000;
// DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRL
pub const DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRL_ADDRL_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRL_ADDRL_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRH
pub const DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRH_ADDRH_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRH_ADDRH_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_OBJS_MON_PAY_DATA
pub const DCORE0_SYNC_MNGR_OBJS_MON_PAY_DATA_DATA_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_OBJS_MON_PAY_DATA_DATA_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_OBJS_MON_ARM
pub const DCORE0_SYNC_MNGR_OBJS_MON_ARM_SID_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_OBJS_MON_ARM_SID_MASK: c_uint = 0xFF;
pub const DCORE0_SYNC_MNGR_OBJS_MON_ARM_MASK_SHIFT: c_int = 8;
pub const DCORE0_SYNC_MNGR_OBJS_MON_ARM_MASK_MASK: c_uint = 0xFF00;
pub const DCORE0_SYNC_MNGR_OBJS_MON_ARM_SOP_SHIFT: c_int = 16;
pub const DCORE0_SYNC_MNGR_OBJS_MON_ARM_SOP_MASK: c_uint = 0x10000;
pub const DCORE0_SYNC_MNGR_OBJS_MON_ARM_SOD_SHIFT: c_int = 17;
pub const DCORE0_SYNC_MNGR_OBJS_MON_ARM_SOD_MASK: c_uint = 0xFFFE0000;
// DCORE0_SYNC_MNGR_OBJS_MON_CONFIG
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LONG_SOB_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LONG_SOB_MASK: c_uint = 0x1;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_CQ_EN_SHIFT: c_int = 4;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_CQ_EN_MASK: c_uint = 0x10;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_WR_NUM_SHIFT: c_int = 5;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_WR_NUM_MASK: c_uint = 0x60;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LBW_EN_SHIFT: c_int = 8;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LBW_EN_MASK: c_uint = 0x100;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_MSB_SID_SHIFT: c_int = 16;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_MSB_SID_MASK: c_uint = 0xF0000;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LONG_HIGH_GROUP_SHIFT: c_int = 31;
pub const DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LONG_HIGH_GROUP_MASK: c_uint = 0x80000000;
// DCORE0_SYNC_MNGR_OBJS_MON_STATUS
pub const DCORE0_SYNC_MNGR_OBJS_MON_STATUS_VALID_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_OBJS_MON_STATUS_VALID_MASK: c_uint = 0x1;
pub const DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PENDING_SHIFT: c_int = 1;
pub const DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PENDING_MASK: c_uint = 0x1FE;
pub const DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PROT_SHIFT: c_int = 9;
pub const DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PROT_MASK: c_uint = 0x200;
pub const DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PRIV_SHIFT: c_int = 10;
pub const DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PRIV_MASK: c_uint = 0x400;
// DCORE0_SYNC_MNGR_OBJS_SM_SEC
pub const DCORE0_SYNC_MNGR_OBJS_SM_SEC_SEC_VEC_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_OBJS_SM_SEC_SEC_VEC_MASK: c_uint = 0xFFFFFFFF;
// DCORE0_SYNC_MNGR_OBJS_SM_PRIV
pub const DCORE0_SYNC_MNGR_OBJS_SM_PRIV_PRIV_SHIFT: c_int = 0;
pub const DCORE0_SYNC_MNGR_OBJS_SM_PRIV_PRIV_MASK: c_uint = 0xFFFFFFFF;
