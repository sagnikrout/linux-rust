//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intersil/p54/p54usb.h
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
// Defines for USB based mac80211 Prism54 driver
//
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
//
// Based on the islsm (softmac prism54) driver, which is:
// Copyright 2004-2006 Jean-Baptiste Note <jbnote@gmail.com>, et al.
//
// for isl3886 register definitions used on ver 1 devices

// pci
pub const NET2280_BASE: c_uint = 0x10000000;
pub const NET2280_BASE2: c_uint = 0x20000000;
// gpio

// devinit

// endpoints

// irq

// registers
pub const NET2280_DEVINIT: c_uint = 0x00;
pub const NET2280_USBIRQENB1: c_uint = 0x24;
pub const NET2280_IRQSTAT1: c_uint = 0x2c;
pub const NET2280_FIFOCTL: c_uint = 0x38;
pub const NET2280_GPIOCTL: c_uint = 0x50;
pub const NET2280_RELNUM: c_uint = 0x88;
pub const NET2280_EPA_RSP: c_uint = 0x324;
pub const NET2280_EPA_STAT: c_uint = 0x32c;
pub const NET2280_EPB_STAT: c_uint = 0x34c;
pub const NET2280_EPC_RSP: c_uint = 0x364;
pub const NET2280_EPC_STAT: c_uint = 0x36c;
pub const NET2280_EPD_STAT: c_uint = 0x38c;
pub const NET2280_EPA_CFG: c_uint = 0x320;
pub const NET2280_EPB_CFG: c_uint = 0x340;
pub const NET2280_EPC_CFG: c_uint = 0x360;
pub const NET2280_EPD_CFG: c_uint = 0x380;
pub const NET2280_EPE_CFG: c_uint = 0x3A0;
pub const NET2280_EPF_CFG: c_uint = 0x3C0;
pub const P54U_DEV_BASE: c_uint = 0x40000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_tx_hdr {
    pub device_addr: __le32,
    pub len: __le16,
    pub /: *mut *mut __le16 follower; / ?,
    pub padding: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm87_tx_hdr {
    pub device_addr: __le32,
    pub chksum: __le32,
    pub __packed: },
// Some flags for the isl hardware registers controlling DMA inside the
// chip
pub const ISL38XX_DMA_STATUS_DONE: c_uint = 0x00000001;
pub const ISL38XX_DMA_STATUS_READY: c_uint = 0x00000002;
pub const NET2280_EPA_FIFO_PCI_ADDR: c_uint = 0x20000000;
pub const ISL38XX_DMA_MASTER_CONTROL_TRIGGER: c_uint = 0x00000004;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net2280_op_type {
    NET2280_BRG_U32		= 0x001F,
    NET2280_BRG_CFG_U32	= 0x000F,
    NET2280_BRG_CFG_U16	= 0x0003,
    NET2280_DEV_U32		= 0x080F,
    NET2280_DEV_CFG_U32	= 0x088F,
    NET2280_DEV_CFG_U16	= 0x0883
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_reg_write {
    pub port: __le16,
    pub addr: __le32,
    pub val: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net2280_reg_read {
    pub port: __le16,
    pub addr: __le32,
    pub __packed: },
pub const P54U_FW_BLOCK: c_int = 2048;

pub const X2_SIGNATURE_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x2_header {
    pub signature: [u8; X2_SIGNATURE_SIZE],
    pub fw_load_addr: __le32,
    pub fw_length: __le32,
    pub crc: __le32,
    pub __packed: },
// pipes 3 and 4 are not used by the driver
pub const P54U_PIPE_NUMBER: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p54u_pipe_addr {
    P54U_PIPE_DATA = 0x01,
    P54U_PIPE_MGMT = 0x02,
    P54U_PIPE_3 = 0x03,
    P54U_PIPE_4 = 0x04,
    P54U_PIPE_BRG = 0x0d,
    P54U_PIPE_DEV = 0x0e,
    P54U_PIPE_INT = 0x0f
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54u_rx_info {
    pub urb: *mut urb,
    pub dev: *mut ieee80211_hw,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p54u_hw_type {
    P54U_INVALID_HW,
    P54U_NET2280,
    P54U_3887,

// keep last
    __NUM_P54U_HWTYPES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54u_priv {
    pub common: p54_common,
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub dev): *mut *mut int (upload_fw)(struct ieee80211_hw,
    pub hw_type: p54u_hw_type,
    pub lock: spinlock_t,
    pub rx_queue: sk_buff_head,
    pub submitted: usb_anchor,
    pub fw: *const firmware,
// asynchronous firmware callback
    pub fw_wait_load: completion,
}
