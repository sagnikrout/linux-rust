//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/avs/lnl.c
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
// Copyright(c) 2021-2025 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#[no_mangle]
pub unsafe extern "C" fn avs_lnl_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> c_int {
    int avs_lnl_core_stall(struct avs_dev *adev, u32 core_mask, bool stall)
    {
    struct hdac_bus *bus = &adev.base.core;
    struct hdac_ext_link *hlink;
    int ret;
    ret = avs_mtl_core_stall(adev, core_mask, stall);
// On unstall, route interrupts from the links to the DSP firmware.
    if (!ret && !stall)
    list_for_each_entry(hlink, &bus.hlink_list, list)
    snd_hdac_updatel(hlink.ml_addr, AZX_REG_ML_LCTL, AZX_ML_LCTL_OFLEN,
    AZX_ML_LCTL_OFLEN);
    return ret;
    }
