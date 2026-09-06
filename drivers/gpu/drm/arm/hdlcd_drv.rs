//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/hdlcd_drv.h
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
// ARM HDLCD Controller register definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdlcd_drm_private {
    pub base: drm_device,
    pub mmio: *mut void __iomem,
    pub clk: *mut clk,
    pub crtc: drm_crtc,
    pub plane: *mut drm_plane,
    pub irq: c_uint,

    pub buffer_underrun_count: core::sync::atomic::AtomicI32,
    pub bus_error_count: core::sync::atomic::AtomicI32,
    pub vsync_count: core::sync::atomic::AtomicI32,
    pub dma_end_count: core::sync::atomic::AtomicI32,

}

extern "C" {
    pub fn readl(reg: hdlcd->mmio +) -> return;
}
extern "C" {
    pub fn hdlcd_setup_crtc(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn hdlcd_set_scanout(hdlcd: *mut hdlcd_drm_private);
}
