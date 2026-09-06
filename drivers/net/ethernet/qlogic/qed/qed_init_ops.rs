//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_init_ops.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

//
// qed_init_iro_array(): init iro_arr.
//
// @cdev: Qed dev pointer.
//
// Return: Void.
//
extern "C" {
    pub fn qed_init_iro_array(cdev: *mut qed_dev);
}
//
// qed_init_run(): Run the init-sequence.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @phase: Phase.
// @phase_id: Phase ID.
// @modes: Mode.
//
// Return: _qed_status_t
//
// qed_init_alloc(): Allocate RT array, Store 'values' ptrs.
//
// @p_hwfn: HW device data.
//
// Return: _qed_status_t.
//
extern "C" {
    pub fn qed_init_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_init_free(): Init HW function deallocate.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_init_free(p_hwfn: *mut qed_hwfn);
}
//
// qed_init_store_rt_reg(): Store a configuration value in the RT array.
//
// @p_hwfn: HW device data.
// @rt_offset: RT offset.
// @val: Val.
//
// Return: Void.
//

//
// qed_gtt_init(): Initialize GTT global windows and set admin window
// related params of GTT/PTT to default values.
//
// @p_hwfn: HW device data.
//
// Return Void.
//
extern "C" {
    pub fn qed_gtt_init(p_hwfn: *mut qed_hwfn);
}
