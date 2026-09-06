//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/usb/pegasus.h
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
// Copyright (c) 1999-2013 Petko Manolov (petkan@nucleusys.com)
//

pub const PEGASUS_II: c_uint = 0x80000000;
pub const HAS_HOME_PNA: c_uint = 0x40000000;
pub const PEGASUS_MTU: c_int = 1536;
pub const EPROM_WRITE: c_uint = 0x01;
pub const EPROM_READ: c_uint = 0x02;
pub const EPROM_DONE: c_uint = 0x04;
pub const EPROM_WR_ENABLE: c_uint = 0x10;
pub const EPROM_LOAD: c_uint = 0x20;
pub const PHY_DONE: c_uint = 0x80;
pub const PHY_READ: c_uint = 0x40;
pub const PHY_WRITE: c_uint = 0x20;
pub const DEFAULT_GPIO_RESET: c_uint = 0x24;
pub const DEFAULT_GPIO_SET: c_uint = 0x26;
pub const PEGASUS_PRESENT: c_uint = 0x00000001;
pub const PEGASUS_TX_BUSY: c_uint = 0x00000004;
pub const PEGASUS_RX_BUSY: c_uint = 0x00000008;
pub const CTRL_URB_RUNNING: c_uint = 0x00000010;
pub const CTRL_URB_SLEEP: c_uint = 0x00000020;
pub const PEGASUS_UNPLUG: c_uint = 0x00000040;
pub const PEGASUS_RX_URB_FAIL: c_uint = 0x00000080;
pub const RX_MULTICAST: c_int = 2;
pub const RX_PROMISCUOUS: c_int = 4;

pub const TX_UNDERRUN: c_uint = 0x80;
pub const EXCESSIVE_COL: c_uint = 0x40;
pub const LATE_COL: c_uint = 0x20;
pub const NO_CARRIER: c_uint = 0x10;
pub const LOSS_CARRIER: c_uint = 0x08;
pub const JABBER_TIMEOUT: c_uint = 0x04;
pub const LINK_STATUS: c_uint = 0x01;
pub const PEGASUS_REQT_READ: c_uint = 0xc0;
pub const PEGASUS_REQT_WRITE: c_uint = 0x40;
pub const PEGASUS_REQ_GET_REGS: c_uint = 0xf0;
pub const PEGASUS_REQ_SET_REGS: c_uint = 0xf1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pegasus_registers {
    EthCtrl0 = 0,
    EthCtrl1 = 1,
    EthCtrl2 = 2,
    EthID = 0x10,
    Reg1d = 0x1d,
    EpromOffset = 0x20,
    EpromData = 0x21,	/* 0x21 low, 0x22 high byte */
    EpromCtrl = 0x23,
    PhyAddr = 0x25,
    PhyData = 0x26,		/* 0x26 low, 0x27 high byte */
    PhyCtrl = 0x28,
    UsbStst = 0x2a,
    EthTxStat0 = 0x2b,
    EthTxStat1 = 0x2c,
    EthRxStat = 0x2d,
    WakeupControl = 0x78,
    Reg7b = 0x7b,
    Gpio0 = 0x7e,
    Gpio1 = 0x7f,
    Reg81 = 0x81,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_eth_dev {
    pub name: *mut c_char,
    pub /: *mut *mut __u32 private; / LSB is gpio reset value,
}

pub const VENDOR_3COM: c_uint = 0x0506;
pub const VENDOR_ABOCOM: c_uint = 0x07b8;
pub const VENDOR_ACCTON: c_uint = 0x083a;
pub const VENDOR_ADMTEK: c_uint = 0x07a6;
pub const VENDOR_AEILAB: c_uint = 0x3334;
pub const VENDOR_ALLIEDTEL: c_uint = 0x07c9;
pub const VENDOR_ATEN: c_uint = 0x0557;
pub const VENDOR_BELKIN: c_uint = 0x050d;
pub const VENDOR_BILLIONTON: c_uint = 0x08dd;
pub const VENDOR_COMPAQ: c_uint = 0x049f;
pub const VENDOR_COREGA: c_uint = 0x07aa;
pub const VENDOR_DLINK: c_uint = 0x2001;
pub const VENDOR_ELCON: c_uint = 0x0db7;
pub const VENDOR_ELECOM: c_uint = 0x056e;
pub const VENDOR_ELSA: c_uint = 0x05cc;
pub const VENDOR_GIGABYTE: c_uint = 0x1044;
pub const VENDOR_HAWKING: c_uint = 0x0e66;
pub const VENDOR_HP: c_uint = 0x03f0;
pub const VENDOR_IODATA: c_uint = 0x04bb;
pub const VENDOR_KINGSTON: c_uint = 0x0951;
pub const VENDOR_LANEED: c_uint = 0x056e;
pub const VENDOR_LINKSYS: c_uint = 0x066b;
pub const VENDOR_LINKSYS2: c_uint = 0x077b;
pub const VENDOR_MELCO: c_uint = 0x0411;
pub const VENDOR_MICROSOFT: c_uint = 0x045e;
pub const VENDOR_MOBILITY: c_uint = 0x1342;
pub const VENDOR_NETGEAR: c_uint = 0x0846;
pub const VENDOR_OCT: c_uint = 0x0b39;
pub const VENDOR_SMARTBRIDGES: c_uint = 0x08d1;
pub const VENDOR_SMC: c_uint = 0x0707;
pub const VENDOR_SOHOWARE: c_uint = 0x15e8;
pub const VENDOR_SIEMENS: c_uint = 0x067c;

//
// Distinguish between this Belkin adaptor and the Belkin bluetooth adaptors
// with the same product IDs by checking the device class too.
//
