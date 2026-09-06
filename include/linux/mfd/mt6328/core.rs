//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mt6328/core.h
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
// Copyright (c) 2015 MediaTek Inc.
// Copyright (c) 2022 Yassine Oudjana <y.oudjana@protonmail.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt6328_irq_status_numbers {
    MT6328_IRQ_STATUS_PWRKEY = 0,
    MT6328_IRQ_STATUS_HOMEKEY,
    MT6328_IRQ_STATUS_PWRKEY_R,
    MT6328_IRQ_STATUS_HOMEKEY_R,
    MT6328_IRQ_STATUS_THR_H,
    MT6328_IRQ_STATUS_THR_L,
    MT6328_IRQ_STATUS_BAT_H,
    MT6328_IRQ_STATUS_BAT_L,
    MT6328_IRQ_STATUS_RTC,
    MT6328_IRQ_STATUS_AUDIO,
    MT6328_IRQ_STATUS_ACCDET,
    MT6328_IRQ_STATUS_ACCDET_EINT,
    MT6328_IRQ_STATUS_ACCDET_NEGV,
    MT6328_IRQ_STATUS_NI_LBAT_INT,
    MT6328_IRQ_STATUS_VPROC_OC = 16,
    MT6328_IRQ_STATUS_VSYS_OC,
    MT6328_IRQ_STATUS_VLTE_OC,
    MT6328_IRQ_STATUS_VCORE_OC,
    MT6328_IRQ_STATUS_VPA_OC,
    MT6328_IRQ_STATUS_LDO_OC,
    MT6328_IRQ_STATUS_BAT2_H,
    MT6328_IRQ_STATUS_BAT2_L,
    MT6328_IRQ_STATUS_VISMPS0_H,
    MT6328_IRQ_STATUS_VISMPS0_L,
    MT6328_IRQ_STATUS_AUXADC_IMP,
    MT6328_IRQ_STATUS_OV = 32,
    MT6328_IRQ_STATUS_BVALID_DET,
    MT6328_IRQ_STATUS_VBATON_HV,
    MT6328_IRQ_STATUS_VBATON_UNDET,
    MT6328_IRQ_STATUS_WATCHDOG,
    MT6328_IRQ_STATUS_PCHR_CM_VDEC,
    MT6328_IRQ_STATUS_CHRDET,
    MT6328_IRQ_STATUS_PCHR_CM_VINC,
    MT6328_IRQ_STATUS_FG_BAT_H,
    MT6328_IRQ_STATUS_FG_BAT_L,
    MT6328_IRQ_STATUS_FG_CUR_H,
    MT6328_IRQ_STATUS_FG_CUR_L,
    MT6328_IRQ_STATUS_FG_ZCV,
    MT6328_IRQ_STATUS_SPKL_D,
    MT6328_IRQ_STATUS_SPKL_AB,
}
