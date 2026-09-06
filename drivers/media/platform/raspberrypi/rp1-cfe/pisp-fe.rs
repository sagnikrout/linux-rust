//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/raspberrypi/rp1-cfe/pisp-fe.h
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
// PiSP Front End Driver
//
// Copyright (c) 2021-2024 Raspberry Pi Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pisp_fe_pads {
    FE_STREAM_PAD,
    FE_CONFIG_PAD,
    FE_OUTPUT0_PAD,
    FE_OUTPUT1_PAD,
    FE_STATS_PAD,
    FE_NUM_PADS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_fe_device {
// Parent V4l2 device
    pub v4l2_dev: *mut v4l2_device,
    pub base: *mut void __iomem,
    pub hw_revision: u32,
    pub inframe_count: u16,
    pub pad: [media_pad; FE_NUM_PADS],
    pub sd: v4l2_subdev,
}

extern "C" {
    pub fn pisp_fe_isr(fe: *mut pisp_fe_device, sof: *mut bool, eof: *mut bool);
}
extern "C" {
    pub fn pisp_fe_start(fe: *mut pisp_fe_device);
}
extern "C" {
    pub fn pisp_fe_stop(fe: *mut pisp_fe_device);
}
extern "C" {
    pub fn pisp_fe_init(fe: *mut pisp_fe_device, debugfs: *mut dentry) -> c_int;
}
extern "C" {
    pub fn pisp_fe_uninit(fe: *mut pisp_fe_device);
}
