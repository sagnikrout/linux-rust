//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intersil/p54/p54pci.h
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
// Defines for PCI based mac80211 Prism54 driver
//
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
//
// Based on the islsm (softmac prism54) driver, which is:
// Copyright 2004-2006 Jean-Baptiste Note <jbnote@gmail.com>, et al.
//
// Device Interrupt register bits
pub const ISL38XX_DEV_INT_RESET: c_uint = 0x0001;
pub const ISL38XX_DEV_INT_UPDATE: c_uint = 0x0002;
pub const ISL38XX_DEV_INT_WAKEUP: c_uint = 0x0008;
pub const ISL38XX_DEV_INT_SLEEP: c_uint = 0x0010;
pub const ISL38XX_DEV_INT_ABORT: c_uint = 0x0020;
// these two only used in USB
pub const ISL38XX_DEV_INT_DATA: c_uint = 0x0040;
pub const ISL38XX_DEV_INT_MGMT: c_uint = 0x0080;
pub const ISL38XX_DEV_INT_PCIUART_CTS: c_uint = 0x4000;
pub const ISL38XX_DEV_INT_PCIUART_DR: c_uint = 0x8000;
// Interrupt Identification/Acknowledge/Enable register bits
pub const ISL38XX_INT_IDENT_UPDATE: c_uint = 0x0002;
pub const ISL38XX_INT_IDENT_INIT: c_uint = 0x0004;
pub const ISL38XX_INT_IDENT_WAKEUP: c_uint = 0x0008;
pub const ISL38XX_INT_IDENT_SLEEP: c_uint = 0x0010;
pub const ISL38XX_INT_IDENT_PCIUART_CTS: c_uint = 0x4000;
pub const ISL38XX_INT_IDENT_PCIUART_DR: c_uint = 0x8000;
// Control/Status register bits
pub const ISL38XX_CTRL_STAT_SLEEPMODE: c_uint = 0x00000200;
pub const ISL38XX_CTRL_STAT_CLKRUN: c_uint = 0x00800000;
pub const ISL38XX_CTRL_STAT_RESET: c_uint = 0x10000000;
pub const ISL38XX_CTRL_STAT_RAMBOOT: c_uint = 0x20000000;
pub const ISL38XX_CTRL_STAT_STARTHALTED: c_uint = 0x40000000;
pub const ISL38XX_CTRL_STAT_HOST_OVERRIDE: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54p_csr {
    pub dev_int: __le32,
    pub unused_1: [u8; 12],
    pub int_ident: __le32,
    pub int_ack: __le32,
    pub int_enable: __le32,
    pub unused_2: [u8; 4],
    pub ring_control_base: __le32,
    pub gen_purp_com: [__le32; 2],
}

// usb backend only needs the register defines above
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54p_desc {
    pub host_addr: __le32,
    pub device_addr: __le32,
    pub len: __le16,
    pub flags: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54p_ring_control {
    pub host_idx: [__le32; 4],
    pub device_idx: [__le32; 4],
    pub rx_data: [p54p_desc; 8],
    pub tx_data: [p54p_desc; 32],
    pub rx_mgmt: [p54p_desc; 4],
    pub tx_mgmt: [p54p_desc; 4],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54p_priv {
    pub common: p54_common,
    pub pdev: *mut pci_dev,
    pub map: *mut p54p_csr __iomem,
    pub tasklet: tasklet_struct,
    pub firmware: *const firmware,
    pub lock: spinlock_t,
    pub ring_control: *mut p54p_ring_control,
    pub ring_control_dma: dma_addr_t,
    pub tx_idx_data: u32 rx_idx_data,,
    pub tx_idx_mgmt: u32 rx_idx_mgmt,,
    pub rx_buf_data: [*mut sk_buff; 8],
    pub rx_buf_mgmt: [*mut sk_buff; 4],
    pub tx_buf_data: [*mut sk_buff; 32],
    pub tx_buf_mgmt: [*mut sk_buff; 4],
    pub boot_comp: completion,
    pub fw_loaded: completion,
}

