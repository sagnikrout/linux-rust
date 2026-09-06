//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/sti/hva/hva-hw.h
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
// Copyright (C) STMicroelectronics SA 2015
// Authors: Yannick Fertre <yannick.fertre@st.com>
// Hugues Fruchet <hugues.fruchet@st.com>
//

// HVA Versions
pub const HVA_VERSION_UNKNOWN: c_uint = 0x000;
pub const HVA_VERSION_V400: c_uint = 0x400;
// HVA command types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hva_hw_cmd_type {
// RESERVED = 0x00
// RESERVED = 0x01
    H264_ENC = 0x02,
// RESERVED = 0x03
// RESERVED = 0x04
// RESERVED = 0x05
// RESERVED = 0x06
// RESERVED = 0x07
    REMOVE_CLIENT = 0x08,
    FREEZE_CLIENT = 0x09,
    START_CLIENT = 0x0A,
    FREEZE_ALL = 0x0B,
    START_ALL = 0x0C,
    REMOVE_ALL = 0x0D
}

extern "C" {
    pub fn hva_hw_probe(pdev: *mut platform_device, hva: *mut hva_dev) -> c_int;
}
extern "C" {
    pub fn hva_hw_remove(hva: *mut hva_dev);
}
extern "C" {
    pub fn hva_hw_runtime_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn hva_hw_runtime_resume(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn hva_hw_dump_regs(hva: *mut hva_dev, s: *mut seq_file);
}

