//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-lite-reg.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012 Samsung Electronics Co., Ltd.
//

// Camera Source size
pub const FLITE_REG_CISRCSIZE: c_uint = 0x00;

// Global control
pub const FLITE_REG_CIGCTRL: c_uint = 0x04;

// User defined formats. x = 0...15

// Interrupts mask bits (1 disables an interrupt)

// Image Capture Enable
pub const FLITE_REG_CIIMGCPT: c_uint = 0x08;

// Capture Sequence
pub const FLITE_REG_CICPTSEQ: c_uint = 0x0c;
// Camera Window Offset
pub const FLITE_REG_CIWDOFST: c_uint = 0x10;

// Camera Window Offset2
pub const FLITE_REG_CIWDOFST2: c_uint = 0x14;
// Camera Output DMA Format
pub const FLITE_REG_CIODMAFMT: c_uint = 0x18;

// Camera Output Canvas
pub const FLITE_REG_CIOCAN: c_uint = 0x20;

// Camera Output DMA Offset
pub const FLITE_REG_CIOOFF: c_uint = 0x24;

// Camera Output DMA Start Address
pub const FLITE_REG_CIOSA: c_uint = 0x30;
// Camera Status
pub const FLITE_REG_CISTATUS: c_uint = 0x40;

// Camera Status2
pub const FLITE_REG_CISTATUS2: c_uint = 0x44;

// Qos Threshold
pub const FLITE_REG_CITHOLD: c_uint = 0xf0;

// Camera General Purpose
pub const FLITE_REG_CIGENERAL: c_uint = 0xfc;
// b0: 1 - camera B, 0 - camera A

pub const FLITE_REG_CIFCNTSEQ: c_uint = 0x100;

// ----------------------------------------------------------------------------
// Function declarations
//
extern "C" {
    pub fn flite_hw_reset(dev: *mut fimc_lite);
}
extern "C" {
    pub fn flite_hw_clear_pending_irq(dev: *mut fimc_lite);
}
extern "C" {
    pub fn flite_hw_get_interrupt_source(dev: *mut fimc_lite) -> u32;
}
extern "C" {
    pub fn flite_hw_clear_last_capture_end(dev: *mut fimc_lite);
}
extern "C" {
    pub fn flite_hw_set_interrupt_mask(dev: *mut fimc_lite);
}
extern "C" {
    pub fn flite_hw_capture_start(dev: *mut fimc_lite);
}
extern "C" {
    pub fn flite_hw_capture_stop(dev: *mut fimc_lite);
}
extern "C" {
    pub fn flite_hw_set_window_offset(dev: *mut fimc_lite, f: *const flite_frame);
}
extern "C" {
    pub fn flite_hw_set_source_format(dev: *mut fimc_lite, f: *const flite_frame);
}
extern "C" {
    pub fn flite_hw_set_dma_window(dev: *mut fimc_lite, f: *const flite_frame);
}
extern "C" {
    pub fn flite_hw_set_test_pattern(dev: *mut fimc_lite, on: bool);
}
extern "C" {
    pub fn flite_hw_dump_regs(dev: *mut fimc_lite, label: *const c_char);
}
extern "C" {
    pub fn flite_hw_set_dma_buffer(dev: *mut fimc_lite, buf: *mut flite_buffer);
}
extern "C" {
    pub fn flite_hw_mask_dma_buffer(dev: *mut fimc_lite, index: u32);
}
