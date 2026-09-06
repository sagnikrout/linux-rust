//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/pci.h
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
// Copyright (c) 2019-2021 The Linux Foundation. All rights reserved.
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
pub const QCN9274_QFPROM_RAW_RFA_PDET_ROW13_LSB: c_uint = 0x1E20338;

pub const PCI_BAR_WINDOW0_BASE: c_uint = 0x1E00000;
pub const PCI_BAR_WINDOW0_END: c_uint = 0x1E7FFFC;
pub const PCI_SOC_RANGE_MASK: c_uint = 0x3FFF;
pub const PCI_SOC_PCI_REG_BASE: c_uint = 0x1E04000;
pub const PCI_SOC_PCI_REG_END: c_uint = 0x1E07FFC;
pub const PCI_PARF_BASE: c_uint = 0x1E08000;
pub const PCI_PARF_END: c_uint = 0x1E0BFFC;
pub const PCI_MHIREGLEN_REG: c_uint = 0x1E0E100;
pub const PCI_MHI_REGION_END: c_uint = 0x1E0EFFC;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_msi_user {
    pub name: *const c_char,
    pub num_vectors: c_int,
    pub base_vector: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_msi_config {
    pub total_vectors: c_int,
    pub total_users: c_int,
    pub users: *const ath12k_msi_user,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_pci_flags {
    ATH12K_PCI_FLAG_INIT_DONE,
    ATH12K_PCI_FLAG_IS_MSI_64,
    ATH12K_PCI_ASPM_RESTORE,
    ATH12K_PCI_FLAG_MULTI_MSI_VECTORS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pci_ops {
    pub ab): *mut *mut int (wakeup)(struct ath12k_base,
    pub ab): *mut *mut void (release)(struct ath12k_base,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pci_device_family_ops {
    pub pci_dev): *const *const *const int (probe)(struct pci_dev pdev, struct pci_device_id,
    pub ab): *mut *mut int (arch_init)(struct ath12k_base,
    pub ab): *mut *mut void (arch_deinit)(struct ath12k_base,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pci_reg_base {
    pub umac_base: u32,
    pub ce_reg_base: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pci {
    pub pdev: *mut pci_dev,
    pub ab: *mut ath12k_base,
    pub dev_id: u16,
    pub amss_path: [c_char; 100],
    pub msi_ep_base_data: u32,
    pub mhi_ctrl: *mut mhi_controller,
    pub msi_config: *const ath12k_msi_config,
    pub mhi_state: c_ulong,
    pub mhi_pre_cb: mhi_callback,
    pub register_window: u32,
// protects register_window above
    pub window_lock: spinlock_t,
// enum ath12k_pci_flags
    pub flags: c_ulong,
    pub link_ctl: u16,
    pub irq_flags: c_ulong,
    pub pci_ops: *const ath12k_pci_ops,
    pub qmi_instance: u32,
    pub dma_mask: u64,
    pub device_family_ops: *const ath12k_pci_device_family_ops,
    pub reg_base: *const ath12k_pci_reg_base,
    pub window_reg_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub ops: ath12k_pci_device_family_ops,
    pub driver: pci_driver,
    pub reg_base: *const ath12k_pci_reg_base,
}

extern "C" {
    pub fn ath12k_pci_get_msi_irq(dev: *mut device, vector: c_uint) -> c_int;
}
extern "C" {
    pub fn ath12k_pci_write32(ab: *mut ath12k_base, offset: u32, value: u32);
}
extern "C" {
    pub fn ath12k_pci_read32(ab: *mut ath12k_base, offset: u32) -> u32;
}
extern "C" {
    pub fn ath12k_pci_hif_ce_irq_enable(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_pci_hif_ce_irq_disable(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_pci_ext_irq_enable(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_pci_ext_irq_disable(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_pci_hif_suspend(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_pci_hif_resume(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_pci_stop(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_pci_start(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_pci_power_up(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_pci_power_down(ab: *mut ath12k_base, is_suspend: bool);
}
extern "C" {
    pub fn ath12k_pci_unregister_driver(device_id: ath12k_device_family);
}
