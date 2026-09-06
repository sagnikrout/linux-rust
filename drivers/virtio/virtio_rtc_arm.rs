//! Automatically rewritten from C to Rust
//! Source: drivers/virtio/virtio_rtc_arm.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Provides cross-timestamp params for Arm.
//
// Copyright (C) 2022-2023 OpenSynergy GmbH
// Copyright (c) 2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

// see header for doc
#[no_mangle]
pub unsafe extern "C" fn viortc_hw_xtstamp_params(hw_counter: *mut u8, cs_id: *mut enum clocksource_ids) -> c_int {
    int viortc_hw_xtstamp_params(u8 *hw_counter, enum clocksource_ids *cs_id)
    {
// hw_counter = VIRTIO_RTC_COUNTER_ARM_VCT;
// cs_id = CSID_ARM_ARCH_COUNTER;
    return 0;
    }
