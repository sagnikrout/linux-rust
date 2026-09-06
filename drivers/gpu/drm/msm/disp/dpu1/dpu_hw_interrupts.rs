//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_interrupts.h
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
// Copyright (c) 2016-2018, The Linux Foundation. All rights reserved.
//

// When making changes be sure to sync with dpu_intr_set
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_hw_intr_reg {
    MDP_SSPP_TOP0_INTR,
    MDP_SSPP_TOP0_INTR2,
    MDP_SSPP_TOP0_HIST_INTR,
// All MDP_INTFn_INTR should come sequentially
    MDP_INTF0_INTR,
    MDP_INTF1_INTR,
    MDP_INTF2_INTR,
    MDP_INTF3_INTR,
    MDP_INTF4_INTR,
    MDP_INTF5_INTR,
    MDP_INTF6_INTR,
    MDP_INTF7_INTR,
    MDP_INTF8_INTR,
    MDP_INTF1_TEAR_INTR,
    MDP_INTF2_TEAR_INTR,
    MDP_AD4_0_INTR,
    MDP_AD4_1_INTR,
    MDP_INTR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_intr_entry {
    pub arg): *mut *mut void (cb)(void,
    pub arg: *mut c_void,
    pub count: core::sync::atomic::AtomicI32,
}

//
// struct dpu_hw_intr: hw interrupts handling data structure
// @hw:               virtual address mapping
// @ops:              function pointer mapping for IRQ handling
// @cache_irq_mask:   array of IRQ enable masks reg storage created during init
// @save_irq_status:  array of IRQ status reg storage created during init
// @irq_lock:         spinlock for accessing IRQ resources
// @irq_cb_tbl:       array of IRQ callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_hw_intr {
    pub hw: dpu_hw_blk_reg_map,
    pub cache_irq_mask: [u32; MDP_INTR_MAX],
    pub save_irq_status: *mut u32,
    pub irq_lock: spinlock_t,
    pub irq_mask: c_ulong,
    pub intr_set: *const dpu_intr_reg,
    pub irq_tbl: [dpu_hw_intr_entry; DPU_NUM_IRQS],
}
