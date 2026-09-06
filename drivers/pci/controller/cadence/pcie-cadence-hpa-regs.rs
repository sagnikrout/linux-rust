//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/cadence/pcie-cadence-hpa-regs.h
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
// Copyright (c) 2024, Cadence Design Systems
// Author: Manikandan K Pillai <mpillai@cadence.com>
//

// High Performance Architecture (HPA) PCIe controller registers
pub const CDNS_PCIE_HPA_IP_REG_BANK: c_uint = 0x01000000;
pub const CDNS_PCIE_HPA_IP_CFG_CTRL_REG_BANK: c_uint = 0x01003C00;
pub const CDNS_PCIE_HPA_IP_AXI_MASTER_COMMON: c_uint = 0x02020000;
// Address Translation Registers
pub const CDNS_PCIE_HPA_AXI_SLAVE: c_uint = 0x03000000;
pub const CDNS_PCIE_HPA_AXI_MASTER: c_uint = 0x03002000;
// Root Port register base address
pub const CDNS_PCIE_HPA_RP_BASE: c_uint = 0x0;
pub const CDNS_PCIE_HPA_LM_ID: c_uint = 0x1420;
// Endpoint Function BARs

// Endpoint Function Configuration Register
pub const CDNS_PCIE_HPA_LM_EP_FUNC_CFG: c_uint = 0x02C0;
// Root Complex BAR Configuration Register
pub const CDNS_PCIE_HPA_LM_RC_BAR_CFG: c_uint = 0x14;

// BAR control values applicable to both Endpoint Function and Root Complex
pub const CDNS_PCIE_HPA_LM_BAR_CFG_CTRL_DISABLED: c_uint = 0x0;
pub const CDNS_PCIE_HPA_LM_BAR_CFG_CTRL_IO_32BITS: c_uint = 0x3;
pub const CDNS_PCIE_HPA_LM_BAR_CFG_CTRL_MEM_32BITS: c_uint = 0x1;
pub const CDNS_PCIE_HPA_LM_BAR_CFG_CTRL_PREFETCH_MEM_32BITS: c_uint = 0x9;
pub const CDNS_PCIE_HPA_LM_BAR_CFG_CTRL_MEM_64BITS: c_uint = 0x5;
pub const CDNS_PCIE_HPA_LM_BAR_CFG_CTRL_PREFETCH_MEM_64BITS: c_uint = 0xD;

pub const CDNS_PCIE_HPA_LM_PTM_CTRL: c_uint = 0x0520;

// Root Port Registers PCI config space for root port function
pub const CDNS_PCIE_HPA_RP_CAP_OFFSET: c_uint = 0xC0;
// Region r Outbound AXI to PCIe Address Translation Register 0

// Region r Outbound AXI to PCIe Address Translation Register 1

// Region r Outbound PCIe Descriptor Register

// Region r Outbound PCIe Descriptor Register

// Region r AXI Region Base Address Register 0

// Region r AXI Region Base Address Register 1

// Root Port BAR Inbound PCIe to AXI Address Translation Register

// AXI link down register
pub const CDNS_PCIE_HPA_AT_LINKDOWN: c_uint = 0x04;
//
// Physical Layer Configuration Register 0
// This register contains the parameters required for functional setup
// of Physical Layer.
//
pub const CDNS_PCIE_HPA_PHY_LAYER_CFG0: c_uint = 0x0400;

pub const CDNS_PCIE_HPA_PHY_DBG_STS_REG0: c_uint = 0x0420;
pub const CDNS_PCIE_HPA_RP_MAX_IB: c_uint = 0x3;
pub const CDNS_PCIE_HPA_MAX_OB: c_int = 15;
// Endpoint Function BAR Inbound PCIe to AXI Address Translation Register

// Miscellaneous offsets definitions
pub const CDNS_PCIE_HPA_TAG_MANAGEMENT: c_uint = 0x0;
pub const CDNS_PCIE_HPA_SLAVE_RESP: c_uint = 0x100;
pub const I_ROOT_PORT_REQ_ID_REG: c_uint = 0x141c;
pub const LM_HAL_SBSA_CTRL: c_uint = 0x1170;

pub const CDNS_PCIE_EROM: c_uint = 0x18;
