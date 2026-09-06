//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/pcie-iproc.h
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
// Copyright (C) 2014-2015 Broadcom Corporation
//
// enum iproc_pcie_type - iProc PCIe interface type
// @IPROC_PCIE_PAXB_BCMA: BCMA-based host controllers
// @IPROC_PCIE_PAXB:	  PAXB-based host controllers for
// NS, NSP, Cygnus, NS2, and Pegasus SOCs
// @IPROC_PCIE_PAXB_V2:   PAXB-based host controllers for Stingray SoCs
// @IPROC_PCIE_PAXC:	  PAXC-based host controllers
// @IPROC_PCIE_PAXC_V2:   PAXC-based host controllers (second generation)
//
// PAXB is the wrapper used in root complex that can be connected to an
// external endpoint device.
//
// PAXC is the wrapper used in root complex dedicated for internal emulated
// endpoint devices.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iproc_pcie_type {
    IPROC_PCIE_PAXB_BCMA = 0,
    IPROC_PCIE_PAXB,
    IPROC_PCIE_PAXB_V2,
    IPROC_PCIE_PAXC,
    IPROC_PCIE_PAXC_V2,
}

//
// struct iproc_pcie_ob - iProc PCIe outbound mapping
// @axi_offset: offset from the AXI address to the internal address used by
// the iProc PCIe core
// @nr_windows: total number of supported outbound mapping windows
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pcie_ob {
    pub axi_offset: resource_size_t,
    pub nr_windows: c_uint,
}

//
// struct iproc_pcie_ib - iProc PCIe inbound mapping
// @nr_regions: total number of supported inbound mapping regions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pcie_ib {
    pub nr_regions: c_uint,
}

//
// struct iproc_pcie - iProc PCIe device
// @dev: pointer to device data structure
// @type: iProc PCIe interface type
// @reg_offsets: register offsets
// @base: PCIe host controller I/O register base
// @base_addr: PCIe host controller register base physical address
// @mem: host bridge memory window resource
// @phy: optional PHY device that controls the Serdes
// @ep_is_internal: indicates an internal emulated endpoint device is connected
// @iproc_cfg_read: indicates the iProc config read function should be used
// @rej_unconfig_pf: indicates the root complex needs to detect and reject
// enumeration against unconfigured physical functions emulated in the ASIC
// @has_apb_err_disable: indicates the controller can be configured to prevent
// unsupported request from being forwarded as an APB bus error
// @fix_paxc_cap: indicates the controller has corrupted capability list in its
// config space registers and requires SW based fixup
//
// @need_ob_cfg: indicates SW needs to configure the outbound mapping window
// @ob: outbound mapping related parameters
// @ob_map: outbound mapping related parameters specific to the controller
//
// @need_ib_cfg: indicates SW needs to configure the inbound mapping window
// @ib: inbound mapping related parameters
// @ib_map: outbound mapping region related parameters
//
// @need_msi_steer: indicates additional configuration of the iProc PCIe
// controller is required to steer MSI writes to external interrupt controller
// @msi: MSI data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pcie {
    pub dev: *mut device,
    pub type: iproc_pcie_type,
    pub reg_offsets: *mut u16,
    pub base: *mut void __iomem,
    pub base_addr: phys_addr_t,
    pub mem: resource,
    pub phy: *mut phy,
    pub ep_is_internal: bool,
    pub iproc_cfg_read: bool,
    pub rej_unconfig_pf: bool,
    pub has_apb_err_disable: bool,
    pub fix_paxc_cap: bool,
    pub need_ob_cfg: bool,
    pub ob: iproc_pcie_ob,
    pub ob_map: *const iproc_pcie_ob_map,
    pub need_ib_cfg: bool,
    pub ib: iproc_pcie_ib,
    pub ib_map: *const iproc_pcie_ib_map,
    pub need_msi_steer: bool,
    pub msi: *mut iproc_msi,
}

extern "C" {
    pub fn iproc_pcie_setup(pcie: *mut iproc_pcie, res: *mut list_head) -> c_int;
}
extern "C" {
    pub fn iproc_pcie_remove(pcie: *mut iproc_pcie);
}
extern "C" {
    pub fn iproc_pcie_shutdown(pcie: *mut iproc_pcie) -> c_int;
}

extern "C" {
    pub fn iproc_msi_init(pcie: *mut iproc_pcie, node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn iproc_msi_exit(pcie: *mut iproc_pcie);
}

