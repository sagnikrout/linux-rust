//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mt6397/core.h
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
// Copyright (c) 2014 MediaTek Inc.
// Author: Flora Fu, MediaTek
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chip_id {
    MT6323_CHIP_ID = 0x23,
    MT6328_CHIP_ID = 0x28,
    MT6331_CHIP_ID = 0x31,
    MT6332_CHIP_ID = 0x32,
    MT6357_CHIP_ID = 0x57,
    MT6358_CHIP_ID = 0x58,
    MT6359_CHIP_ID = 0x59,
    MT6366_CHIP_ID = 0x66,
    MT6391_CHIP_ID = 0x91,
    MT6397_CHIP_ID = 0x97,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt6397_irq_numbers {
    MT6397_IRQ_SPKL_AB = 0,
    MT6397_IRQ_SPKR_AB,
    MT6397_IRQ_SPKL,
    MT6397_IRQ_SPKR,
    MT6397_IRQ_BAT_L,
    MT6397_IRQ_BAT_H,
    MT6397_IRQ_FG_BAT_L,
    MT6397_IRQ_FG_BAT_H,
    MT6397_IRQ_WATCHDOG,
    MT6397_IRQ_PWRKEY,
    MT6397_IRQ_THR_L,
    MT6397_IRQ_THR_H,
    MT6397_IRQ_VBATON_UNDET,
    MT6397_IRQ_BVALID_DET,
    MT6397_IRQ_CHRDET,
    MT6397_IRQ_OV,
    MT6397_IRQ_LDO,
    MT6397_IRQ_HOMEKEY,
    MT6397_IRQ_ACCDET,
    MT6397_IRQ_AUDIO,
    MT6397_IRQ_RTC,
    MT6397_IRQ_PWRKEY_RSTB,
    MT6397_IRQ_HDMI_SIFM,
    MT6397_IRQ_HDMI_CEC,
    MT6397_IRQ_VCA15,
    MT6397_IRQ_VSRMCA15,
    MT6397_IRQ_VCORE,
    MT6397_IRQ_VGPU,
    MT6397_IRQ_VIO18,
    MT6397_IRQ_VPCA7,
    MT6397_IRQ_VSRMCA7,
    MT6397_IRQ_VDRM,
    MT6397_IRQ_NR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6397_chip {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pm_nb: notifier_block,
    pub irq: c_int,
    pub irq_domain: *mut irq_domain,
    pub irqlock: mutex,
    pub wake_mask: [u16; 3],
    pub irq_masks_cur: [u16; 3],
    pub irq_masks_cache: [u16; 3],
    pub int_con: [u16; 3],
    pub int_status: [u16; 3],
    pub chip_id: u16,
    pub irq_data: *mut c_void,
}

extern "C" {
    pub fn mt6358_irq_init(chip: *mut mt6397_chip) -> c_int;
}
extern "C" {
    pub fn mt6397_irq_init(chip: *mut mt6397_chip) -> c_int;
}
