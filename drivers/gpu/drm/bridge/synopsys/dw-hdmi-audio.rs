//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/synopsys/dw-hdmi-audio.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdmi_audio_data {
    pub phys: phys_addr_t,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub hdmi: *mut dw_hdmi,
    pub hdmi): *mut *mut *mut u8 (get_eld)(struct dw_hdmi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdmi_i2s_audio_data {
    pub hdmi: *mut dw_hdmi,
    pub offset): *mut *mut *mut void (write)(struct dw_hdmi hdmi, u8 val, int,
    pub offset): *mut *mut *mut u8 (read)(struct dw_hdmi hdmi, int,
    pub hdmi): *mut *mut *mut u8 (get_eld)(struct dw_hdmi,
}
