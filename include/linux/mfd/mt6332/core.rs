//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mt6332/core.h
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
// Copyright (c) 2022 AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt6332_irq_status_numbers {
    MT6332_IRQ_STATUS_CHR_COMPLETE = 0,
    MT6332_IRQ_STATUS_THERMAL_SD,
    MT6332_IRQ_STATUS_THERMAL_REG_IN,
    MT6332_IRQ_STATUS_THERMAL_REG_OUT,
    MT6332_IRQ_STATUS_OTG_OC,
    MT6332_IRQ_STATUS_CHR_OC,
    MT6332_IRQ_STATUS_OTG_THERMAL,
    MT6332_IRQ_STATUS_CHRIN_SHORT,
    MT6332_IRQ_STATUS_DRVCDT_SHORT,
    MT6332_IRQ_STATUS_PLUG_IN_FLASH,
    MT6332_IRQ_STATUS_CHRWDT_FLAG,
    MT6332_IRQ_STATUS_FLASH_EN_TIMEOUT,
    MT6332_IRQ_STATUS_FLASH_VLED1_SHORT,
    MT6332_IRQ_STATUS_FLASH_VLED1_OPEN = 13,
    MT6332_IRQ_STATUS_OV = 16,
    MT6332_IRQ_STATUS_BVALID_DET,
    MT6332_IRQ_STATUS_VBATON_UNDET,
    MT6332_IRQ_STATUS_CHR_PLUG_IN,
    MT6332_IRQ_STATUS_CHR_PLUG_OUT,
    MT6332_IRQ_STATUS_BC11_TIMEOUT,
    MT6332_IRQ_STATUS_FLASH_VLED2_SHORT,
    MT6332_IRQ_STATUS_FLASH_VLED2_OPEN = 23,
    MT6332_IRQ_STATUS_THR_H = 32,
    MT6332_IRQ_STATUS_THR_L,
    MT6332_IRQ_STATUS_BAT_H,
    MT6332_IRQ_STATUS_BAT_L,
    MT6332_IRQ_STATUS_M3_H,
    MT6332_IRQ_STATUS_M3_L,
    MT6332_IRQ_STATUS_FG_BAT_H,
    MT6332_IRQ_STATUS_FG_BAT_L,
    MT6332_IRQ_STATUS_FG_CUR_H,
    MT6332_IRQ_STATUS_FG_CUR_L,
    MT6332_IRQ_STATUS_SPKL_D,
    MT6332_IRQ_STATUS_SPKL_AB,
    MT6332_IRQ_STATUS_BIF,
    MT6332_IRQ_STATUS_VWLED_OC = 45,
    MT6332_IRQ_STATUS_VDRAM_OC = 48,
    MT6332_IRQ_STATUS_VDVFS2_OC,
    MT6332_IRQ_STATUS_VRF1_OC,
    MT6332_IRQ_STATUS_VRF2_OC,
    MT6332_IRQ_STATUS_VPA_OC,
    MT6332_IRQ_STATUS_VSBST_OC,
    MT6332_IRQ_STATUS_LDO_OC,
    MT6332_IRQ_STATUS_NR,
}

