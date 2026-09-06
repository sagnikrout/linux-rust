//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mt6357/core.h
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
// Copyright (c) 2022 BayLibre, SAS
// Author: Fabien Parent <fparent@baylibre.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt6357_irq_top_status_shift {
    MT6357_BUCK_TOP = 0,
    MT6357_LDO_TOP,
    MT6357_PSC_TOP,
    MT6357_SCK_TOP,
    MT6357_BM_TOP,
    MT6357_HK_TOP,
    MT6357_XPP_TOP,
    MT6357_AUD_TOP,
    MT6357_MISC_TOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt6357_irq_numbers {
    MT6357_IRQ_VPROC_OC = 0,
    MT6357_IRQ_VCORE_OC,
    MT6357_IRQ_VMODEM_OC,
    MT6357_IRQ_VS1_OC,
    MT6357_IRQ_VPA_OC,
    MT6357_IRQ_VCORE_PREOC,
    MT6357_IRQ_VFE28_OC = 16,
    MT6357_IRQ_VXO22_OC,
    MT6357_IRQ_VRF18_OC,
    MT6357_IRQ_VRF12_OC,
    MT6357_IRQ_VEFUSE_OC,
    MT6357_IRQ_VCN33_OC,
    MT6357_IRQ_VCN28_OC,
    MT6357_IRQ_VCN18_OC,
    MT6357_IRQ_VCAMA_OC,
    MT6357_IRQ_VCAMD_OC,
    MT6357_IRQ_VCAMIO_OC,
    MT6357_IRQ_VLDO28_OC,
    MT6357_IRQ_VUSB33_OC,
    MT6357_IRQ_VAUX18_OC,
    MT6357_IRQ_VAUD28_OC,
    MT6357_IRQ_VIO28_OC,
    MT6357_IRQ_VIO18_OC,
    MT6357_IRQ_VSRAM_PROC_OC,
    MT6357_IRQ_VSRAM_OTHERS_OC,
    MT6357_IRQ_VIBR_OC,
    MT6357_IRQ_VDRAM_OC,
    MT6357_IRQ_VMC_OC,
    MT6357_IRQ_VMCH_OC,
    MT6357_IRQ_VEMC_OC,
    MT6357_IRQ_VSIM1_OC,
    MT6357_IRQ_VSIM2_OC,
    MT6357_IRQ_PWRKEY = 48,
    MT6357_IRQ_HOMEKEY,
    MT6357_IRQ_PWRKEY_R,
    MT6357_IRQ_HOMEKEY_R,
    MT6357_IRQ_NI_LBAT_INT,
    MT6357_IRQ_CHRDET,
    MT6357_IRQ_CHRDET_EDGE,
    MT6357_IRQ_VCDT_HV_DET,
    MT6357_IRQ_WATCHDOG,
    MT6357_IRQ_VBATON_UNDET,
    MT6357_IRQ_BVALID_DET,
    MT6357_IRQ_OV,
    MT6357_IRQ_RTC = 64,
    MT6357_IRQ_FG_BAT0_H = 80,
    MT6357_IRQ_FG_BAT0_L,
    MT6357_IRQ_FG_CUR_H,
    MT6357_IRQ_FG_CUR_L,
    MT6357_IRQ_FG_ZCV,
    MT6357_IRQ_BATON_LV = 96,
    MT6357_IRQ_BATON_HT,
    MT6357_IRQ_BAT_H = 112,
    MT6357_IRQ_BAT_L,
    MT6357_IRQ_AUXADC_IMP,
    MT6357_IRQ_NAG_C_DLTV,
    MT6357_IRQ_AUDIO = 128,
    MT6357_IRQ_ACCDET = 133,
    MT6357_IRQ_ACCDET_EINT0,
    MT6357_IRQ_ACCDET_EINT1,
    MT6357_IRQ_SPI_CMD_ALERT = 144,
    MT6357_IRQ_NR,
}

