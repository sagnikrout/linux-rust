//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/debug.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_debug_mask {
    ATH12K_DBG_AHB		= 0x00000001,
    ATH12K_DBG_WMI		= 0x00000002,
    ATH12K_DBG_HTC		= 0x00000004,
    ATH12K_DBG_DP_HTT	= 0x00000008,
    ATH12K_DBG_MAC		= 0x00000010,
    ATH12K_DBG_BOOT		= 0x00000020,
    ATH12K_DBG_QMI		= 0x00000040,
    ATH12K_DBG_DATA		= 0x00000080,
    ATH12K_DBG_MGMT		= 0x00000100,
    ATH12K_DBG_REG		= 0x00000200,
    ATH12K_DBG_TESTMODE	= 0x00000400,
    ATH12K_DBG_HAL		= 0x00000800,
    ATH12K_DBG_PCI		= 0x00001000,
    ATH12K_DBG_DP_TX	= 0x00002000,
    ATH12K_DBG_DP_RX	= 0x00004000,
    ATH12K_DBG_WOW		= 0x00008000,
    ATH12K_DBG_CE		= 0x00010000,
    ATH12K_DBG_ANY		= 0xffffffff,
}

