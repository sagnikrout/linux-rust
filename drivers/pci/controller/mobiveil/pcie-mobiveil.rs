//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/mobiveil/pcie-mobiveil.h
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
// PCIe host controller driver for Mobiveil PCIe Host controller
//
// Copyright (c) 2018 Mobiveil Inc.
// Copyright 2019 NXP
//
// Author: Subrahmanya Lingappa <l.subrahmanya@mobiveil.co.in>
// Hou Zhiqiang <Zhiqiang.Hou@nxp.com>
//

// register offsets and bit positions
//
// translation tables are grouped into windows, each window registers are
// grouped into blocks of 4 or 16 registers each
//
pub const PAB_REG_BLOCK_SIZE: c_int = 16;
pub const PAB_EXT_REG_BLOCK_SIZE: c_int = 4;

pub const LTSSM_STATUS: c_uint = 0x0404;
pub const LTSSM_STATUS_L0_MASK: c_uint = 0x3f;
pub const LTSSM_STATUS_L0: c_uint = 0x2d;
pub const PAB_CTRL: c_uint = 0x0808;
pub const AMBA_PIO_ENABLE_SHIFT: c_int = 0;
pub const PEX_PIO_ENABLE_SHIFT: c_int = 1;
pub const PAGE_SEL_SHIFT: c_int = 13;
pub const PAGE_SEL_MASK: c_uint = 0x3f;
pub const PAGE_LO_MASK: c_uint = 0x3ff;
pub const PAGE_SEL_OFFSET_SHIFT: c_int = 10;
pub const PAB_ACTIVITY_STAT: c_uint = 0x81c;
pub const PAB_AXI_PIO_CTRL: c_uint = 0x0840;
pub const APIO_EN_MASK: c_uint = 0xf;
pub const PAB_PEX_PIO_CTRL: c_uint = 0x08c0;
pub const PIO_ENABLE_SHIFT: c_int = 0;
pub const PAB_INTP_AMBA_MISC_ENB: c_uint = 0x0b0c;
pub const PAB_INTP_AMBA_MISC_STAT: c_uint = 0x0b1c;

pub const WIN_ENABLE_SHIFT: c_int = 0;
pub const WIN_TYPE_SHIFT: c_int = 1;
pub const WIN_TYPE_MASK: c_uint = 0x3;
pub const WIN_SIZE_MASK: c_uint = 0xfffffc00;

pub const AXI_WINDOW_ALIGN_MASK: c_int = 3;

pub const PAB_BUS_SHIFT: c_int = 24;
pub const PAB_DEVICE_SHIFT: c_int = 19;
pub const PAB_FUNCTION_SHIFT: c_int = 16;

pub const PAB_INTP_AXI_PIO_CLASS: c_uint = 0x474;

pub const AMAP_CTRL_EN_SHIFT: c_int = 0;
pub const AMAP_CTRL_TYPE_SHIFT: c_int = 1;
pub const AMAP_CTRL_TYPE_MASK: c_int = 3;

// starting offset of INTX bits in status register
pub const PAB_INTX_START: c_int = 5;
// supported number of MSI interrupts
pub const PCI_NUM_MSI: c_int = 16;
// MSI registers
pub const MSI_BASE_LO_OFFSET: c_uint = 0x04;
pub const MSI_BASE_HI_OFFSET: c_uint = 0x08;
pub const MSI_SIZE_OFFSET: c_uint = 0x0c;
pub const MSI_ENABLE_OFFSET: c_uint = 0x14;
pub const MSI_STATUS_OFFSET: c_uint = 0x18;
pub const MSI_DATA_OFFSET: c_uint = 0x20;
pub const MSI_ADDR_L_OFFSET: c_uint = 0x24;
pub const MSI_ADDR_H_OFFSET: c_uint = 0x28;
// outbound and inbound window definitions
pub const WIN_NUM_0: c_int = 0;
pub const WIN_NUM_1: c_int = 1;
pub const CFG_WINDOW_TYPE: c_int = 0;
pub const IO_WINDOW_TYPE: c_int = 1;
pub const MEM_WINDOW_TYPE: c_int = 2;

pub const MAX_PIO_WINDOWS: c_int = 8;
pub const PAGED_ADDR_BNDRY: c_uint = 0xc00;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mobiveil_msi {
    pub /: *mut *mut mutex lock; / protect bitmap variable,
    pub dev_domain: *mut irq_domain,
    pub msi_pages_phys: phys_addr_t,
    pub num_of_vectors: c_int,
    pub PCI_NUM_MSI): DECLARE_BITMAP(msi_irq_in_use,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mobiveil_rp_ops {
    pub pcie): *mut *mut int (interrupt_init)(struct mobiveil_pcie,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mobiveil_root_port {
    pub /: *mut *mut *mut void __iomem config_axi_slave_base; / endpoint config base,
    pub ob_io_res: *mut resource,
    pub ops: *const mobiveil_rp_ops,
    pub irq: c_int,
    pub intx_mask_lock: raw_spinlock_t,
    pub intx_domain: *mut irq_domain,
    pub msi: mobiveil_msi,
    pub bridge: *mut pci_host_bridge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mobiveil_pab_ops {
    pub pcie): *mut *mut bool (link_up)(struct mobiveil_pcie,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mobiveil_pcie {
    pub pdev: *mut platform_device,
    pub /: *mut *mut *mut void __iomem csr_axi_slave_base; / root port config base,
    pub /: *mut *mut *mut void __iomem apb_csr_base; / MSI register base,
    pub /: *mut *mut phys_addr_t pcie_reg_base; / Physical PCIe Controller Base,
    pub apio_wins: c_int,
    pub ppio_wins: c_int,
    pub /: *mut *mut int ob_wins_configured; / configured outbound windows,
    pub /: *mut *mut int ib_wins_configured; / configured inbound windows,
    pub ops: *const mobiveil_pab_ops,
    pub rp: mobiveil_root_port,
}

extern "C" {
    pub fn mobiveil_pcie_host_probe(pcie: *mut mobiveil_pcie) -> c_int;
}
extern "C" {
    pub fn mobiveil_host_init(pcie: *mut mobiveil_pcie, reinit: bool) -> c_int;
}
extern "C" {
    pub fn mobiveil_pcie_link_up(pcie: *mut mobiveil_pcie) -> bool;
}
extern "C" {
    pub fn mobiveil_bringup_link(pcie: *mut mobiveil_pcie) -> c_int;
}
extern "C" {
    pub fn mobiveil_csr_read(pcie: *mut mobiveil_pcie, off: u32, size: usize) -> u32;
}
extern "C" {
    pub fn mobiveil_csr_read(_arg: pcie, _arg: off, _arg: 0x4) -> return;
}
extern "C" {
    pub fn mobiveil_csr_read(_arg: pcie, _arg: off, _arg: 0x2) -> return;
}
extern "C" {
    pub fn mobiveil_csr_read(_arg: pcie, _arg: off, _arg: 0x1) -> return;
}
