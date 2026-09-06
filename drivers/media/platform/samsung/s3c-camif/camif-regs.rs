//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s3c-camif/camif-regs.h
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
// Register definition file for s3c24xx/s3c64xx SoC CAMIF driver
//
// Copyright (C) 2012 Sylwester Nawrocki <sylvester.nawrocki@gmail.com>
// Copyright (C) 2012 Tomasz Figa <tomasz.figa@gmail.com>
//

//
// The id argument indicates the processing path:
// id = 0 - codec (FIMC C), 1 - preview (FIMC P).
//
// Camera input format
pub const S3C_CAMIF_REG_CISRCFMT: c_uint = 0x00;

// Window offset
pub const S3C_CAMIF_REG_CIWDOFST: c_uint = 0x04;

// #define  CIWDOFST_CLROVPRFIY			BIT(27)

// Window offset 2
pub const S3C_CAMIF_REG_CIWDOFST2: c_uint = 0x14;

// Global control
pub const S3C_CAMIF_REG_CIGCTRL: c_uint = 0x08;

// IRQ_CLR_C, IRQ_CLR_P

// Y DMA output frame start address. n = 0..3.

// Cb plane output DMA start address. n = 0..3. Only codec path.

// Cr plane output DMA start address. n = 0..3. Only codec path.

// CICOTRGFMT, CIPRTRGFMT - Target format

// Preview path only

// CICOCTRL, CIPRCTRL. Output DMA control.

// xBURSTn - 5-bits width

// CICOSCPRERATIO, CIPRSCPRERATIO. Pre-scaler control 1.

// CICOSCPREDST, CIPRSCPREDST. Pre-scaler control 2.

// CICOSCCTRL, CIPRSCCTRL. Main scaler control.

// s3c244x preview path only, s3c64xx both

// 0 - 16-bit RGB, 1 - 24-bit RGB

// s3c64xx

// CICOTAREA, CIPRTAREA. Target area for DMA (Hsize x Vsize).

pub const CITAREA_MASK: c_uint = 0xfffffff;
// Codec (id = 0) or preview (id = 1) path status.

// Image capture enable

// Frame control: 1 - one-shot, 0 - free run

// Capture sequence
pub const S3C_CAMIF_REG_CICPTSEQ: c_uint = 0xc4;
// Image effects

// Image effect: 1 - after scaler, 0 - before scaler

// MSCOY0SA, MSPRY0SA. Y/Cb/Cr frame start address for input DMA.

// MSCOY0END, MSCOY0END. Y/Cb/Cr frame end address for input DMA.

// MSPRYOFF, MSPRYOFF. Y/Cb/Cr offset. n: 0 - codec, 1 - preview.

// Real input DMA data size. n = 0 - codec, 1 - preview.

// Input DMA control. n = 0 - codec, 1 - preview

// 0 - camera, 1 - DMA

// CICOSCOSY, CIPRSCOSY. Scan line Y/Cb/Cr offset.

// ------------------------------------------------------------------
extern "C" {
    pub fn camif_hw_reset(camif: *mut camif_dev);
}
extern "C" {
    pub fn camif_hw_clear_pending_irq(vp: *mut camif_vp);
}
extern "C" {
    pub fn camif_hw_clear_fifo_overflow(vp: *mut camif_vp);
}
extern "C" {
    pub fn camif_hw_set_lastirq(vp: *mut camif_vp, enable: c_int);
}
extern "C" {
    pub fn camif_hw_set_input_path(vp: *mut camif_vp);
}
extern "C" {
    pub fn camif_hw_enable_scaler(vp: *mut camif_vp, on: bool);
}
extern "C" {
    pub fn camif_hw_enable_capture(vp: *mut camif_vp);
}
extern "C" {
    pub fn camif_hw_disable_capture(vp: *mut camif_vp);
}
extern "C" {
    pub fn camif_hw_set_camera_bus(camif: *mut camif_dev);
}
extern "C" {
    pub fn camif_hw_set_source_format(camif: *mut camif_dev);
}
extern "C" {
    pub fn camif_hw_set_camera_crop(camif: *mut camif_dev);
}
extern "C" {
    pub fn camif_hw_set_scaler(vp: *mut camif_vp);
}
extern "C" {
    pub fn camif_hw_set_flip(vp: *mut camif_vp);
}
extern "C" {
    pub fn camif_hw_set_output_dma(vp: *mut camif_vp);
}
extern "C" {
    pub fn camif_hw_set_target_format(vp: *mut camif_vp);
}
extern "C" {
    pub fn camif_hw_set_test_pattern(camif: *mut camif_dev, pattern: c_uint);
}
extern "C" {
    pub fn camif_hw_dump_regs(camif: *mut camif_dev, label: *const c_char);
}
