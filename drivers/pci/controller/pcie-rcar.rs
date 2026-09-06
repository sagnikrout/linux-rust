//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/pcie-rcar.h
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
// PCIe driver for Renesas R-Car SoCs
// Copyright (C) 2014-2020 Renesas Electronics Europe Ltd
//
// Author: Phil Edworthy <phil.edworthy@renesas.com>
//
pub const PCIECAR: c_uint = 0x000010;
pub const PCIECCTLR: c_uint = 0x000018;

pub const PCIECDR: c_uint = 0x000020;
pub const PCIEMSR: c_uint = 0x000028;
pub const PCIEINTXR: c_uint = 0x000400;

pub const PCIEPHYSR: c_uint = 0x0007f0;

pub const PCIEMSITXR: c_uint = 0x000840;
// Transfer control
pub const PCIETCTLR: c_uint = 0x02000;

pub const PCIETSTR: c_uint = 0x02004;

pub const PCIEERRFR: c_uint = 0x02020;

pub const PCIEMSIFR: c_uint = 0x02044;
pub const PCIEMSIALR: c_uint = 0x02048;

pub const PCIEMSIAUR: c_uint = 0x0204c;
pub const PCIEMSIIER: c_uint = 0x02050;
// root port address

// local address reg & mask

// PCIe address reg & mask

// Configuration

pub const MSICAP0_MMESCAP_OFFSET: c_int = 17;
pub const MSICAP0_MMESE_OFFSET: c_int = 20;

// link layer
pub const IDSETR0: c_uint = 0x011000;
pub const IDSETR1: c_uint = 0x011004;
pub const SUBIDSETR: c_uint = 0x011024;
pub const TLCTLR: c_uint = 0x011048;
pub const MACSR: c_uint = 0x011054;

pub const MACCTLR: c_uint = 0x011058;

pub const PMSR: c_uint = 0x01105c;

pub const PMCTLR: c_uint = 0x011060;

pub const MACS2R: c_uint = 0x011078;
pub const MACCGSPSETR: c_uint = 0x011084;

// R-Car H1 PHY
pub const H1_PCIEPHYADRR: c_uint = 0x04000c;

pub const RATE_POS: c_int = 12;
pub const LANE_POS: c_int = 8;
pub const ADR_POS: c_int = 0;
pub const H1_PCIEPHYDOUTR: c_uint = 0x040014;
// R-Car Gen2 PHY
pub const GEN2_PCIEPHYADDR: c_uint = 0x780;
pub const GEN2_PCIEPHYDATA: c_uint = 0x784;
pub const GEN2_PCIEPHYCTRL: c_uint = 0x78c;
pub const INT_PCI_MSI_NR: c_int = 32;

pub const RCAR_PCI_MAX_RESOURCES: c_int = 4;
pub const MAX_NR_INBOUND_MAPS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_pcie {
    pub dev: *mut device,
    pub base: *mut void __iomem,
}

extern "C" {
    pub fn rcar_pci_write_reg(pcie: *mut rcar_pcie, val: u32, reg: c_uint);
}
extern "C" {
    pub fn rcar_pci_read_reg(pcie: *mut rcar_pcie, reg: c_uint) -> u32;
}
extern "C" {
    pub fn rcar_rmw32(pcie: *mut rcar_pcie, where: c_int, mask: u32, data: u32);
}
extern "C" {
    pub fn rcar_pcie_wait_for_phyrdy(pcie: *mut rcar_pcie) -> c_int;
}
extern "C" {
    pub fn rcar_pcie_wait_for_dl(pcie: *mut rcar_pcie) -> c_int;
}
