//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panfrost/panfrost_features.h
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
// (C) COPYRIGHT 2014-2018 ARM Limited. All rights reserved.
// Copyright 2019 Linaro, Ltd., Rob Herring <robh@kernel.org>

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panfrost_hw_feature {
    HW_FEATURE_JOBCHAIN_DISAMBIGUATION,
    HW_FEATURE_PWRON_DURING_PWROFF_TRANS,
    HW_FEATURE_XAFFINITY,
    HW_FEATURE_V4,
    HW_FEATURE_FLUSH_REDUCTION,
    HW_FEATURE_PROTECTED_MODE,
    HW_FEATURE_COHERENCY_REG,
    HW_FEATURE_PROTECTED_DEBUG_MODE,
    HW_FEATURE_AARCH64_MMU,
    HW_FEATURE_TLS_HASHING,
    HW_FEATURE_THREAD_GROUP_SPLIT,
    HW_FEATURE_IDVS_GROUP_SIZE,
    HW_FEATURE_CLEAN_ONLY_SAFE,
    HW_FEATURE_3BIT_EXT_RW_L2_MMU_CONFIG,
}

extern "C" {
    pub fn test_bit(_arg: feat, _arg: pfdev->features.hw_features) -> return;
}
