//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/xhci-sideband.h
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
// xHCI host controller sideband support
//
// Copyright (c) 2023-2025, Intel Corporation.
//
// Author: Mathias Nyman <mathias.nyman@linux.intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xhci_sideband_type {
    XHCI_SIDEBAND_AUDIO,
    XHCI_SIDEBAND_VENDOR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xhci_sideband_notify_type {
    XHCI_SIDEBAND_XFER_RING_FREE,
}

//
// struct xhci_sideband_event - sideband event
// @type: notifier type
// @evt_data: event data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_sideband_event {
    pub type: xhci_sideband_notify_type,
    pub evt_data: *mut c_void,
}

//
// struct xhci_sideband - representation of a sideband accessed usb device.
// @xhci: The xhci host controller the usb device is connected to
// @vdev: the usb device accessed via sideband
// @eps: array of endpoints controlled via sideband
// @ir: event handling and buffer for sideband accessed device
// @type: xHCI sideband type
// @mutex: mutex for sideband operations
// @intf: USB sideband client interface
// @notify_client: callback for xHCI sideband sequences
//
// FIXME usb device accessed via sideband Keeping track of sideband accessed usb devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_sideband {
    pub xhci: *mut xhci_hcd,
    pub vdev: *mut xhci_virt_device,
    pub eps: [*mut xhci_virt_ep; EP_CTX_PER_DEV],
    pub ir: *mut xhci_interrupter,
    pub type: xhci_sideband_type,
// Synchronizing xHCI sideband operations with client drivers operations
    pub mutex: mutex,
    pub intf: *mut usb_interface,
    pub evt): *mut xhci_sideband_event,
}

extern "C" {
    pub fn xhci_sideband_check(hcd: *mut usb_hcd) -> bool;
}

