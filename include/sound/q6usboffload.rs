//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/q6usboffload.h
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
// sound/q6usboffload.h -- QDSP6 USB offload
//
// Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
//
// struct q6usb_offload - USB backend DAI link offload parameters
// @dev: dev handle to usb be
// @domain: allocated iommu domain
// @intr_num: usb interrupter number
// @sid: streamID for iommu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6usb_offload {
    pub dev: *mut device,
    pub domain: *mut iommu_domain,
    pub intr_num: u16,
    pub sid: u8,
}
