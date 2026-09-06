//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/early/xhci-dbc.h
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
// xhci-dbc.h - xHCI debug capability early driver
//
// Copyright (C) 2016 Intel Corporation
//
// Author: Lu Baolu <baolu.lu@linux.intel.com>
//

//
// xHCI Debug Capability Register interfaces:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdbc_regs {
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

//
// xHCI Debug Capability data structures:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdbc_trb {
    pub field: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdbc_erst_entry {
    pub seg_addr: __le64,
    pub seg_size: __le32,
    pub __reserved_0: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdbc_info_context {
    pub string0: __le64,
    pub manufacturer: __le64,
    pub product: __le64,
    pub serial: __le64,
    pub length: __le32,
    pub __reserved_0: [__le32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdbc_ep_context {
    pub ep_info1: __le32,
    pub ep_info2: __le32,
    pub deq: __le64,
    pub tx_info: __le32,
    pub __reserved_0: [__le32; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdbc_context {
    pub info: xdbc_info_context,
    pub out: xdbc_ep_context,
    pub in: xdbc_ep_context,
}

pub const XDBC_INFO_CONTEXT_SIZE: c_int = 48;
pub const XDBC_MAX_STRING_LENGTH: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdbc_strings {
    pub string0: [c_char; XDBC_MAX_STRING_LENGTH],
    pub manufacturer: [c_char; XDBC_MAX_STRING_LENGTH],
    pub product: [c_char; XDBC_MAX_STRING_LENGTH],
    pub serial: [c_char; XDBC_MAX_STRING_LENGTH],
}

pub const XDBC_VENDOR_ID: c_uint = 0x1d6b	/* Linux Foundation 0x1d6b */;
pub const XDBC_PRODUCT_ID: c_uint = 0x0011	/* __le16 idProduct; device 0011 */;
pub const XDBC_DEVICE_REV: c_uint = 0x0010	/* 0.10 */;
//
// xHCI Debug Capability software state structures:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdbc_segment {
    pub trbs: *mut xdbc_trb,
    pub dma: dma_addr_t,
}

pub const XDBC_TRBS_PER_SEGMENT: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdbc_ring {
    pub segment: *mut xdbc_segment,
    pub enqueue: *mut xdbc_trb,
    pub dequeue: *mut xdbc_trb,
    pub cycle_state: u32,
}

//
// These are the "Endpoint ID" (also known as "Context Index") values for the
// OUT Transfer Ring and the IN Transfer Ring of a Debug Capability Context data
// structure.
// According to the "eXtensible Host Controller Interface for Universal Serial
// Bus (xHCI)" specification, section "7.6.3.2 Endpoint Contexts and Transfer
// Rings", these should be 0 and 1, and those are the values AMD machines give
// you; but Intel machines seem to use the formula from section "4.5.1 Device
// Context Index", which is supposed to be used for the Device Context only.
// Luckily the values from Intel don't overlap with those from AMD, so we can
// just test for both.
//
pub const XDBC_EPID_OUT: c_int = 0;
pub const XDBC_EPID_IN: c_int = 1;
pub const XDBC_EPID_OUT_INTEL: c_int = 2;
pub const XDBC_EPID_IN_INTEL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdbc_state {
    pub vendor: u16,
    pub device: u16,
    pub bus: u32,
    pub dev: u32,
    pub func: u32,
    pub xhci_base: *mut void __iomem,
    pub xhci_start: u64,
    pub xhci_length: usize,
    pub port_number: c_int,
// DbC register base
    pub xdbc_reg: *mut xdbc_regs __iomem,
// DbC table page
    pub table_dma: dma_addr_t,
    pub table_base: *mut c_void,
// event ring segment table
    pub erst_dma: dma_addr_t,
    pub erst_size: usize,
    pub erst_base: *mut c_void,
// event ring segments
    pub evt_ring: xdbc_ring,
    pub evt_seg: xdbc_segment,
// debug capability contexts
    pub dbcc_dma: dma_addr_t,
    pub dbcc_size: usize,
    pub dbcc_base: *mut c_void,
// descriptor strings
    pub string_dma: dma_addr_t,
    pub string_size: usize,
    pub string_base: *mut c_void,
// bulk OUT endpoint
    pub out_ring: xdbc_ring,
    pub out_seg: xdbc_segment,
    pub out_buf: *mut c_void,
    pub out_dma: dma_addr_t,
// bulk IN endpoint
    pub in_ring: xdbc_ring,
    pub in_seg: xdbc_segment,
    pub in_buf: *mut c_void,
    pub in_dma: dma_addr_t,
    pub flags: u32,
// spinlock for early_xdbc_write() reentrancy
    pub lock: raw_spinlock_t,
}

pub const XDBC_PCI_MAX_BUSES: c_int = 256;
pub const XDBC_PCI_MAX_DEVICES: c_int = 32;
pub const XDBC_PCI_MAX_FUNCTION: c_int = 8;
pub const XDBC_TABLE_ENTRY_SIZE: c_int = 64;
pub const XDBC_ERST_ENTRY_NUM: c_int = 1;
pub const XDBC_DBCC_ENTRY_NUM: c_int = 3;
pub const XDBC_STRING_ENTRY_NUM: c_int = 4;
// Bits definitions for xdbc_state.flags:

pub const XDBC_MAX_PACKET: c_int = 1024;
// Door bell target:
pub const OUT_EP_DOORBELL: c_int = 0;
pub const IN_EP_DOORBELL: c_int = 1;

