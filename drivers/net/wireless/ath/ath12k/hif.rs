//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/hif.h
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
// Copyright (c) 2019-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_hif_ops {
    pub address): *mut *mut *mut u32 (read32)(struct ath12k_base ab, u32,
    pub data): *mut *mut *mut void (write32)(struct ath12k_base ab, u32 address, u32,
    pub ab): *mut *mut void (irq_enable)(struct ath12k_base,
    pub ab): *mut *mut void (irq_disable)(struct ath12k_base,
    pub ab): *mut *mut int (start)(struct ath12k_base,
    pub ab): *mut *mut void (stop)(struct ath12k_base,
    pub ab): *mut *mut int (power_up)(struct ath12k_base,
    pub is_suspend): *mut *mut *mut void (power_down)(struct ath12k_base ab, bool,
    pub ab): *mut *mut int (suspend)(struct ath12k_base,
    pub ab): *mut *mut int (resume)(struct ath12k_base,
    pub dl_pipe): *mut *mut u8 ul_pipe, u8,
    pub base_vector): *mut u32,
    pub msi_addr_hi): *mut u32,
    pub ab): *mut *mut void (ce_irq_enable)(struct ath12k_base,
    pub ab): *mut *mut void (ce_irq_disable)(struct ath12k_base,
    pub msi_idx): *mut *mut *mut void (get_ce_msi_idx)(struct ath12k_base ab, u32 ce_id, u32,
    pub ab): *mut *mut int (panic_handler)(struct ath12k_base,
    pub ab): *mut *mut void (coredump_download)(struct ath12k_base,
}

// msi_data_idx = ce_id;
