//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mt6359/core.h
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
// Copyright (c) 2021 MediaTek Inc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt6359_irq_top_status_shift {
    MT6359_BUCK_TOP = 0,
    MT6359_LDO_TOP,
    MT6359_PSC_TOP,
    MT6359_SCK_TOP,
    MT6359_BM_TOP,
    MT6359_HK_TOP,
    MT6359_AUD_TOP = 7,
    MT6359_MISC_TOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt6359_irq_numbers {
    MT6359_IRQ_VCORE_OC = 1,
    MT6359_IRQ_VGPU11_OC,
    MT6359_IRQ_VGPU12_OC,
    MT6359_IRQ_VMODEM_OC,
    MT6359_IRQ_VPROC1_OC,
    MT6359_IRQ_VPROC2_OC,
    MT6359_IRQ_VS1_OC,
    MT6359_IRQ_VS2_OC,
    MT6359_IRQ_VPA_OC = 9,
    MT6359_IRQ_VFE28_OC = 16,
    MT6359_IRQ_VXO22_OC,
    MT6359_IRQ_VRF18_OC,
    MT6359_IRQ_VRF12_OC,
    MT6359_IRQ_VEFUSE_OC,
    MT6359_IRQ_VCN33_1_OC,
    MT6359_IRQ_VCN33_2_OC,
    MT6359_IRQ_VCN13_OC,
    MT6359_IRQ_VCN18_OC,
    MT6359_IRQ_VA09_OC,
    MT6359_IRQ_VCAMIO_OC,
    MT6359_IRQ_VA12_OC,
    MT6359_IRQ_VAUX18_OC,
    MT6359_IRQ_VAUD18_OC,
    MT6359_IRQ_VIO18_OC,
    MT6359_IRQ_VSRAM_PROC1_OC,
    MT6359_IRQ_VSRAM_PROC2_OC,
    MT6359_IRQ_VSRAM_OTHERS_OC,
    MT6359_IRQ_VSRAM_MD_OC,
    MT6359_IRQ_VEMC_OC,
    MT6359_IRQ_VSIM1_OC,
    MT6359_IRQ_VSIM2_OC,
    MT6359_IRQ_VUSB_OC,
    MT6359_IRQ_VRFCK_OC,
    MT6359_IRQ_VBBCK_OC,
    MT6359_IRQ_VBIF28_OC,
    MT6359_IRQ_VIBR_OC,
    MT6359_IRQ_VIO28_OC,
    MT6359_IRQ_VM18_OC,
    MT6359_IRQ_VUFS_OC = 45,
    MT6359_IRQ_PWRKEY = 48,
    MT6359_IRQ_HOMEKEY,
    MT6359_IRQ_PWRKEY_R,
    MT6359_IRQ_HOMEKEY_R,
    MT6359_IRQ_NI_LBAT_INT,
    MT6359_IRQ_CHRDET_EDGE = 53,
    MT6359_IRQ_RTC = 64,
    MT6359_IRQ_FG_BAT_H = 80,
    MT6359_IRQ_FG_BAT_L,
    MT6359_IRQ_FG_CUR_H,
    MT6359_IRQ_FG_CUR_L,
    MT6359_IRQ_FG_ZCV = 84,
    MT6359_IRQ_FG_N_CHARGE_L = 87,
    MT6359_IRQ_FG_IAVG_H,
    MT6359_IRQ_FG_IAVG_L = 89,
    MT6359_IRQ_FG_DISCHARGE = 91,
    MT6359_IRQ_FG_CHARGE,
    MT6359_IRQ_BATON_LV = 96,
    MT6359_IRQ_BATON_BAT_IN = 98,
    MT6359_IRQ_BATON_BAT_OU,
    MT6359_IRQ_BIF = 100,
    MT6359_IRQ_BAT_H = 112,
    MT6359_IRQ_BAT_L,
    MT6359_IRQ_BAT2_H,
    MT6359_IRQ_BAT2_L,
    MT6359_IRQ_BAT_TEMP_H,
    MT6359_IRQ_BAT_TEMP_L,
    MT6359_IRQ_THR_H,
    MT6359_IRQ_THR_L,
    MT6359_IRQ_AUXADC_IMP,
    MT6359_IRQ_NAG_C_DLTV = 121,
    MT6359_IRQ_AUDIO = 128,
    MT6359_IRQ_ACCDET = 133,
    MT6359_IRQ_ACCDET_EINT0,
    MT6359_IRQ_ACCDET_EINT1,
    MT6359_IRQ_SPI_CMD_ALERT = 144,
    MT6359_IRQ_NR,
}

