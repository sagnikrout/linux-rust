//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/usbip/vhci.h
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
// Copyright (C) 2003-2008 Takahiro Hirofuchi
// Copyright (C) 2015 Nobuo Iwata
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhci_device {
    pub udev: *mut usb_device,
//
// devid specifies a remote usb device uniquely instead
// of combination of busnum and devnum.
//
    pub devid: __u32,
// speed of a remote device
    pub speed: usb_device_speed,
// vhci root-hub port to which this device is attached
    pub rhport: __u32,
    pub ud: usbip_device,
// lock for the below link lists
    pub priv_lock: spinlock_t,
// vhci_priv is linked to one of them.
    pub priv_tx: list_head,
    pub priv_rx: list_head,
// vhci_unlink is linked to one of them
    pub unlink_tx: list_head,
    pub unlink_rx: list_head,
// vhci_tx thread sleeps for this queue
    pub waitq_tx: wait_queue_head_t,
}

// urb->hcpriv, use container_of()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhci_priv {
    pub seqnum: c_ulong,
    pub list: list_head,
    pub vdev: *mut vhci_device,
    pub urb: *mut urb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhci_unlink {
// seqnum of this request
    pub seqnum: c_ulong,
    pub list: list_head,
// seqnum of the unlink target
    pub unlink_seqnum: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hub_speed {
    HUB_SPEED_HIGH = 0,
    HUB_SPEED_SUPER,
}

// Number of supported ports. Value has an upperbound of USB_MAXCHILDREN

pub const VHCI_HC_PORTS: c_int = 8;

// Each VHCI has 2 hubs (USB2 and USB3), each has VHCI_HC_PORTS ports

pub const VHCI_NR_HCS: c_int = 1;

pub const MAX_STATUS_NAME: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhci {
    pub lock: spinlock_t,
    pub pdev: *mut platform_device,
    pub vhci_hcd_hs: *mut vhci_hcd,
    pub vhci_hcd_ss: *mut vhci_hcd,
}

// for usb_hcd.hcd_priv[0]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhci_hcd {
    pub vhci: *mut vhci,
    pub port_status: [u32; VHCI_HC_PORTS],
    pub resuming:1: unsigned,
    pub re_timeout: c_ulong,
    pub seqnum: core::sync::atomic::AtomicI32,
//
// NOTE:
// wIndex shows the port number and begins from 1.
// But, the index of this array begins from 0.
//
    pub vdev: [vhci_device; VHCI_HC_PORTS],
}

// vhci_hcd.c
extern "C" {
    pub fn rh_port_connect(vdev: *mut vhci_device, speed: usb_device_speed);
}
// vhci_sysfs.c
extern "C" {
    pub fn vhci_init_attr_group() -> c_int;
}
extern "C" {
    pub fn vhci_finish_attr_group();
}
// vhci_rx.c
extern "C" {
    pub fn vhci_rx_loop(data: *mut c_void) -> c_int;
}
// vhci_tx.c
extern "C" {
    pub fn vhci_tx_loop(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn container_of(vhci_hcd: *mut *mut (void ), usb_hcd: struct, _arg: hcd_priv) -> return;
}
extern "C" {
    pub fn container_of(vdev->rhport): *mut *mut (void )(vdev -, vhci_hcd: struct, _arg: vdev) -> return;
}
