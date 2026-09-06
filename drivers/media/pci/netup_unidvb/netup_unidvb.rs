//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/netup_unidvb/netup_unidvb.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// netup_unidvb.h
//
// Data type definitions for NetUP Universal Dual DVB-CI
//
// Copyright (C) 2014 NetUP Inc.
// Copyright (C) 2014 Sergey Kozlov <serjk@netup.ru>
// Copyright (C) 2014 Abylay Ospan <aospan@netup.ru>
//

pub const NETUP_VENDOR_ID: c_uint = 0x1b55;
pub const NETUP_PCI_DEV_REVISION: c_uint = 0x2;
// IRQ-related regisers
pub const REG_ISR: c_uint = 0x4890;
pub const REG_ISR_MASKED: c_uint = 0x4892;
pub const REG_IMASK_SET: c_uint = 0x4894;
pub const REG_IMASK_CLEAR: c_uint = 0x4896;
// REG_ISR register bits

// NetUP Universal DVB card hardware revisions and it's PCI device id's:
// 1.3 - CXD2841ER demod, ASCOT2E and HORUS3A tuners
// 1.4 - CXD2854ER demod, HELENE tuner
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netup_hw_rev {
    NETUP_HW_REV_1_3 = 0x18F6,
    NETUP_HW_REV_1_4 = 0x18F7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netup_dma {
    pub num: u8,
    pub lock: spinlock_t,
    pub ndev: *mut netup_unidvb_dev,
    pub regs: *mut netup_dma_regs __iomem,
    pub ring_buffer_size: u32,
    pub addr_virt: *mut u8,
    pub addr_phys: dma_addr_t,
    pub addr_last: u64,
    pub high_addr: u32,
    pub data_offset: u32,
    pub data_size: u32,
    pub free_buffers: list_head,
    pub work: work_struct,
    pub timeout: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netup_i2c_state {
    STATE_DONE,
    STATE_WAIT,
    STATE_WANT_READ,
    STATE_WANT_WRITE,
    STATE_ERROR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netup_i2c {
    pub lock: spinlock_t,
    pub wq: wait_queue_head_t,
    pub adap: i2c_adapter,
    pub dev: *mut netup_unidvb_dev,
    pub regs: *mut netup_i2c_regs __iomem,
    pub msg: *mut i2c_msg,
    pub state: netup_i2c_state,
    pub xmit_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netup_ci_state {
    pub ca: dvb_ca_en50221,
    pub membase8_config: *mut u8 __iomem,
    pub membase8_io: *mut u8 __iomem,
    pub dev: *mut netup_unidvb_dev,
    pub status: c_int,
    pub nr: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netup_unidvb_dev {
    pub pci_dev: *mut pci_dev,
    pub pci_bus: c_int,
    pub pci_slot: c_int,
    pub pci_func: c_int,
    pub board_num: c_int,
    pub old_fw: c_int,
    pub lmmio0: *mut u32 __iomem,
    pub bmmio0: *mut u8 __iomem,
    pub lmmio1: *mut u32 __iomem,
    pub bmmio1: *mut u8 __iomem,
    pub dma_virt: *mut u8,
    pub dma_phys: dma_addr_t,
    pub dma_size: u32,
    pub frontends: [vb2_dvb_frontends; 2],
    pub i2c: [netup_i2c; 2],
    pub wq: *mut workqueue_struct,
    pub dma: [netup_dma; 2],
    pub ci: [netup_ci_state; 2],
    pub spi: *mut netup_spi,
    pub rev: netup_hw_rev,
}

extern "C" {
    pub fn netup_i2c_register(ndev: *mut netup_unidvb_dev) -> c_int;
}
extern "C" {
    pub fn netup_i2c_unregister(ndev: *mut netup_unidvb_dev);
}
extern "C" {
    pub fn netup_ci_interrupt(ndev: *mut netup_unidvb_dev) -> irqreturn_t;
}
extern "C" {
    pub fn netup_i2c_interrupt(i2c: *mut netup_i2c) -> irqreturn_t;
}
extern "C" {
    pub fn netup_spi_interrupt(spi: *mut netup_spi) -> irqreturn_t;
}
extern "C" {
    pub fn netup_unidvb_ci_unregister(dev: *mut netup_unidvb_dev, num: c_int);
}
extern "C" {
    pub fn netup_spi_init(ndev: *mut netup_unidvb_dev) -> c_int;
}
extern "C" {
    pub fn netup_spi_release(ndev: *mut netup_unidvb_dev);
}
