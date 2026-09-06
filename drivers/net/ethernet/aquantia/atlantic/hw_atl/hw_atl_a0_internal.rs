//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl/hw_atl_a0_internal.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// aQuantia Corporation Network Driver
// Copyright (C) 2014-2017 aQuantia Corporation. All rights reserved
//
// File hw_atl_a0_internal.h: Definition of Atlantic A0 chip specific
// constants.
//

// interrupts

pub const HW_ATL_A0_INT_MASK: c_uint = 0xFFFFFFFFU;
pub const HW_ATL_A0_TXD_CTL2_LEN: c_uint = 0xFFFFC000U;
pub const HW_ATL_A0_TXD_CTL2_CTX_EN: c_uint = 0x00002000U;
pub const HW_ATL_A0_TXD_CTL2_CTX_IDX: c_uint = 0x00001000U;
pub const HW_ATL_A0_TXD_CTL_DESC_TYPE_TXD: c_uint = 0x00000001U;
pub const HW_ATL_A0_TXD_CTL_DESC_TYPE_TXC: c_uint = 0x00000002U;
pub const HW_ATL_A0_TXD_CTL_BLEN: c_uint = 0x000FFFF0U;
pub const HW_ATL_A0_TXD_CTL_DD: c_uint = 0x00100000U;
pub const HW_ATL_A0_TXD_CTL_EOP: c_uint = 0x00200000U;
pub const HW_ATL_A0_TXD_CTL_CMD_X: c_uint = 0x3FC00000U;

pub const HW_ATL_A0_MPI_CONTROL_ADR: c_uint = 0x0368U;
pub const HW_ATL_A0_MPI_STATE_ADR: c_uint = 0x036CU;
pub const HW_ATL_A0_MPI_SPEED_MSK: c_uint = 0xFFFFU;

pub const HW_ATL_A0_FW_SEMA_RAM: c_uint = 0x2U;
pub const HW_ATL_A0_RXD_DD: c_uint = 0x1U;
pub const HW_ATL_A0_RXD_NCEA0: c_uint = 0x1U;
pub const HW_ATL_A0_RXD_WB_STAT2_EOP: c_uint = 0x0002U;
pub const HW_ATL_A0_UCP_0X370_REG: c_uint = 0x370U;
pub const HW_ATL_A0_FW_VER_EXPECTED: c_uint = 0x01050006U;

