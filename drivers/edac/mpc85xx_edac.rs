//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/edac/mpc85xx_edac.h
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
// Freescale MPC85xx Memory Controller kernel module
// Author: Dave Jiang <djiang@mvista.com>
//
// 2006-2007 (c) MontaVista Software, Inc.
//

//
// L2 Err defines
//
pub const MPC85XX_L2_ERRINJHI: c_uint = 0x0000;
pub const MPC85XX_L2_ERRINJLO: c_uint = 0x0004;
pub const MPC85XX_L2_ERRINJCTL: c_uint = 0x0008;
pub const MPC85XX_L2_CAPTDATAHI: c_uint = 0x0020;
pub const MPC85XX_L2_CAPTDATALO: c_uint = 0x0024;
pub const MPC85XX_L2_CAPTECC: c_uint = 0x0028;
pub const MPC85XX_L2_ERRDET: c_uint = 0x0040;
pub const MPC85XX_L2_ERRDIS: c_uint = 0x0044;
pub const MPC85XX_L2_ERRINTEN: c_uint = 0x0048;
pub const MPC85XX_L2_ERRATTR: c_uint = 0x004c;
pub const MPC85XX_L2_ERRADDR: c_uint = 0x0050;
pub const MPC85XX_L2_ERRCTL: c_uint = 0x0058;
// Error Interrupt Enable
pub const L2_EIE_L2CFGINTEN: c_uint = 0x1;
pub const L2_EIE_SBECCINTEN: c_uint = 0x4;
pub const L2_EIE_MBECCINTEN: c_uint = 0x8;
pub const L2_EIE_TPARINTEN: c_uint = 0x10;

// Error Detect
pub const L2_EDE_L2CFGERR: c_uint = 0x1;
pub const L2_EDE_SBECCERR: c_uint = 0x4;
pub const L2_EDE_MBECCERR: c_uint = 0x8;
pub const L2_EDE_TPARERR: c_uint = 0x10;
pub const L2_EDE_MULL2ERR: c_uint = 0x80000000;

//
// PCI Err defines
//
pub const PCI_EDE_TOE: c_uint = 0x00000001;
pub const PCI_EDE_SCM: c_uint = 0x00000002;
pub const PCI_EDE_IRMSV: c_uint = 0x00000004;
pub const PCI_EDE_ORMSV: c_uint = 0x00000008;
pub const PCI_EDE_OWMSV: c_uint = 0x00000010;
pub const PCI_EDE_TGT_ABRT: c_uint = 0x00000020;
pub const PCI_EDE_MST_ABRT: c_uint = 0x00000040;
pub const PCI_EDE_TGT_PERR: c_uint = 0x00000080;
pub const PCI_EDE_MST_PERR: c_uint = 0x00000100;
pub const PCI_EDE_RCVD_SERR: c_uint = 0x00000200;
pub const PCI_EDE_ADDR_PERR: c_uint = 0x00000400;
pub const PCI_EDE_MULTI_ERR: c_uint = 0x80000000;

pub const MPC85XX_PCI_ERR_DR: c_uint = 0x0000;
pub const MPC85XX_PCI_ERR_CAP_DR: c_uint = 0x0004;
pub const MPC85XX_PCI_ERR_EN: c_uint = 0x0008;
pub const PEX_ERR_ICCAIE_EN_BIT: c_uint = 0x00020000;
pub const MPC85XX_PCI_ERR_ATTRIB: c_uint = 0x000c;
pub const MPC85XX_PCI_ERR_ADDR: c_uint = 0x0010;
pub const PEX_ERR_ICCAD_DISR_BIT: c_uint = 0x00020000;
pub const MPC85XX_PCI_ERR_EXT_ADDR: c_uint = 0x0014;
pub const MPC85XX_PCI_ERR_DL: c_uint = 0x0018;
pub const MPC85XX_PCI_ERR_DH: c_uint = 0x001c;
pub const MPC85XX_PCI_GAS_TIMR: c_uint = 0x0020;
pub const MPC85XX_PCI_PCIX_TIMR: c_uint = 0x0024;
pub const MPC85XX_PCIE_ERR_CAP_R0: c_uint = 0x0028;
pub const MPC85XX_PCIE_ERR_CAP_R1: c_uint = 0x002c;
pub const MPC85XX_PCIE_ERR_CAP_R2: c_uint = 0x0030;
pub const MPC85XX_PCIE_ERR_CAP_R3: c_uint = 0x0034;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc85xx_l2_pdata {
    pub name: *mut c_char,
    pub edac_idx: c_int,
    pub l2_vbase: *mut void __iomem,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc85xx_pci_pdata {
    pub name: *mut c_char,
    pub is_pcie: bool,
    pub edac_idx: c_int,
    pub pci_vbase: *mut void __iomem,
    pub irq: c_int,
}
