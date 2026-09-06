//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/imagination/e5010-jpeg-enc-hw.h
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
// Imagination E5010 JPEG Encoder driver.
//
// Copyright (C) 2023 Texas Instruments Incorporated - https://www.ti.com
//
// Author: David Huang <d-huang@ti.com>
// Author: Devarsh Thakkar <devarsht@ti.com>
//

extern "C" {
    pub fn e5010_hw_enable_output_address_error_irq(core_offset: *mut void __iomem, enable: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_enable_picture_done_irq(core_offset: *mut void __iomem, enable: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_enable_auto_clock_gating(core_offset: *mut void __iomem, enable: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_enable_manual_clock_gating(core_offset: *mut void __iomem, enable: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_enable_crc_check(core_offset: *mut void __iomem, enable: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_input_source_to_memory(core_offset: *mut void __iomem, set: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_input_luma_addr(core_offset: *mut void __iomem, val: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_input_chroma_addr(core_offset: *mut void __iomem, val: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_output_base_addr(core_offset: *mut void __iomem, val: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_get_output_size(core_offset: *mut void __iomem) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_horizontal_size(core_offset: *mut void __iomem, val: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_vertical_size(core_offset: *mut void __iomem, val: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_luma_stride(core_offset: *mut void __iomem, bytesperline: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_chroma_stride(core_offset: *mut void __iomem, bytesperline: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_input_subsampling(core_offset: *mut void __iomem, val: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_chroma_order(core_offset: *mut void __iomem, val: u32) -> c_int;
}
extern "C" {
    pub fn e5010_hw_set_qpvalue(core_offset: *mut void __iomem, offset: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn e5010_reset(dev: *mut device, core_offset: *mut void __iomem, mmu_offset: *mut void __iomem);
}
extern "C" {
    pub fn e5010_hw_set_output_max_size(core_offset: *mut void __iomem, val: u32);
}
extern "C" {
    pub fn e5010_hw_clear_picture_done(core_offset: *mut void __iomem, clear: u32);
}
extern "C" {
    pub fn e5010_hw_encode_start(core_offset: *mut void __iomem, start: u32);
}
extern "C" {
    pub fn e5010_hw_clear_output_error(core_offset: *mut void __iomem, clear: u32);
}
extern "C" {
    pub fn e5010_hw_bypass_mmu(mmu_base: *mut void __iomem, enable: u32);
}
extern "C" {
    pub fn e5010_hw_pic_done_irq(core_base: *mut void __iomem) -> bool;
}
extern "C" {
    pub fn e5010_hw_output_address_irq(core_base: *mut void __iomem) -> bool;
}
