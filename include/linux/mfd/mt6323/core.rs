//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mt6323/core.h
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
// Copyright (c) 2016 Chen Zhong <chen.zhong@mediatek.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MT6323_IRQ_STATUS_numbers {
    MT6323_IRQ_STATUS_SPKL_AB = 0,
    MT6323_IRQ_STATUS_SPKL,
    MT6323_IRQ_STATUS_BAT_L,
    MT6323_IRQ_STATUS_BAT_H,
    MT6323_IRQ_STATUS_WATCHDOG,
    MT6323_IRQ_STATUS_PWRKEY,
    MT6323_IRQ_STATUS_THR_L,
    MT6323_IRQ_STATUS_THR_H,
    MT6323_IRQ_STATUS_VBATON_UNDET,
    MT6323_IRQ_STATUS_BVALID_DET,
    MT6323_IRQ_STATUS_CHRDET,
    MT6323_IRQ_STATUS_OV,
    MT6323_IRQ_STATUS_LDO = 16,
    MT6323_IRQ_STATUS_FCHRKEY,
    MT6323_IRQ_STATUS_ACCDET,
    MT6323_IRQ_STATUS_AUDIO,
    MT6323_IRQ_STATUS_RTC,
    MT6323_IRQ_STATUS_VPROC,
    MT6323_IRQ_STATUS_VSYS,
    MT6323_IRQ_STATUS_VPA,
    MT6323_IRQ_STATUS_NR,
}
