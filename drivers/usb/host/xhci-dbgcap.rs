//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/xhci-dbgcap.h
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
// xhci-dbgcap.h - xHCI debug capability support
//
// Copyright (C) 2017 Intel Corporation
//
// Author: Lu Baolu <baolu.lu@linux.intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_regs {
    pub capability: __le32,
    pub doorbell: __le32,
    pub Size*/: *mut *mut __le32 ersts; / Event Ring Segment Table,
    pub /: *mut *mut __le32 __reserved_0; / 0c~0f reserved bits,
    pub /: *mut *mut __le64 erstba; / Event Ring Segment Table Base Address,
    pub /: *mut *mut __le64 erdp; / Event Ring Dequeue Pointer,
    pub control: __le32,
    pub status: __le32,
    pub /: *mut *mut __le32 portsc; / Port status and control,
    pub /: *mut *mut __le32 __reserved_1; / 2b~28 reserved bits,
    pub /: *mut *mut __le64 dccp; / Debug Capability Context Pointer,
    pub /: *mut *mut __le32 devinfo1; / Device Descriptor Info Register 1,
    pub /: *mut *mut __le32 devinfo2; / Device Descriptor Info Register 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_info_context {
    pub string0: __le64,
    pub manufacturer: __le64,
    pub product: __le64,
    pub serial: __le64,
    pub length: __le32,
    pub __reserved_0: [__le32; 7],
}

pub const DBC_MAX_PACKET: c_int = 1024;
pub const DBC_CONTEXT_SIZE: c_int = 64;
//
// Port status:
//

//
// The maximum length of a string descriptor is 255, because the bLength
// field in the usb_string_descriptor struct is __u8.  In practice the
// maximum length is 254, because a string descriptor consists of a 2 byte
// header followed by UTF-16 characters (2 bytes each). This allows for
// only 126 characters (code points) in the string, which is where
// USB_MAX_STRING_LEN comes from.
//
pub const USB_MAX_STRING_DESC_LEN: c_int = 254;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_str_descs {
    pub string0: [c_char; USB_MAX_STRING_DESC_LEN],
    pub manufacturer: [c_char; USB_MAX_STRING_DESC_LEN],
    pub product: [c_char; USB_MAX_STRING_DESC_LEN],
    pub serial: [c_char; USB_MAX_STRING_DESC_LEN],
}

//
// NULL terminated UTF-8 strings used to create UTF-16 strings
// (with maxiumum USB_MAX_STRING_LEN 2 byte characters).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_str {
    pub manufacturer: [c_char; USB_MAX_STRING_LEN+1],
    pub product: [c_char; USB_MAX_STRING_LEN+1],
    pub serial: [c_char; USB_MAX_STRING_LEN+1],
}

pub const DBC_VENDOR_ID: c_uint = 0x1d6b	/* Linux Foundation 0x1d6b */;
pub const DBC_PRODUCT_ID: c_uint = 0x0010	/* device 0010 */;
pub const DBC_DEVICE_REV: c_uint = 0x0010	/* 0.10 */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbc_state {
    DS_DISABLED = 0,
    DS_INITIALIZED,
    DS_ENABLED,
    DS_CONNECTED,
    DS_CONFIGURED,
    DS_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_ep {
    pub dbc: *mut xhci_dbc,
    pub list_pending: list_head,
    pub ring: *mut xhci_ring,
    pub direction:1: c_uint,
    pub halted:1: c_uint,
}

pub const DBC_QUEUE_SIZE: c_int = 16;
pub const DBC_WRITE_BUF_SIZE: c_int = 8192;

//
// Private structure for DbC hardware state:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_port {
    pub port: tty_port,
    pub /: *mut *mut spinlock_t port_lock; / port access,
    pub minor: c_int,
    pub read_pool: list_head,
    pub read_queue: list_head,
    pub n_read: c_uint,
    pub push: tasklet_struct,
    pub write_pool: list_head,
    pub tx_boundary: c_uint,
    pub registered: bool,
    pub tx_running: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_driver {
    pub dbc): *mut *mut int (configure)(struct xhci_dbc,
    pub dbc): *mut *mut void (disconnect)(struct xhci_dbc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_dbc {
    pub /: *mut *mut spinlock_t lock; / device access,
    pub enable_mutex: mutex,
    pub dev: *mut device,
    pub xhci: *mut xhci_hcd,
    pub regs: *mut dbc_regs __iomem,
    pub ring_evt: *mut xhci_ring,
    pub ring_in: *mut xhci_ring,
    pub ring_out: *mut xhci_ring,
    pub erst: xhci_erst,
    pub ctx: *mut xhci_container_ctx,
    pub str_descs: *mut dbc_str_descs,
    pub str_descs_dma: dma_addr_t,
    pub str_descs_size: usize,
    pub str: dbc_str,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub bInterfaceProtocol: u8,
    pub state: dbc_state,
    pub event_work: delayed_work,
    pub /: *mut *mut unsigned int poll_interval; / ms,
    pub xfer_timestamp: c_ulong,
    pub state_timestamp: c_ulong,
    pub resume_required:1: unsigned,
    pub pending_rpm_put:1: unsigned,
    pub eps: [dbc_ep; 2],
    pub driver: *const dbc_driver,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_request {
    pub buf: *mut c_void,
    pub length: c_uint,
    pub dma: dma_addr_t,
    pub req): *mut dbc_request,
    pub list_pool: list_head,
    pub status: c_int,
    pub actual: c_uint,
    pub dbc: *mut xhci_dbc,
    pub list_pending: list_head,
    pub trb_dma: dma_addr_t,
    pub trb: *mut xhci_trb,
    pub direction:1: unsigned,
}

pub const BULK_OUT: c_int = 0;
pub const BULK_IN: c_int = 1;
pub const EPID_OUT: c_int = 2;
pub const EPID_IN: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum evtreturn {
    EVT_ERR	= -1,
    EVT_DONE,
    EVT_XFER_DONE,
    EVT_GSER,
    EVT_DISC,
}

extern "C" {
    pub fn xhci_create_dbc_dev(xhci: *mut xhci_hcd) -> c_int;
}
extern "C" {
    pub fn xhci_remove_dbc_dev(xhci: *mut xhci_hcd);
}
extern "C" {
    pub fn xhci_dbc_init() -> c_int;
}
extern "C" {
    pub fn xhci_dbc_exit();
}
extern "C" {
    pub fn dbc_tty_init() -> c_int;
}
extern "C" {
    pub fn dbc_tty_exit();
}
extern "C" {
    pub fn xhci_dbc_tty_probe(dev: *mut device, res: *mut void __iomem, xhci: *mut xhci_hcd) -> c_int;
}
extern "C" {
    pub fn xhci_dbc_tty_remove(dbc: *mut xhci_dbc);
}
extern "C" {
    pub fn xhci_dbc_remove(dbc: *mut xhci_dbc);
}
extern "C" {
    pub fn dbc_free_request(req: *mut dbc_request);
}
extern "C" {
    pub fn dbc_ep_queue(req: *mut dbc_request) -> c_int;
}

extern "C" {
    pub fn xhci_dbc_suspend(xhci: *mut xhci_hcd) -> c_int;
}
extern "C" {
    pub fn xhci_dbc_resume(xhci: *mut xhci_hcd) -> c_int;
}

