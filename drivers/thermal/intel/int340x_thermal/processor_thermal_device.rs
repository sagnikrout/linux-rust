//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/intel/int340x_thermal/processor_thermal_device.h
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
// processor_thermal_device.h
// Copyright (c) 2020, Intel Corporation.
//

pub const PCI_DEVICE_ID_INTEL_ADL_THERMAL: c_uint = 0x461d;
pub const PCI_DEVICE_ID_INTEL_ARL_S_THERMAL: c_uint = 0xAD03;
pub const PCI_DEVICE_ID_INTEL_BDW_THERMAL: c_uint = 0x1603;
pub const PCI_DEVICE_ID_INTEL_BSW_THERMAL: c_uint = 0x22DC;
pub const PCI_DEVICE_ID_INTEL_BXT0_THERMAL: c_uint = 0x0A8C;
pub const PCI_DEVICE_ID_INTEL_BXT1_THERMAL: c_uint = 0x1A8C;
pub const PCI_DEVICE_ID_INTEL_BXTX_THERMAL: c_uint = 0x4A8C;
pub const PCI_DEVICE_ID_INTEL_BXTP_THERMAL: c_uint = 0x5A8C;
pub const PCI_DEVICE_ID_INTEL_CNL_THERMAL: c_uint = 0x5a03;
pub const PCI_DEVICE_ID_INTEL_CFL_THERMAL: c_uint = 0x3E83;
pub const PCI_DEVICE_ID_INTEL_GLK_THERMAL: c_uint = 0x318C;
pub const PCI_DEVICE_ID_INTEL_HSB_THERMAL: c_uint = 0x0A03;
pub const PCI_DEVICE_ID_INTEL_ICL_THERMAL: c_uint = 0x8a03;
pub const PCI_DEVICE_ID_INTEL_JSL_THERMAL: c_uint = 0x4E03;
pub const PCI_DEVICE_ID_INTEL_LNLM_THERMAL: c_uint = 0x641D;
pub const PCI_DEVICE_ID_INTEL_MTLP_THERMAL: c_uint = 0x7D03;
pub const PCI_DEVICE_ID_INTEL_NVL_H_THERMAL: c_uint = 0xD703;
pub const PCI_DEVICE_ID_INTEL_NVL_S_THERMAL: c_uint = 0xAD03;
pub const PCI_DEVICE_ID_INTEL_RPL_THERMAL: c_uint = 0xA71D;
pub const PCI_DEVICE_ID_INTEL_SKL_THERMAL: c_uint = 0x1903;
pub const PCI_DEVICE_ID_INTEL_TGL_THERMAL: c_uint = 0x9A03;
pub const PCI_DEVICE_ID_INTEL_PTL_THERMAL: c_uint = 0xB01D;
pub const PCI_DEVICE_ID_INTEL_WCL_THERMAL: c_uint = 0xFD1D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_config {
    pub index: u32,
    pub min_uw: u32,
    pub max_uw: u32,
    pub tmin_us: u32,
    pub tmax_us: u32,
    pub step_uw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_thermal_device {
    pub dev: *mut device,
    pub adev: *mut acpi_device,
    pub power_limits: [power_config; 2],
    pub int340x_zone: *mut int34x_thermal_zone,
    pub soc_dts: *mut intel_soc_dts_sensors,
    pub mmio_feature_mask: u32,
    pub mmio_base: *mut void __iomem,
    pub priv_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_mmio_regs {
    pub reg_unit: u64,
    pub regs: [u64; RAPL_DOMAIN_MAX][RAPL_DOMAIN_REG_MAX],
    pub limits: [c_int; RAPL_DOMAIN_MAX],
}

pub const PROC_THERMAL_FEATURE_NONE: c_uint = 0x00;
pub const PROC_THERMAL_FEATURE_RAPL: c_uint = 0x01;
pub const PROC_THERMAL_FEATURE_FIVR: c_uint = 0x02;
pub const PROC_THERMAL_FEATURE_DVFS: c_uint = 0x04;
pub const PROC_THERMAL_FEATURE_WT_REQ: c_uint = 0x08;
pub const PROC_THERMAL_FEATURE_DLVR: c_uint = 0x10;
pub const PROC_THERMAL_FEATURE_WT_HINT: c_uint = 0x20;
pub const PROC_THERMAL_FEATURE_POWER_FLOOR: c_uint = 0x40;
pub const PROC_THERMAL_FEATURE_MSI_SUPPORT: c_uint = 0x80;
pub const PROC_THERMAL_FEATURE_PTC: c_uint = 0x100;
pub const PROC_THERMAL_FEATURE_SOC_POWER_SLIDER: c_uint = 0x200;

extern "C" {
    pub fn proc_thermal_rapl_add(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device) -> c_int;
}
extern "C" {
    pub fn proc_thermal_rapl_remove();
}

extern "C" {
    pub fn proc_thermal_rfim_add(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device) -> c_int;
}
extern "C" {
    pub fn proc_thermal_rfim_remove(pdev: *mut pci_dev);
}
extern "C" {
    pub fn proc_thermal_wt_req_add(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device) -> c_int;
}
extern "C" {
    pub fn proc_thermal_wt_req_remove(pdev: *mut pci_dev);
}
pub const MBOX_CMD_WORKLOAD_TYPE_READ: c_uint = 0x0E;
pub const MBOX_CMD_WORKLOAD_TYPE_WRITE: c_uint = 0x0F;
pub const MBOX_DATA_BIT_AC_DC: c_int = 30;
pub const MBOX_DATA_BIT_VALID: c_int = 31;
pub const SOC_WT_RES_INT_STATUS_OFFSET: c_uint = 0x5B18;

extern "C" {
    pub fn proc_thermal_read_power_floor_status(proc_priv: *mut proc_thermal_device) -> c_int;
}
extern "C" {
    pub fn proc_thermal_power_floor_set_state(proc_priv: *mut proc_thermal_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn proc_thermal_power_floor_get_state(proc_priv: *mut proc_thermal_device) -> bool;
}
extern "C" {
    pub fn proc_thermal_check_power_floor_intr(proc_priv: *mut proc_thermal_device) -> bool;
}
extern "C" {
    pub fn processor_thermal_send_mbox_read_cmd(pdev: *mut pci_dev, id: u16, resp: *mut u64) -> c_int;
}
extern "C" {
    pub fn processor_thermal_send_mbox_write_cmd(pdev: *mut pci_dev, id: u16, data: u32) -> c_int;
}
extern "C" {
    pub fn proc_thermal_add(dev: *mut device, priv: *mut proc_thermal_device) -> c_int;
}
extern "C" {
    pub fn proc_thermal_remove(proc_priv: *mut proc_thermal_device);
}
extern "C" {
    pub fn proc_thermal_wt_hint_add(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device) -> c_int;
}
extern "C" {
    pub fn proc_thermal_wt_hint_remove(pdev: *mut pci_dev);
}
extern "C" {
    pub fn proc_thermal_wt_intr_callback(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device);
}
extern "C" {
    pub fn proc_thermal_check_wt_intr(proc_priv: *mut proc_thermal_device) -> bool;
}
extern "C" {
    pub fn proc_thermal_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn proc_thermal_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn proc_thermal_mmio_remove(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device);
}
extern "C" {
    pub fn proc_thermal_ptc_add(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device) -> c_int;
}
extern "C" {
    pub fn proc_thermal_ptc_remove(pdev: *mut pci_dev);
}
extern "C" {
    pub fn proc_thermal_soc_power_slider_add(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device) -> c_int;
}
extern "C" {
    pub fn proc_thermal_soc_power_slider_suspend(proc_priv: *mut proc_thermal_device);
}
extern "C" {
    pub fn proc_thermal_soc_power_slider_resume(proc_priv: *mut proc_thermal_device);
}
