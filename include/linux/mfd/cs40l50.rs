//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/cs40l50.h
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
// CS40L50 Advanced Haptic Driver with waveform memory,
// integrated DSP, and closed-loop algorithms
//
// Copyright 2024 Cirrus Logic, Inc.
//
// Author: James Ogletree <james.ogletree@cirrus.com>
//

// Power Supply Configuration
pub const CS40L50_BLOCK_ENABLES2: c_uint = 0x201C;
pub const CS40L50_ERR_RLS: c_uint = 0x2034;
pub const CS40L50_BST_LPMODE_SEL: c_uint = 0x3810;
pub const CS40L50_DCM_LOW_POWER: c_uint = 0x1;
pub const CS40L50_OVERTEMP_WARN: c_uint = 0x4000010;
// Interrupts
pub const CS40L50_IRQ1_INT_1: c_uint = 0xE010;

pub const CS40L50_IRQ1_INT_2: c_uint = 0xE014;
pub const CS40L50_IRQ1_INT_8: c_uint = 0xE02C;
pub const CS40L50_IRQ1_INT_9: c_uint = 0xE030;
pub const CS40L50_IRQ1_INT_10: c_uint = 0xE034;
pub const CS40L50_IRQ1_INT_18: c_uint = 0xE054;
pub const CS40L50_IRQ1_MASK_1: c_uint = 0xE090;
pub const CS40L50_IRQ1_MASK_2: c_uint = 0xE094;
pub const CS40L50_IRQ1_MASK_20: c_uint = 0xE0DC;

pub const CS40L50_IRQ_MASK_2_OVERRIDE: c_uint = 0xFFDF7FFF;
pub const CS40L50_IRQ_MASK_20_OVERRIDE: c_uint = 0x15C01000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs40l50_irq_list {
    CS40L50_DSP_QUEUE_IRQ,
    CS40L50_GLOBAL_ERROR_IRQ,
    CS40L50_UVLO_VDDBATT_IRQ,
    CS40L50_BST_ILIMIT_IRQ,
    CS40L50_BST_SHORT_IRQ,
    CS40L50_BST_UVP_IRQ,
    CS40L50_TEMP_ERR_IRQ,
    CS40L50_AMP_SHORT_IRQ,
}

// DSP
pub const CS40L50_XMEM_PACKED_0: c_uint = 0x2000000;
pub const CS40L50_XMEM_UNPACKED24_0: c_uint = 0x2800000;
pub const CS40L50_SYS_INFO_ID: c_uint = 0x25E0000;
pub const CS40L50_DSP_QUEUE_WT: c_uint = 0x28042C8;
pub const CS40L50_DSP_QUEUE_RD: c_uint = 0x28042CC;
pub const CS40L50_NUM_WAVES: c_uint = 0x2805C18;
pub const CS40L50_CORE_BASE: c_uint = 0x2B80000;
pub const CS40L50_YMEM_PACKED_0: c_uint = 0x2C00000;
pub const CS40L50_YMEM_UNPACKED24_0: c_uint = 0x3400000;
pub const CS40L50_PMEM_0: c_uint = 0x3800000;
pub const CS40L50_DSP_POLL_US: c_int = 1000;
pub const CS40L50_DSP_TIMEOUT_COUNT: c_int = 100;
pub const CS40L50_RESET_PULSE_US: c_int = 2200;
pub const CS40L50_CP_READY_US: c_int = 3100;
pub const CS40L50_AUTOSUSPEND_MS: c_int = 2000;
pub const CS40L50_PM_ALGO: c_uint = 0x9F206;

pub const CS40L50_GLOBAL_ERR_RLS_CLEAR: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs40l50_wseqs {
    CS40L50_PWR_ON,
    CS40L50_STANDBY,
    CS40L50_ACTIVE,
    CS40L50_NUM_WSEQS,
}

// DSP Queue
pub const CS40L50_DSP_QUEUE_BASE: c_uint = 0x11004;
pub const CS40L50_DSP_QUEUE_END: c_uint = 0x1101C;
pub const CS40L50_DSP_QUEUE: c_uint = 0x11020;
pub const CS40L50_PREVENT_HIBER: c_uint = 0x2000003;
pub const CS40L50_ALLOW_HIBER: c_uint = 0x2000004;
pub const CS40L50_SHUTDOWN: c_uint = 0x2000005;
pub const CS40L50_SYSTEM_RESET: c_uint = 0x2000007;
pub const CS40L50_START_I2S: c_uint = 0x3000002;
pub const CS40L50_OWT_PUSH: c_uint = 0x3000008;
pub const CS40L50_STOP_PLAYBACK: c_uint = 0x5000000;
pub const CS40L50_OWT_DELETE: c_uint = 0xD000000;
// Firmware files

// Device
pub const CS40L50_DEVID: c_uint = 0x0;
pub const CS40L50_REVID: c_uint = 0x4;
pub const CS40L50_DEVID_A: c_uint = 0x40A50;
pub const CS40L50_REVID_B0: c_uint = 0xB0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs40l50 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub dsp: cs_dsp,
    pub reset_gpio: *mut gpio_desc,
    pub irq_data: *mut regmap_irq_chip_data,
    pub fw: *const firmware,
    pub bin: *const firmware,
    pub wseqs: [cs_dsp_wseq; CS40L50_NUM_WSEQS],
    pub irq: c_int,
    pub devid: u32,
    pub revid: u32,
}

extern "C" {
    pub fn cs40l50_dsp_write(dev: *mut device, regmap: *mut regmap, val: u32) -> c_int;
}
extern "C" {
    pub fn cs40l50_probe(cs40l50: *mut cs40l50) -> c_int;
}
extern "C" {
    pub fn cs40l50_remove(cs40l50: *mut cs40l50) -> c_int;
}
