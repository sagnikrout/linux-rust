//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/zydas/zd1211rw/zd_usb.h
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
// ZD1211 USB-WLAN driver for Linux
//
// Copyright (C) 2005-2007 Ulrich Kunitz <kune@deine-taler.de>
// Copyright (C) 2006-2007 Daniel Drake <dsd@gentoo.org>
//

pub const ZD_USB_TX_HIGH: c_int = 5;
pub const ZD_USB_TX_LOW: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devicetype {
    DEVICE_ZD1211  = 0,
    DEVICE_ZD1211B = 1,
    DEVICE_INSTALLER = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum endpoints {
    EP_CTRL	    = 0,
    EP_DATA_OUT = 1,
    EP_DATA_IN  = 2,
    EP_INT_IN   = 3,
    EP_REGS_OUT = 4,
}

// FIXME: The original driver uses this value. We have to check,
// whether the MAX_TRANSFER_SIZE is sufficient and this needs only be
// used if one combined frame is split over two USB transactions.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum control_requests {
    USB_REQ_WRITE_REGS		= 0x21,
    USB_REQ_READ_REGS		= 0x22,
    USB_REQ_WRITE_RF		= 0x23,
    USB_REQ_PROG_FLASH		= 0x24,
    USB_REQ_EEPROM_START		= 0x0128, /* ? request is a byte */
    USB_REQ_EEPROM_MID		= 0x28,
    USB_REQ_EEPROM_END		= 0x0228, /* ? request is a byte */
    USB_REQ_FIRMWARE_DOWNLOAD	= 0x30,
    USB_REQ_FIRMWARE_CONFIRM	= 0x31,
    USB_REQ_FIRMWARE_READ_DATA	= 0x32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_req_read_regs {
    pub id: __le16,
    pub addr: [__le16; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_data {
    pub addr: __le16,
    pub value: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_req_write_regs {
    pub id: __le16,
    pub reg_writes: [reg_data; ],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_req_rfwrite {
    pub id: __le16,
    pub value: __le16,
// 1: 3683a
// 2: other (default)
    pub bits: __le16,
// RF2595: 24
    pub bit_values: [__le16; ],
// (ZD_CR203 & ~(RF_IF_LE | RF_CLK | RF_DATA)) | (bit ? RF_DATA : 0)
    pub __packed: },
// USB interrupt
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_int_id {
    USB_INT_TYPE			= 0x01,
    USB_INT_ID_REGS			= 0x90,
    USB_INT_ID_RETRY_FAILED		= 0xa0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_int_flags {
    USB_INT_READ_REGS_EN		= 0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_int_header {
    pub /: *mut *mut u8 type; / must always be 1,
    pub id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_int_regs {
    pub hdr: usb_int_header,
    pub regs: [reg_data; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_int_retry_fail {
    pub hdr: usb_int_header,
    pub new_rate: u8,
    pub _dummy: u8,
    pub addr: [u8; ETH_ALEN],
    pub ibss_wakeup_dest: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_regs_int {
    pub completion: completion,
    pub req: *mut usb_req_read_regs,
    pub req_count: c_uint,
// Stores the USB int structure and contains the USB address of the
// first requested register before request.
//
    pub buffer: [u8; USB_MAX_EP_INT_BUFFER],
    pub length: c_int,
    pub cr_int_addr: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_ioreq16 {
    pub addr: zd_addr_t,
    pub value: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_ioreq32 {
    pub addr: zd_addr_t,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_usb_interrupt {
    pub read_regs: read_regs_int,
    pub lock: spinlock_t,
    pub urb: *mut urb,
    pub buffer: *mut c_void,
    pub buffer_dma: dma_addr_t,
    pub interval: c_int,
    pub read_regs_enabled: core::sync::atomic::AtomicI32,
    pub read_regs_int_overridden:1: u8,
}

pub const RX_URBS_COUNT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_usb_rx {
    pub lock: spinlock_t,
    pub setup_mutex: mutex,
    pub idle_work: delayed_work,
    pub reset_timer_tasklet: tasklet_struct,
    pub USB_MAX_RX_SIZE]: *mut *mut u8 fragment[2,
    pub fragment_length: c_uint,
    pub usb_packet_size: c_uint,
    pub urbs: *mut urb,
    pub urbs_count: c_int,
}

//
// struct zd_usb_tx - structure used for transmitting frames
// @enabled: atomic enabled flag, indicates whether tx is enabled
// @lock: lock for transmission
// @submitted: anchor for URBs sent to device
// @submitted_urbs: atomic integer that counts the URBs having sent to the
// device, which haven't been completed
// @stopped: indicates whether higher level tx queues are stopped
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_usb_tx {
    pub enabled: core::sync::atomic::AtomicI32,
    pub lock: spinlock_t,
    pub watchdog_work: delayed_work,
    pub submitted_skbs: sk_buff_head,
    pub submitted: usb_anchor,
    pub submitted_urbs: c_int,
    pub watchdog_enabled:1: u8 stopped:1,,
}

// Contains the usb parts. The structure doesn't require a lock because intf
// will not be changed after initialization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_usb {
    pub intr: zd_usb_interrupt,
    pub rx: zd_usb_rx,
    pub tx: zd_usb_tx,
    pub intf: *mut usb_interface,
    pub submitted_cmds: usb_anchor,
    pub urb_async_waiting: *mut urb,
    pub cmd_error: c_int,
    pub /: *mut *mut u8 req_buf[64]; / zd_usb_iowrite16v needs 62 bytes,
    pub in_async:1: u8 is_zd1211b:1, initialized:1, was_running:1,,
}

extern "C" {
    pub fn interface_to_usbdev(_arg: usb->intf) -> return;
}
extern "C" {
    pub fn usb_get_intfdata(_arg: intf) -> return;
}
extern "C" {
    pub fn zd_intf_to_hw(_arg: usb->intf) -> return;
}
extern "C" {
    pub fn zd_usb_init_hw(usb: *mut zd_usb) -> c_int;
}
extern "C" {
    pub fn zd_usb_clear(usb: *mut zd_usb);
}
extern "C" {
    pub fn zd_usb_scnprint_id(usb: *mut zd_usb, buffer: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn zd_tx_watchdog_enable(usb: *mut zd_usb);
}
extern "C" {
    pub fn zd_tx_watchdog_disable(usb: *mut zd_usb);
}
extern "C" {
    pub fn zd_usb_enable_int(usb: *mut zd_usb) -> c_int;
}
extern "C" {
    pub fn zd_usb_disable_int(usb: *mut zd_usb);
}
extern "C" {
    pub fn zd_usb_enable_rx(usb: *mut zd_usb) -> c_int;
}
extern "C" {
    pub fn zd_usb_disable_rx(usb: *mut zd_usb);
}
extern "C" {
    pub fn zd_usb_reset_rx_idle_timer(usb: *mut zd_usb);
}
extern "C" {
    pub fn zd_usb_enable_tx(usb: *mut zd_usb);
}
extern "C" {
    pub fn zd_usb_disable_tx(usb: *mut zd_usb);
}
extern "C" {
    pub fn zd_usb_tx(usb: *mut zd_usb, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn zd_usb_ioread16v(_arg: usb, _arg: value, _arg: &addr, _arg: 1) -> return;
}
extern "C" {
    pub fn zd_usb_iowrite16v_async_start(usb: *mut zd_usb);
}
extern "C" {
    pub fn zd_usb_iowrite16v_async_end(usb: *mut zd_usb, timeout: c_uint) -> c_int;
}
extern "C" {
    pub fn zd_usb_rfwrite(usb: *mut zd_usb, value: u32, bits: u8) -> c_int;
}
extern "C" {
    pub fn zd_usb_read_fw(usb: *mut zd_usb, addr: zd_addr_t, data: *mut u8, len: u16) -> c_int;
}
