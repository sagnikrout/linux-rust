//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/host/ufs-mediatek.h
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
// Copyright (C) 2019 MediaTek Inc.
//

//
// MCQ define and struct
//
pub const UFSHCD_MAX_Q_NR: c_int = 8;
pub const MTK_MCQ_INVALID_IRQ: c_uint = 0xFFFF;
// REG_UFS_MMIO_OPT_CTRL_0 160h

//
// Vendor specific UFSHCI Registers
//
pub const REG_UFS_XOUFS_CTRL: c_uint = 0x140;
pub const REG_UFS_REFCLK_CTRL: c_uint = 0x144;
pub const REG_UFS_UFS_MMIO_OTSD_CTRL: c_uint = 0x14C;
pub const REG_UFS_MMIO_OPT_CTRL_0: c_uint = 0x160;
pub const REG_UFS_EXTREG: c_uint = 0x2100;
pub const REG_UFS_MPHYCTRL: c_uint = 0x2200;
pub const REG_UFS_MTK_IP_VER: c_uint = 0x2240;
pub const REG_UFS_REJECT_MON: c_uint = 0x22AC;
pub const REG_UFS_DEBUG_SEL: c_uint = 0x22C0;
pub const REG_UFS_PROBE: c_uint = 0x22C8;
pub const REG_UFS_DEBUG_SEL_B0: c_uint = 0x22D0;
pub const REG_UFS_DEBUG_SEL_B1: c_uint = 0x22D4;
pub const REG_UFS_DEBUG_SEL_B2: c_uint = 0x22D8;
pub const REG_UFS_DEBUG_SEL_B3: c_uint = 0x22DC;
pub const REG_UFS_MTK_SQD: c_uint = 0x2800;
pub const REG_UFS_MTK_SQIS: c_uint = 0x2814;
pub const REG_UFS_MTK_CQD: c_uint = 0x281C;
pub const REG_UFS_MTK_CQIS: c_uint = 0x2824;
pub const REG_UFS_MCQ_STRIDE: c_uint = 0x30;
//
// Ref-clk control
//
// Values for register REG_UFS_REFCLK_CTRL
//
pub const REFCLK_RELEASE: c_uint = 0x0;

pub const REFCLK_REQ_TIMEOUT_US: c_int = 3000;
pub const REFCLK_DEFAULT_WAIT_US: c_int = 32;
//
// Other attributes
//
pub const VS_DEBUGCLOCKENABLE: c_uint = 0xD0A1;
pub const VS_SAVEPOWERCONTROL: c_uint = 0xD0A6;
pub const VS_UNIPROPOWERDOWNCONTROL: c_uint = 0xD0A8;
//
// Vendor specific link state
//
// Vendor specific host controller state
//
// VS_DEBUGCLOCKENABLE
//
// VS_SAVEPOWERCONTROL
//
// Host capability
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_mtk_host_caps {
    UFS_MTK_CAP_BOOST_CRYPT_ENGINE         = 1 << 0,
    UFS_MTK_CAP_VA09_PWR_CTRL              = 1 << 1,
    UFS_MTK_CAP_DISABLE_AH8                = 1 << 2,
    UFS_MTK_CAP_BROKEN_VCC                 = 1 << 3,

//
// Override UFS_MTK_CAP_BROKEN_VCC's behavior to
// allow vccqx upstream to enter LPM
//
    UFS_MTK_CAP_ALLOW_VCCQX_LPM            = 1 << 5,
    UFS_MTK_CAP_PMC_VIA_FASTAUTO           = 1 << 6,
    UFS_MTK_CAP_TX_SKEW_FIX                = 1 << 7,
    UFS_MTK_CAP_DISABLE_MCQ                = 1 << 8,
// Control MTCMOS with RTFF
    UFS_MTK_CAP_RTFF_MTCMOS                = 1 << 9,

    UFS_MTK_CAP_MCQ_BROKEN_RTC             = 1 << 10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_mtk_crypt_cfg {
    pub reg_vcore: *mut regulator,
    pub clk_crypt_perf: *mut clk,
    pub clk_crypt_mux: *mut clk,
    pub clk_crypt_lp: *mut clk,
    pub vcore_volt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_mtk_clk {
    pub /: *mut *mut *mut ufs_clk_info ufs_sel_clki; / Mux,
    pub /: *mut *mut *mut ufs_clk_info ufs_sel_max_clki; / Max src,
    pub /: *mut *mut *mut ufs_clk_info ufs_sel_min_clki; / Min src,
    pub /: *mut *mut *mut ufs_clk_info ufs_fde_clki; / Mux,
    pub /: *mut *mut *mut ufs_clk_info ufs_fde_max_clki; / Max src,
    pub /: *mut *mut *mut ufs_clk_info ufs_fde_min_clki; / Min src,
    pub reg_vcore: *mut regulator,
    pub vcore_volt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_mtk_hw_ver {
    pub step: u8,
    pub minor: u8,
    pub major: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_mtk_mcq_intr_info {
    pub hba: *mut ufs_hba,
    pub irq: u32,
    pub qid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_mtk_host {
    pub mphy: *mut phy,
    pub reg_va09: *mut regulator,
    pub hci_reset: *mut reset_control,
    pub unipro_reset: *mut reset_control,
    pub crypto_reset: *mut reset_control,
    pub mphy_reset: *mut reset_control,
    pub hba: *mut ufs_hba,
    pub crypt: *mut ufs_mtk_crypt_cfg,
    pub mclk: ufs_mtk_clk,
    pub hw_ver: ufs_mtk_hw_ver,
    pub caps: ufs_mtk_host_caps,
    pub mphy_powered_on: bool,
    pub unipro_lpm: bool,
    pub ref_clk_enabled: bool,
    pub clk_scale_up: bool,
    pub ref_clk_ungating_wait_us: u16,
    pub ref_clk_gating_wait_us: u16,
    pub ip_ver: u32,
    pub legacy_ip_ver: bool,
    pub mcq_set_intr: bool,
    pub is_mcq_intr_enabled: bool,
    pub mcq_nr_intr: c_int,
    pub mcq_intr_info: [ufs_mtk_mcq_intr_info; UFSHCD_MAX_Q_NR],
    pub phy_dev: *mut device,
}

// MTK delay of autosuspend: 500 ms
pub const MTK_RPM_AUTOSUSPEND_DELAY_MS: c_int = 500;
// MTK RTT support number for platforms before MT6995 B0
pub const MTK_MAX_NUM_RTT_LEGACY: c_int = 2;
// UFSHCI MTK ip version value
// UFSHCI 3.1
// UFSHCI 4.0
// UFSHCI 5.0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip_ver_legacy {
    IP_LEGACY_VER_MT6781 = 0x10380000,
    IP_LEGACY_VER_MT6879 = 0x10360000,
    IP_LEGACY_VER_MT6893 = 0x20160706
}
