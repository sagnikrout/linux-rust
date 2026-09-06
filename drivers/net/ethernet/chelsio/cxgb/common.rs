//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/common.h
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
// File: common.h
// $Revision: 1.21 $
// $Date: 2005/06/22 00:43:25 $
// Description:
// part of the Chelsio 10Gb Ethernet Driver.
//
// http://www.chelsio.com
//
// Copyright (c) 2003 - 2005 Chelsio Communications, Inc.
// All rights reserved.
//
// Maintainers: maintainers@chelsio.com
//
// Authors: Dimitrios Michailidis   <dm@chelsio.com>
// Tina Yang               <tainay@chelsio.com>
// Felix Marti             <felix@chelsio.com>
// Scott Bardone           <sbardone@chelsio.com>
// Kurt Ottaway            <kottaway@chelsio.com>
// Frank DiMambro          <frank@chelsio.com>
//
// History:
//

pub type adapter_t = adapter;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t1_rx_mode {
    pub dev: *mut net_device,
}

pub const MAX_NPORTS: c_int = 4;

pub const NMTUS: c_int = 8;
pub const TCB_SIZE: c_int = 128;
pub const SPEED_INVALID: c_uint = 0xffff;
pub const DUPLEX_INVALID: c_uint = 0xff;
// Max frame size PM3393 can handle. Includes Ethernet header and CRC.
pub const PM3393_MAX_FRAME_SIZE: c_int = 9600;
pub const VSC7326_MAX_MTU: c_int = 9600;
// Revisions of T1 chip
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_params {
    pub cmdQ_size: [c_uint; 2],
    pub freelQ_size: [c_uint; 2],
    pub large_buf_capacity: c_uint,
    pub rx_coalesce_usecs: c_uint,
    pub last_rx_coalesce_raw: c_uint,
    pub default_rx_coalesce_usecs: c_uint,
    pub sample_interval_usecs: c_uint,
    pub coalesce_enable: c_uint,
    pub polling: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chelsio_pci_params {
    pub speed: c_ushort,
    pub width: c_uchar,
    pub is_pcix: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_params {
    pub pm_size: c_uint,
    pub cm_size: c_uint,
    pub pm_rx_base: c_uint,
    pub pm_tx_base: c_uint,
    pub pm_rx_pg_size: c_uint,
    pub pm_tx_pg_size: c_uint,
    pub pm_rx_num_pgs: c_uint,
    pub pm_tx_num_pgs: c_uint,
    pub rx_coalescing_size: c_uint,
    pub use_5tuple_mode: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc5_params {
    pub /: *mut *mut unsigned int mode; / selects MC5 width,
    pub /: *mut *mut unsigned int nservers; / size of server region,
    pub /: *mut *mut unsigned int nroutes; / size of routing region,
}

// Default MC5 region sizes
pub const DEFAULT_SERVER_REGION_LEN: c_int = 256;
pub const DEFAULT_RT_REGION_LEN: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter_params {
    pub sge: sge_params,
    pub mc5: mc5_params,
    pub tp: tp_params,
    pub pci: chelsio_pci_params,
    pub brd_info: *const board_info,
    pub mtus: [c_ushort; NMTUS],
    pub /: *mut *mut unsigned int nports; / # of ethernet ports,
    pub stats_update_period: c_uint,
    pub chip_revision: c_ushort,
    pub chip_version: c_uchar,
    pub is_asic: c_uchar,
    pub has_msi: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_config {
    pub /: *mut *mut unsigned int supported; / link capabilities,
    pub /: *mut *mut unsigned int advertising; / advertised capabilities,
    pub /: *mut *mut unsigned short requested_speed; / speed user has requested,
    pub /: *mut *mut unsigned short speed; / actual link speed,
    pub /: *mut *mut unsigned char requested_duplex; / duplex user has requested,
    pub /: *mut *mut unsigned char duplex; / actual link duplex,
    pub /: *mut *mut unsigned char requested_fc; / flow control user has requested,
    pub /: *mut *mut unsigned char fc; / actual link flow control,
    pub /: *mut *mut unsigned char autoneg; / autonegotiating?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_info {
    pub dev: *mut net_device,
    pub mac: *mut cmac,
    pub phy: *mut cphy,
    pub link_config: link_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter {
    pub regs: *mut u8 __iomem,
    pub pdev: *mut pci_dev,
    pub registered_device_map: c_ulong,
    pub open_device_map: c_ulong,
    pub flags: c_ulong,
    pub name: *const c_char,
    pub msg_enable: c_int,
    pub mmio_len: u32,
    pub params: adapter_params,
// Terminator modules.
    pub sge: *mut sge,
    pub espi: *mut peespi,
    pub tp: *mut petp,
    pub napi: napi_struct,
    pub port: [port_info; MAX_NPORTS],
    pub stats_update_task: delayed_work,
    pub stats_update_timer: timer_list,
    pub tpi_lock: spinlock_t,
    pub work_lock: spinlock_t,
    pub mac_lock: spinlock_t,
// guards async operations
    pub ____cacheline_aligned: spinlock_t async_lock,
    pub pending_thread_intr: u32,
    pub slow_intr_mask: u32,
    pub t1powersave: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct board_info {
    pub board: c_uchar,
    pub port_number: c_uchar,
    pub caps: c_ulong,
    pub chip_term: c_uchar,
    pub chip_mac: c_uchar,
    pub chip_phy: c_uchar,
    pub clock_core: c_uint,
    pub clock_mc3: c_uint,
    pub clock_mc4: c_uint,
    pub espi_nports: c_uint,
    pub clock_elmer0: c_uint,
    pub mdio_mdien: c_uchar,
    pub mdio_mdiinv: c_uchar,
    pub mdio_mdc: c_uchar,
    pub mdio_phybaseaddr: c_uchar,
    pub gmac: *const gmac,
    pub gphy: *const gphy,
    pub mdio_ops: *const mdio_ops,
    pub desc: *const c_char,
}

// Returns true if an adapter supports VLAN acceleration and TSO

extern "C" {
    pub fn __t1_tpi_read(adapter: *mut adapter_t, addr: u32, valp: *mut u32) -> c_int;
}
extern "C" {
    pub fn __t1_tpi_write(adapter: *mut adapter_t, addr: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn t1_tpi_write(adapter: *mut adapter_t, addr: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn t1_tpi_read(adapter: *mut adapter_t, addr: u32, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn t1_interrupts_enable(adapter: *mut adapter_t);
}
extern "C" {
    pub fn t1_interrupts_disable(adapter: *mut adapter_t);
}
extern "C" {
    pub fn t1_interrupts_clear(adapter: *mut adapter_t);
}
extern "C" {
    pub fn t1_elmer0_ext_intr_handler(adapter: *mut adapter_t) -> c_int;
}
extern "C" {
    pub fn t1_slow_intr_handler(adapter: *mut adapter_t) -> irqreturn_t;
}
extern "C" {
    pub fn t1_link_start(phy: *mut cphy, mac: *mut cmac, lc: *mut link_config) -> c_int;
}
extern "C" {
    pub fn t1_seeprom_read(adapter: *mut adapter_t, addr: u32, data: *mut __le32) -> c_int;
}
extern "C" {
    pub fn t1_init_hw_modules(adapter: *mut adapter_t) -> c_int;
}
extern "C" {
    pub fn t1_init_sw_modules(adapter: *mut adapter_t, bi: *const board_info) -> c_int;
}
extern "C" {
    pub fn t1_free_sw_modules(adapter: *mut adapter_t);
}
extern "C" {
    pub fn t1_link_changed(adapter: *mut adapter_t, port_id: c_int);
}
