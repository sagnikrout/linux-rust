//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mt6358/core.h
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
// Copyright (c) 2020 MediaTek Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_top_t {
    pub hwirq_base: c_int,
    pub num_int_regs: c_uint,
    pub en_reg: c_uint,
    pub en_reg_shift: c_uint,
    pub sta_reg: c_uint,
    pub sta_reg_shift: c_uint,
    pub top_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic_irq_data {
    pub num_top: c_uint,
    pub num_pmic_irqs: c_uint,
    pub top_int_status_reg: c_ushort,
    pub enable_hwirq: *mut bool,
    pub cache_hwirq: *mut bool,
    pub pmic_ints: *const irq_top_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt6358_irq_top_status_shift {
    MT6358_BUCK_TOP = 0,
    MT6358_LDO_TOP,
    MT6358_PSC_TOP,
    MT6358_SCK_TOP,
    MT6358_BM_TOP,
    MT6358_HK_TOP,
    MT6358_AUD_TOP,
    MT6358_MISC_TOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt6358_irq_numbers {
    MT6358_IRQ_VPROC11_OC = 0,
    MT6358_IRQ_VPROC12_OC,
    MT6358_IRQ_VCORE_OC,
    MT6358_IRQ_VGPU_OC,
    MT6358_IRQ_VMODEM_OC,
    MT6358_IRQ_VDRAM1_OC,
    MT6358_IRQ_VS1_OC,
    MT6358_IRQ_VS2_OC,
    MT6358_IRQ_VPA_OC,
    MT6358_IRQ_VCORE_PREOC,
    MT6358_IRQ_VFE28_OC = 16,
    MT6358_IRQ_VXO22_OC,
    MT6358_IRQ_VRF18_OC,
    MT6358_IRQ_VRF12_OC,
    MT6358_IRQ_VEFUSE_OC,
    MT6358_IRQ_VCN33_OC,
    MT6358_IRQ_VCN28_OC,
    MT6358_IRQ_VCN18_OC,
    MT6358_IRQ_VCAMA1_OC,
    MT6358_IRQ_VCAMA2_OC,
    MT6358_IRQ_VCAMD_OC,
    MT6358_IRQ_VCAMIO_OC,
    MT6358_IRQ_VLDO28_OC,
    MT6358_IRQ_VA12_OC,
    MT6358_IRQ_VAUX18_OC,
    MT6358_IRQ_VAUD28_OC,
    MT6358_IRQ_VIO28_OC,
    MT6358_IRQ_VIO18_OC,
    MT6358_IRQ_VSRAM_PROC11_OC,
    MT6358_IRQ_VSRAM_PROC12_OC,
    MT6358_IRQ_VSRAM_OTHERS_OC,
    MT6358_IRQ_VSRAM_GPU_OC,
    MT6358_IRQ_VDRAM2_OC,
    MT6358_IRQ_VMC_OC,
    MT6358_IRQ_VMCH_OC,
    MT6358_IRQ_VEMC_OC,
    MT6358_IRQ_VSIM1_OC,
    MT6358_IRQ_VSIM2_OC,
    MT6358_IRQ_VIBR_OC,
    MT6358_IRQ_VUSB_OC,
    MT6358_IRQ_VBIF28_OC,
    MT6358_IRQ_PWRKEY = 48,
    MT6358_IRQ_HOMEKEY,
    MT6358_IRQ_PWRKEY_R,
    MT6358_IRQ_HOMEKEY_R,
    MT6358_IRQ_NI_LBAT_INT,
    MT6358_IRQ_CHRDET,
    MT6358_IRQ_CHRDET_EDGE,
    MT6358_IRQ_VCDT_HV_DET,
    MT6358_IRQ_RTC = 64,
    MT6358_IRQ_FG_BAT0_H = 80,
    MT6358_IRQ_FG_BAT0_L,
    MT6358_IRQ_FG_CUR_H,
    MT6358_IRQ_FG_CUR_L,
    MT6358_IRQ_FG_ZCV,
    MT6358_IRQ_FG_BAT1_H,
    MT6358_IRQ_FG_BAT1_L,
    MT6358_IRQ_FG_N_CHARGE_L,
    MT6358_IRQ_FG_IAVG_H,
    MT6358_IRQ_FG_IAVG_L,
    MT6358_IRQ_FG_TIME_H,
    MT6358_IRQ_FG_DISCHARGE,
    MT6358_IRQ_FG_CHARGE,
    MT6358_IRQ_BATON_LV = 96,
    MT6358_IRQ_BATON_HT,
    MT6358_IRQ_BATON_BAT_IN,
    MT6358_IRQ_BATON_BAT_OUT,
    MT6358_IRQ_BIF,
    MT6358_IRQ_BAT_H = 112,
    MT6358_IRQ_BAT_L,
    MT6358_IRQ_BAT2_H,
    MT6358_IRQ_BAT2_L,
    MT6358_IRQ_BAT_TEMP_H,
    MT6358_IRQ_BAT_TEMP_L,
    MT6358_IRQ_AUXADC_IMP,
    MT6358_IRQ_NAG_C_DLTV,
    MT6358_IRQ_AUDIO = 128,
    MT6358_IRQ_ACCDET = 133,
    MT6358_IRQ_ACCDET_EINT0,
    MT6358_IRQ_ACCDET_EINT1,
    MT6358_IRQ_SPI_CMD_ALERT = 144,
    MT6358_IRQ_NR,
}

