//! Automatically rewritten from C Header to Rust Module
//! Source: tools/usb/usbip/src/usbip_network.h
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
// Copyright (C) 2005-2007 Takahiro Hirofuchi
//

extern "C" {
    pub fn usbip_setup_port_number(arg: *mut c_char);
}
// ----------------------------------------------------------------------
// Common header for all the kinds of PDUs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_common {
    pub version: u16,

    pub code: u16,
// status codes defined in usbip_common.h
    pub /: *mut *mut uint32_t status; / op_code status (for reply),
    pub __attribute__((packed)): },
// ----------------------------------------------------------------------
// Dummy Code
pub const OP_UNSPEC: c_uint = 0x00;

// ----------------------------------------------------------------------
// Retrieve USB device information. (still not used)
pub const OP_DEVINFO: c_uint = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_devinfo_request {
    pub busid: [c_char; SYSFS_BUS_ID_SIZE],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_devinfo_reply {
    pub udev: usbip_usb_device,
    pub uinf: [usbip_usb_interface; ],
    pub __attribute__((packed)): },
// ----------------------------------------------------------------------
// Import a remote USB device.
pub const OP_IMPORT: c_uint = 0x03;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_import_request {
    pub busid: [c_char; SYSFS_BUS_ID_SIZE],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_import_reply {
    pub udev: usbip_usb_device,
// struct usbip_usb_interface uinf[];
    pub __attribute__((packed)): },

    pub &(reply)->udev);\: usbip_net_pack_usb_device(pack,,
// ----------------------------------------------------------------------
// Export a USB device to a remote host.
pub const OP_EXPORT: c_uint = 0x06;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_export_request {
    pub udev: usbip_usb_device,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_export_reply {
    pub returncode: c_int,
    pub __attribute__((packed)): },

    pub &(request)->udev);\: usbip_net_pack_usb_device(pack,,

// ----------------------------------------------------------------------
// un-Export a USB device from a remote host.
pub const OP_UNEXPORT: c_uint = 0x07;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_unexport_request {
    pub udev: usbip_usb_device,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_unexport_reply {
    pub returncode: c_int,
    pub __attribute__((packed)): },

    pub &(request)->udev);\: usbip_net_pack_usb_device(pack,,

// ----------------------------------------------------------------------
// Negotiate IPSec encryption key. (still not used)
pub const OP_CRYPKEY: c_uint = 0x04;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_crypkey_request {
// 128bit key
    pub key: [u32; 4],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_crypkey_reply {
    pub __reserved: u32,
    pub __attribute__((packed)): },
// ----------------------------------------------------------------------
// Retrieve the list of exported USB devices.
pub const OP_DEVLIST: c_uint = 0x05;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_devlist_request {
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_devlist_reply {
    pub ndev: u32,
// followed by reply_extra[]
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_devlist_reply_extra {
    pub udev: usbip_usb_device,
    pub uinf: [usbip_usb_interface; ],
    pub __attribute__((packed)): },

    pub (reply)->ndev);\: (reply)->ndev = usbip_net_pack_uint32_t(pack,,
    pub num): uint32_t usbip_net_pack_uint32_t(int pack, uint32_t,
    pub num): uint16_t usbip_net_pack_uint16_t(int pack, uint16_t,
    pub udev): *mut void usbip_net_pack_usb_device(int pack, struct usbip_usb_device,
    pub uinf): *mut void usbip_net_pack_usb_interface(int pack, struct usbip_usb_interface,
    pub bufflen): *mut *mut ssize_t usbip_net_recv(int sockfd, void buff, size_t,
    pub bufflen): *mut *mut ssize_t usbip_net_send(int sockfd, void buff, size_t,
    pub status): int usbip_net_send_op_common(int sockfd, uint32_t code, uint32_t,
    pub status): *mut *mut int usbip_net_recv_op_common(int sockfd, uint16_t code, int,
    pub sockfd): int usbip_net_set_reuseaddr(int,
    pub sockfd): int usbip_net_set_nodelay(int,
    pub sockfd): int usbip_net_set_keepalive(int,
    pub sockfd): int usbip_net_set_v6only(int,
    pub port): *mut *mut int usbip_net_tcp_connect(char hostname, char,
