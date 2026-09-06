//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/c67x00/c67x00.h
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
//
// c67x00.h: Cypress C67X00 USB register and field definitions
//
// Copyright (C) 2006-2008 Barco N.V.
// Derived from the Cypress cy7c67200/300 ezusb linux driver and
// based on multiple host controller drivers inside the linux kernel.
//

// ---------------------------------------------------------------------
// Cypress C67x00 register definitions
//
// Hardware Revision Register
pub const HW_REV_REG: c_uint = 0xC004;
// General USB registers
// =====================
// USB Control Register

pub const HOST_MODE: c_uint = 0x0200;

// USB status register - Notice it has different content in hcd/udc mode

pub const EP0_IRQ_FLG: c_uint = 0x0001;
pub const EP1_IRQ_FLG: c_uint = 0x0002;
pub const EP2_IRQ_FLG: c_uint = 0x0004;
pub const EP3_IRQ_FLG: c_uint = 0x0008;
pub const EP4_IRQ_FLG: c_uint = 0x0010;
pub const EP5_IRQ_FLG: c_uint = 0x0020;
pub const EP6_IRQ_FLG: c_uint = 0x0040;
pub const EP7_IRQ_FLG: c_uint = 0x0080;
pub const RESET_IRQ_FLG: c_uint = 0x0100;
pub const SOF_EOP_IRQ_FLG: c_uint = 0x0200;
pub const ID_IRQ_FLG: c_uint = 0x4000;
pub const VBUS_IRQ_FLG: c_uint = 0x8000;
// USB Host only registers
// =======================
// Host n Control Register

pub const PREAMBLE_EN: c_uint = 0x0080	/* Preamble enable */;
pub const SEQ_SEL: c_uint = 0x0040	/* Data Toggle Sequence Bit Select */;
pub const ISO_EN: c_uint = 0x0010	/* Isochronous enable  */;
pub const ARM_EN: c_uint = 0x0001	/* Arm operation */;
// Host n Interrupt Enable Register

pub const SOF_EOP_IRQ_EN: c_uint = 0x0200	/* SOF/EOP Interrupt Enable  */;
pub const SOF_EOP_TMOUT_IRQ_EN: c_uint = 0x0800	/* SOF/EOP Timeout Interrupt Enable  */;
pub const ID_IRQ_EN: c_uint = 0x4000	/* ID interrupt enable */;
pub const VBUS_IRQ_EN: c_uint = 0x8000	/* VBUS interrupt enable */;
pub const DONE_IRQ_EN: c_uint = 0x0001	/* Done Interrupt Enable  */;
// USB status register
pub const HOST_STAT_MASK: c_uint = 0x02FD;

// Host Frame Register

pub const HOST_FRAME_MASK: c_uint = 0x07FF;
// USB Peripheral only registers
// =============================
// Device n Port Sel reg

// Device n Interrupt Enable Register

// HPI registers
// =============
// HPI Status register

pub const MBX_OUT_FLG: c_uint = 0x0001	/* Message out available */;
pub const MBX_IN_FLG: c_uint = 0x0100;
pub const ID_FLG: c_uint = 0x4000;
pub const VBUS_FLG: c_uint = 0x8000;
// Interrupt routing register
pub const HPI_IRQ_ROUTING_REG: c_uint = 0x0142;

pub const ID_TO_HPI_ENABLE: c_uint = 0x4000;
pub const VBUS_TO_HPI_ENABLE: c_uint = 0x8000;
// SIE msg registers

pub const HUSB_TDListDone: c_uint = 0x1000;
pub const SUSB_EP0_MSG: c_uint = 0x0001;
pub const SUSB_EP1_MSG: c_uint = 0x0002;
pub const SUSB_EP2_MSG: c_uint = 0x0004;
pub const SUSB_EP3_MSG: c_uint = 0x0008;
pub const SUSB_EP4_MSG: c_uint = 0x0010;
pub const SUSB_EP5_MSG: c_uint = 0x0020;
pub const SUSB_EP6_MSG: c_uint = 0x0040;
pub const SUSB_EP7_MSG: c_uint = 0x0080;
pub const SUSB_RST_MSG: c_uint = 0x0100;
pub const SUSB_SOF_MSG: c_uint = 0x0200;
pub const SUSB_CFG_MSG: c_uint = 0x0400;
pub const SUSB_SUS_MSG: c_uint = 0x0800;
pub const SUSB_ID_MSG: c_uint = 0x4000;
pub const SUSB_VBUS_MSG: c_uint = 0x8000;
// BIOS interrupt routines

pub const CY_HCD_BUF_ADDR: c_uint = 0x500	/* Base address for host */;
pub const SIE_TD_SIZE: c_uint = 0x200	/* size of the td list */;
pub const SIE_TD_BUF_SIZE: c_uint = 0x400	/* size of the data buffer */;

