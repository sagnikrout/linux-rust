//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/pci.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2019-2020 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const PCIE_SOC_GLOBAL_RESET: c_uint = 0x3008;
pub const PCIE_SOC_GLOBAL_RESET_V: c_int = 1;
pub const WLAON_WARM_SW_ENTRY: c_uint = 0x1f80504;
pub const WLAON_SOC_RESET_CAUSE_REG: c_uint = 0x01f8060c;
pub const PCIE_Q6_COOKIE_ADDR: c_uint = 0x01f80500;
pub const PCIE_Q6_COOKIE_DATA: c_uint = 0xc0000000;
// register to wake the UMAC from power collapse
pub const PCIE_SCRATCH_0_SOC_PCIE_REG: c_uint = 0x4040;
// register used for handshake mechanism to validate UMAC is awake
pub const PCIE_SOC_WAKE_PCIE_LOCAL_REG: c_uint = 0x3004;
pub const PCIE_PCIE_PARF_LTSSM: c_uint = 0x1e081b0;
pub const PARM_LTSSM_VALUE: c_uint = 0x111;
pub const GCC_GCC_PCIE_HOT_RST: c_uint = 0x1e402bc;
pub const GCC_GCC_PCIE_HOT_RST_VAL: c_uint = 0x10;
pub const PCIE_PCIE_INT_ALL_CLEAR: c_uint = 0x1e08228;
pub const PCIE_SMLH_REQ_RST_LINK_DOWN: c_uint = 0x2;
pub const PCIE_INT_CLEAR_ALL: c_uint = 0xffffffff;

pub const PCIE_QSERDES_COM_SYSCLK_EN_SEL_VAL: c_uint = 0x10;
pub const PCIE_QSERDES_COM_SYSCLK_EN_SEL_MSK: c_uint = 0xffffffff;

pub const PCIE_PCS_OSC_DTCT_CONFIG1_VAL: c_uint = 0x02;

pub const PCIE_PCS_OSC_DTCT_CONFIG2_VAL: c_uint = 0x52;

pub const PCIE_PCS_OSC_DTCT_CONFIG4_VAL: c_uint = 0xff;
pub const PCIE_PCS_OSC_DTCT_CONFIG_MSK: c_uint = 0x000000ff;
pub const WLAON_QFPROM_PWR_CTRL_REG: c_uint = 0x01f8031c;
pub const QFPROM_PWR_CTRL_VDD4BLOW_MASK: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_pci_flags {
    ATH11K_PCI_ASPM_RESTORE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_pci {
    pub pdev: *mut pci_dev,
    pub ab: *mut ath11k_base,
    pub dev_id: u16,
    pub amss_path: [c_char; 100],
    pub mhi_ctrl: *mut mhi_controller,
    pub msi_config: *const ath11k_msi_config,
    pub mhi_pre_cb: mhi_callback,
    pub register_window: u32,
// protects register_window above
    pub window_lock: spinlock_t,
// enum ath11k_pci_flags
    pub flags: c_ulong,
    pub link_ctl: u16,
    pub dma_mask: u64,
}

extern "C" {
    pub fn ath11k_pci_get_msi_irq(ab: *mut ath11k_base, vector: c_uint) -> c_int;
}
