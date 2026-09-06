//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/davinci/vpfe_types.h
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
// Copyright (C) 2008-2009 Texas Instruments Inc
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpfe_pin_pol {
    VPFE_PINPOL_POSITIVE,
    VPFE_PINPOL_NEGATIVE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpfe_hw_if_type {
// BT656 - 8 bit
    VPFE_BT656,
// BT1120 - 16 bit
    VPFE_BT1120,
// Raw Bayer
    VPFE_RAW_BAYER,
// YCbCr - 8 bit with external sync
    VPFE_YCBCR_SYNC_8,
// YCbCr - 16 bit with external sync
    VPFE_YCBCR_SYNC_16,
// BT656 - 10 bit
    VPFE_BT656_10BIT
}

// interface description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_hw_if_param {
    pub if_type: vpfe_hw_if_type,
    pub hdpol: vpfe_pin_pol,
    pub vdpol: vpfe_pin_pol,
}

