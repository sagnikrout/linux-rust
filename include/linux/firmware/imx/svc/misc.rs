//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/imx/svc/misc.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2016 Freescale Semiconductor, Inc.
// Copyright 2017~2018 NXP
//
// Header file containing the public API for the System Controller (SC)
// Miscellaneous (MISC) function.
//
// MISC_SVC (SVC) Miscellaneous Service
//
// Module for the Miscellaneous (MISC) service.
//

//
// This type is used to indicate RPC MISC function calls.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_misc_func {
    IMX_SC_MISC_FUNC_UNKNOWN = 0,
    IMX_SC_MISC_FUNC_SET_CONTROL = 1,
    IMX_SC_MISC_FUNC_GET_CONTROL = 2,
    IMX_SC_MISC_FUNC_SET_MAX_DMA_GROUP = 4,
    IMX_SC_MISC_FUNC_SET_DMA_GROUP = 5,
    IMX_SC_MISC_FUNC_SECO_IMAGE_LOAD = 8,
    IMX_SC_MISC_FUNC_SECO_AUTHENTICATE = 9,
    IMX_SC_MISC_FUNC_DEBUG_OUT = 10,
    IMX_SC_MISC_FUNC_WAVEFORM_CAPTURE = 6,
    IMX_SC_MISC_FUNC_BUILD_INFO = 15,
    IMX_SC_MISC_FUNC_UNIQUE_ID = 19,
    IMX_SC_MISC_FUNC_SET_ARI = 3,
    IMX_SC_MISC_FUNC_BOOT_STATUS = 7,
    IMX_SC_MISC_FUNC_BOOT_DONE = 14,
    IMX_SC_MISC_FUNC_OTP_FUSE_READ = 11,
    IMX_SC_MISC_FUNC_OTP_FUSE_WRITE = 17,
    IMX_SC_MISC_FUNC_SET_TEMP = 12,
    IMX_SC_MISC_FUNC_GET_TEMP = 13,
    IMX_SC_MISC_FUNC_GET_BOOT_DEV = 16,
    IMX_SC_MISC_FUNC_GET_BUTTON_STATUS = 18,
}

//
// Control Functions
//

