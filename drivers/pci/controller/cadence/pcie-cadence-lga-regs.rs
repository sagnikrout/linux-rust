//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/cadence/pcie-cadence-lga-regs.h
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
// Cadence PCIe controller driver.
//
// Copyright (c) 2017 Cadence
// Author: Cyrille Pitchen <cyrille.pitchen@free-electrons.com>
//

// Local Management Registers
pub const CDNS_PCIE_LM_BASE: c_uint = 0x00100000;
// Vendor ID Register

pub const CDNS_PCIE_LM_ID_VENDOR_SHIFT: c_int = 0;

pub const CDNS_PCIE_LM_ID_SUBSYS_SHIFT: c_int = 16;

// Root Port Requester ID Register

pub const CDNS_PCIE_LM_RP_RID_SHIFT: c_int = 0;

// Endpoint Bus and Device Number Register

pub const CDNS_PCIE_LM_EP_ID_DEV_SHIFT: c_int = 0;

pub const CDNS_PCIE_LM_EP_ID_BUS_SHIFT: c_int = 8;
// Endpoint Function f BAR b Configuration Registers

// Endpoint Function Configuration Register

// Root Complex BAR Configuration Register

pub const CDNS_PCIE_LM_RC_BAR_CFG_PREFETCH_MEM_32BITS: c_int = 0;

pub const CDNS_PCIE_LM_RC_BAR_CFG_IO_16BITS: c_int = 0;

// BAR control values applicable to both Endpoint Function and Root Complex
pub const CDNS_PCIE_LM_BAR_CFG_CTRL_DISABLED: c_uint = 0x0;
pub const CDNS_PCIE_LM_BAR_CFG_CTRL_IO_32BITS: c_uint = 0x1;
pub const CDNS_PCIE_LM_BAR_CFG_CTRL_MEM_32BITS: c_uint = 0x4;
pub const CDNS_PCIE_LM_BAR_CFG_CTRL_PREFETCH_MEM_32BITS: c_uint = 0x5;
pub const CDNS_PCIE_LM_BAR_CFG_CTRL_MEM_64BITS: c_uint = 0x6;
pub const CDNS_PCIE_LM_BAR_CFG_CTRL_PREFETCH_MEM_64BITS: c_uint = 0x7;

// PTM Control Register

//
// Endpoint Function Registers (PCI configuration space for endpoint functions)
//

pub const CDNS_PCIE_EP_FUNC_MSI_CAP_OFFSET: c_uint = 0x90;
pub const CDNS_PCIE_EP_FUNC_MSIX_CAP_OFFSET: c_uint = 0xB0;
pub const CDNS_PCIE_EP_FUNC_DEV_CAP_OFFSET: c_uint = 0xC0;
pub const CDNS_PCIE_EP_FUNC_SRIOV_CAP_OFFSET: c_uint = 0x200;
// Endpoint PF Registers

// Root Port Registers (PCI configuration space for the root port function)
pub const CDNS_PCIE_RP_BASE: c_uint = 0x00200000;
pub const CDNS_PCIE_RP_CAP_OFFSET: c_uint = 0xC0;
// Address Translation Registers
pub const CDNS_PCIE_AT_BASE: c_uint = 0x00400000;
// Region r Outbound AXI to PCIe Address Translation Register 0

// Region r Outbound AXI to PCIe Address Translation Register 1

// Region r Outbound PCIe Descriptor Register 0

pub const CDNS_PCIE_AT_OB_REGION_DESC0_TYPE_MEM: c_uint = 0x2;
pub const CDNS_PCIE_AT_OB_REGION_DESC0_TYPE_IO: c_uint = 0x6;
pub const CDNS_PCIE_AT_OB_REGION_DESC0_TYPE_CONF_TYPE0: c_uint = 0xA;
pub const CDNS_PCIE_AT_OB_REGION_DESC0_TYPE_CONF_TYPE1: c_uint = 0xB;
pub const CDNS_PCIE_AT_OB_REGION_DESC0_TYPE_NORMAL_MSG: c_uint = 0xC;
pub const CDNS_PCIE_AT_OB_REGION_DESC0_TYPE_VENDOR_MSG: c_uint = 0xD;
// Bit 23 MUST be set in RC mode.

// Region r Outbound PCIe Descriptor Register 1

// Region r AXI Region Base Address Register 0

// Region r AXI Region Base Address Register 1

// Root Port BAR Inbound PCIe to AXI Address Translation Register

// AXI link down register

// LTSSM Capabilities register

pub const CDNS_PCIE_DETECT_QUIET_MIN_DELAY_SHIFT: c_int = 1;

pub const CDNS_PCIE_RP_MAX_IB: c_uint = 0x3;
pub const CDNS_PCIE_MAX_OB: c_int = 32;
// Endpoint Function BAR Inbound PCIe to AXI Address Translation Register

// Normal/Vendor specific message access: offset inside some outbound region