// Base address of HCD + 2 x TD_SIZE + 2 x TD_BUF_SIZE
pub const CY_UDC_REQ_HEADER_BASE: c_uint = 0x1100;
// 8- byte request headers for IN/OUT transfers
pub const CY_UDC_REQ_HEADER_SIZE: c_int = 8;

pub const CY_UDC_BIOS_REPLACE_BASE: c_uint = 0x1800;
pub const CY_UDC_REQ_BUFFER_BASE: c_uint = 0x2000;
pub const CY_UDC_REQ_BUFFER_SIZE: c_uint = 0x0400;

// ---------------------------------------------------------------------
// Driver data structures
//
// struct c67x00_sie - Common data associated with a SIE
// @lock: lock to protect this struct and the associated chip registers
// @private_data: subdriver dependent data
// @irq: subdriver dependent irq handler, set NULL when not used
// @dev: link to common driver structure
// @sie_num: SIE number on chip, starting from 0
// @mode: SIE mode (host/peripheral/otg/not used)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c67x00_sie {
// Entries to be used by the subdrivers
    pub /: *mut *mut spinlock_t lock; / protect this structure,
    pub private_data: *mut c_void,
    pub msg): *mut *mut *mut void (irq) (struct c67x00_sie sie, u16 int_status, u16,
// Read only:
    pub dev: *mut c67x00_device,
    pub sie_num: c_int,
    pub mode: c_int,
}

//
// struct c67x00_lcp
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c67x00_lcp {
// Internal use only
    pub mutex: mutex,
    pub msg_received: completion,
    pub last_msg: u16,
}

//
// struct c67x00_hpi
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c67x00_hpi {
    pub base: *mut void __iomem,
    pub regstep: c_int,
    pub lock: spinlock_t,
    pub lcp: c67x00_lcp,
}

pub const C67X00_SIES: c_int = 2;
pub const C67X00_PORTS: c_int = 2;
//
// struct c67x00_device - Common data associated with a c67x00 instance
// @hpi: hpi addresses
// @sie: array of sie's on this chip
// @pdev: platform device of instance
// @pdata: configuration provided by the platform
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c67x00_device {
    pub hpi: c67x00_hpi,
    pub sie: [c67x00_sie; C67X00_SIES],
    pub pdev: *mut platform_device,
    pub pdata: *mut c67x00_platform_data,
}

// ---------------------------------------------------------------------
// Low level interface functions
//
// Host Port Interface (HPI) functions
extern "C" {
    pub fn c67x00_ll_hpi_status(dev: *mut c67x00_device) -> u16;
}
extern "C" {
    pub fn c67x00_ll_hpi_reg_init(dev: *mut c67x00_device);
}
extern "C" {
    pub fn c67x00_ll_hpi_enable_sofeop(sie: *mut c67x00_sie);
}
extern "C" {
    pub fn c67x00_ll_hpi_disable_sofeop(sie: *mut c67x00_sie);
}
// General functions
extern "C" {
    pub fn c67x00_ll_fetch_siemsg(dev: *mut c67x00_device, sie_num: c_int) -> u16;
}
extern "C" {
    pub fn c67x00_ll_get_usb_ctl(sie: *mut c67x00_sie) -> u16;
}
extern "C" {
    pub fn c67x00_ll_usb_clear_status(sie: *mut c67x00_sie, bits: u16);
}
extern "C" {
    pub fn c67x00_ll_usb_get_status(sie: *mut c67x00_sie) -> u16;
}
// Host specific functions
extern "C" {
    pub fn c67x00_ll_set_husb_eot(dev: *mut c67x00_device, value: u16);
}
extern "C" {
    pub fn c67x00_ll_husb_reset(sie: *mut c67x00_sie, port: c_int);
}
extern "C" {
    pub fn c67x00_ll_husb_set_current_td(sie: *mut c67x00_sie, addr: u16);
}
extern "C" {
    pub fn c67x00_ll_husb_get_current_td(sie: *mut c67x00_sie) -> u16;
}
extern "C" {
    pub fn c67x00_ll_husb_get_frame(sie: *mut c67x00_sie) -> u16;
}
extern "C" {
    pub fn c67x00_ll_husb_init_host_port(sie: *mut c67x00_sie);
}
extern "C" {
    pub fn c67x00_ll_husb_reset_port(sie: *mut c67x00_sie, port: c_int);
}
// Called by c67x00_irq to handle lcp interrupts
extern "C" {
    pub fn c67x00_ll_irq(dev: *mut c67x00_device, int_status: u16);
}
// Setup and teardown
extern "C" {
    pub fn c67x00_ll_init(dev: *mut c67x00_device);
}
extern "C" {
    pub fn c67x00_ll_release(dev: *mut c67x00_device);
}
extern "C" {
    pub fn c67x00_ll_reset(dev: *mut c67x00_device) -> c_int;
}
