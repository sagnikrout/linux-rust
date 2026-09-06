//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/avs/debug.h
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
// Copyright(c) 2024-2025 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

extern "C" {
    pub fn avs_dsp_op(_arg: adev, _arg: log_buffer_status, _arg: msg) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_apl_log_buffer_layout {
    pub read_ptr: u32,
    pub write_ptr: u32,
    pub buffer: [u8; ],
    pub __packed: },
    pub 8): static_assert(sizeof(struct avs_apl_log_buffer_layout) ==,

    pub name): *const *const int avs_register_probe_component(struct avs_dev adev, char,

    pub adev): *mut bool avs_logging_fw(struct avs_dev,
    pub len): *const *const *const void avs_dump_fw_log(struct avs_dev adev, void __iomem src, unsigned int,
    pub len): *const *const *const void avs_dump_fw_log_wakeup(struct avs_dev adev, void __iomem src, unsigned int,
    pub adev): *mut void avs_debugfs_init(struct avs_dev,
    pub adev): *mut void avs_debugfs_exit(struct avs_dev,

    pub -EOPNOTSUPP: return,
// Macro flag: #define AVS_SET_ENABLE_LOGS_OP(name)
    pub false: return,

