//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/media/mmp-camera.h
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
// Information for the Marvell Armada MMP camera
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dphy3_algo {
    DPHY3_ALGO_DEFAULT = 0,
    DPHY3_ALGO_PXA910,
    DPHY3_ALGO_PXA2128
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_camera_platform_data {
    pub bus_type: v4l2_mbus_type,
    pub /: *mut *mut int mclk_src; / which clock source the MCLK derives from,
    pub /: *mut *mut int mclk_div; / Clock Divider Value for MCLK,
//
// MIPI support
//
    pub /: *mut *mut int dphy[3]; / DPHY: CSI2_DPHY3, CSI2_DPHY5, CSI2_DPHY6,
    pub /: *mut *mut dphy3_algo dphy3_algo; / algos for calculate CSI2_DPHY3,
    pub /: *mut *mut int lane; / ccic used lane number; 0 means DVP mode,
    pub lane_clk: c_int,
}
