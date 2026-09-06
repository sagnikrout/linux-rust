//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mdio/mdio-xgene.h
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


// SPDX-License-Identifier: GPL-2.0+
// Applied Micro X-Gene SoC MDIO Driver
//
// Copyright (c) 2016, Applied Micro Circuits Corporation
// Author: Iyappan Subramanian <isubramanian@apm.com>
//

pub const BLOCK_XG_MDIO_CSR_OFFSET: c_uint = 0x5000;
pub const BLOCK_DIAG_CSR_OFFSET: c_uint = 0xd000;
pub const XGENET_CONFIG_REG_ADDR: c_uint = 0x20;
pub const MAC_ADDR_REG_OFFSET: c_uint = 0x00;
pub const MAC_COMMAND_REG_OFFSET: c_uint = 0x04;
pub const MAC_WRITE_REG_OFFSET: c_uint = 0x08;
pub const MAC_READ_REG_OFFSET: c_uint = 0x0c;
pub const MAC_COMMAND_DONE_REG_OFFSET: c_uint = 0x10;
pub const CLKEN_OFFSET: c_uint = 0x08;
pub const SRST_OFFSET: c_uint = 0x00;
pub const MENET_CFG_MEM_RAM_SHUTDOWN_ADDR: c_uint = 0x70;
pub const MENET_BLOCK_MEM_RDY_ADDR: c_uint = 0x74;
pub const MAC_CONFIG_1_ADDR: c_uint = 0x00;
pub const MII_MGMT_COMMAND_ADDR: c_uint = 0x24;
pub const MII_MGMT_ADDRESS_ADDR: c_uint = 0x28;
pub const MII_MGMT_CONTROL_ADDR: c_uint = 0x2c;
pub const MII_MGMT_STATUS_ADDR: c_uint = 0x30;
pub const MII_MGMT_INDICATORS_ADDR: c_uint = 0x34;

pub const MII_MGMT_CONFIG_ADDR: c_uint = 0x20;
pub const MII_MGMT_COMMAND_ADDR: c_uint = 0x24;
pub const MII_MGMT_ADDRESS_ADDR: c_uint = 0x28;
pub const MII_MGMT_CONTROL_ADDR: c_uint = 0x2c;
pub const MII_MGMT_STATUS_ADDR: c_uint = 0x30;
pub const MII_MGMT_INDICATORS_ADDR: c_uint = 0x34;
pub const MIIM_COMMAND_ADDR: c_uint = 0x20;
pub const MIIM_FIELD_ADDR: c_uint = 0x24;
pub const MIIM_CONFIGURATION_ADDR: c_uint = 0x28;
pub const MIIM_LINKFAILVECTOR_ADDR: c_uint = 0x2c;
pub const MIIM_INDICATOR_ADDR: c_uint = 0x30;
pub const MIIMRD_FIELD_ADDR: c_uint = 0x34;
pub const MDIO_CSR_OFFSET: c_uint = 0x5000;
pub const REG_ADDR_POS: c_int = 0;
pub const REG_ADDR_LEN: c_int = 5;
pub const PHY_ADDR_POS: c_int = 8;
pub const PHY_ADDR_LEN: c_int = 5;
pub const HSTMIIMWRDAT_POS: c_int = 0;
pub const HSTMIIMWRDAT_LEN: c_int = 16;
pub const HSTPHYADX_POS: c_int = 23;
pub const HSTPHYADX_LEN: c_int = 5;
pub const HSTREGADX_POS: c_int = 18;
pub const HSTREGADX_LEN: c_int = 5;

pub const HSTMIIMCMD_POS: c_int = 0;
pub const HSTMIIMCMD_LEN: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_enet_cmd {
    XGENE_ENET_WR_CMD = BIT(31),
    XGENE_ENET_RD_CMD = BIT(30)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_mdio_id {
    XGENE_MDIO_RGMII = 1,
    XGENE_MDIO_XFI
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_mdio_pdata {
    pub clk: *mut clk,
    pub dev: *mut device,
    pub mac_csr_addr: *mut void __iomem,
    pub diag_csr_addr: *mut void __iomem,
    pub mdio_csr_addr: *mut void __iomem,
    pub mdio_bus: *mut mii_bus,
    pub mdio_id: c_int,
    pub /: *mut *mut spinlock_t mac_lock; / mac lock,
}

// Set the specified value into a bit-field defined by its starting position
// and length within a single u64.
//

// Get the value from a bit-field defined by its starting position
// and length within the specified u64.
//

extern "C" {
    pub fn xgene_mdio_rd_mac(pdata: *mut xgene_mdio_pdata, rd_addr: u32) -> u32;
}
extern "C" {
    pub fn xgene_mdio_wr_mac(pdata: *mut xgene_mdio_pdata, wr_addr: u32, data: u32);
}
extern "C" {
    pub fn xgene_mdio_rgmii_read(bus: *mut mii_bus, phy_id: c_int, reg: c_int) -> c_int;
}
extern "C" {
    pub fn xgene_mdio_rgmii_write(bus: *mut mii_bus, phy_id: c_int, reg: c_int, data: u16) -> c_int;
}
