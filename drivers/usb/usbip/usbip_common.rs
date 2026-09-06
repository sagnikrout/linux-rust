//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/usbip/usbip_common.h
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
// Copyright (C) 2015-2016 Samsung Electronics
// Krzysztof Opasiak <k.opasiak@samsung.com>
//

//
// USB/IP request headers
//
// Each request is transferred across the network to its counterpart, which
// facilitates the normal USB communication. The values contained in the headers
// are basically the same as in a URB. Currently, four request types are
// defined:
//
// - USBIP_CMD_SUBMIT: a USB request block, corresponds to usb_submit_urb()
// (client to server)
//
// - USBIP_RET_SUBMIT: the result of USBIP_CMD_SUBMIT
// (server to client)
//
// - USBIP_CMD_UNLINK: an unlink request of a pending USBIP_CMD_SUBMIT,
// corresponds to usb_unlink_urb()
// (client to server)
//
// - USBIP_RET_UNLINK: the result of USBIP_CMD_UNLINK
// (server to client)
//
pub const USBIP_CMD_SUBMIT: c_uint = 0x0001;
pub const USBIP_CMD_UNLINK: c_uint = 0x0002;
pub const USBIP_RET_SUBMIT: c_uint = 0x0003;
pub const USBIP_RET_UNLINK: c_uint = 0x0004;
pub const USBIP_DIR_OUT: c_uint = 0x00;
pub const USBIP_DIR_IN: c_uint = 0x01;
//
// Arbitrary limit for the maximum number of isochronous packets in an URB,
// compare for example the uhci_submit_isochronous function in
// drivers/usb/host/uhci-q.c
//
pub const USBIP_MAX_ISO_PACKETS: c_int = 1024;
//
// struct usbip_header_basic - data pertinent to every request
// @command: the usbip request type
// @seqnum: sequential number that identifies requests; incremented per
// connection
// @devid: specifies a remote USB device uniquely instead of busnum and devnum;
// in the stub driver, this value is ((busnum << 16) | devnum)
// @direction: direction of the transfer
// @ep: endpoint number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_header_basic {
    pub command: __u32,
    pub seqnum: __u32,
    pub devid: __u32,
    pub direction: __u32,
    pub ep: __u32,
    pub __packed: },
//
// struct usbip_header_cmd_submit - USBIP_CMD_SUBMIT packet header
// @transfer_flags: URB flags
// @transfer_buffer_length: the data size for (in) or (out) transfer
// @start_frame: initial frame for isochronous or interrupt transfers
// @number_of_packets: number of isochronous packets
// @interval: maximum time for the request on the server-side host controller
// @setup: setup data for a control request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_header_cmd_submit {
    pub transfer_flags: __u32,
    pub transfer_buffer_length: __s32,
// it is difficult for usbip to sync frames (reserved only?)
    pub start_frame: __s32,
    pub number_of_packets: __s32,
    pub interval: __s32,
    pub setup: [c_uchar; 8],
    pub __packed: },
//
// struct usbip_header_ret_submit - USBIP_RET_SUBMIT packet header
// @status: return status of a non-iso request
// @actual_length: number of bytes transferred
// @start_frame: initial frame for isochronous or interrupt transfers
// @number_of_packets: number of isochronous packets
// @error_count: number of errors for isochronous transfers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_header_ret_submit {
    pub status: __s32,
    pub actual_length: __s32,
    pub start_frame: __s32,
    pub number_of_packets: __s32,
    pub error_count: __s32,
    pub __packed: },
//
// struct usbip_header_cmd_unlink - USBIP_CMD_UNLINK packet header
// @seqnum: the URB seqnum to unlink
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_header_cmd_unlink {
    pub seqnum: __u32,
    pub __packed: },
//
// struct usbip_header_ret_unlink - USBIP_RET_UNLINK packet header
// @status: return status of the request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_header_ret_unlink {
    pub status: __s32,
    pub __packed: },
//
// struct usbip_header - common header for all usbip packets
// @base: the basic header
// @u: packet type dependent header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_header {
    pub base: usbip_header_basic,
    pub cmd_submit: usbip_header_cmd_submit,
    pub ret_submit: usbip_header_ret_submit,
    pub cmd_unlink: usbip_header_cmd_unlink,
    pub ret_unlink: usbip_header_ret_unlink,
    pub u: },
    pub __packed: },
//
// This is the same as usb_iso_packet_descriptor but packed for pdu.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_iso_packet_descriptor {
    pub offset: __u32,
    pub /: *mut *mut __u32 length; / expected length,
    pub actual_length: __u32,
    pub status: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usbip_side {
    USBIP_VHCI,
    USBIP_STUB,
    USBIP_VUDC,
}

// event handler

// catastrophic emulated usb error

// a common structure for stub_device and vhci_device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_device {
    pub side: usbip_side,
    pub status: usbip_device_status,
// lock for status
    pub lock: spinlock_t,
// mutex for synchronizing sysfs store paths
    pub sysfs_lock: mutex,
    pub sockfd: c_int,
    pub tcp_socket: *mut socket,
    pub tcp_rx: *mut task_struct,
    pub tcp_tx: *mut task_struct,
    pub event: c_ulong,
    pub eh_waitq: wait_queue_head_t,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eh_ops {
    pub ): *mut *mut void (shutdown)(struct usbip_device,
    pub ): *mut *mut void (reset)(struct usbip_device,
    pub ): *mut *mut void (unusable)(struct usbip_device,
    pub eh_ops: },
    pub kcov_handle: kcov_common_handle_id,
}

// usbip_common.c
extern "C" {
    pub fn usbip_dump_urb(purb: *mut urb);
}
extern "C" {
    pub fn usbip_dump_header(pdu: *mut usbip_header);
}
extern "C" {
    pub fn usbip_recv(sock: *mut socket, buf: *mut c_void, size: c_int) -> c_int;
}
extern "C" {
    pub fn usbip_header_correct_endian(pdu: *mut usbip_header, send: c_int);
}
// some members of urb must be substituted before.
extern "C" {
    pub fn usbip_recv_iso(ud: *mut usbip_device, urb: *mut urb) -> c_int;
}
extern "C" {
    pub fn usbip_pad_iso(ud: *mut usbip_device, urb: *mut urb);
}
extern "C" {
    pub fn usbip_recv_xbuff(ud: *mut usbip_device, urb: *mut urb) -> c_int;
}
// usbip_event.c
extern "C" {
    pub fn usbip_init_eh() -> c_int;
}
extern "C" {
    pub fn usbip_finish_eh();
}
extern "C" {
    pub fn usbip_start_eh(ud: *mut usbip_device) -> c_int;
}
extern "C" {
    pub fn usbip_stop_eh(ud: *mut usbip_device);
}
extern "C" {
    pub fn usbip_event_add(ud: *mut usbip_device, event: c_ulong);
}
extern "C" {
    pub fn usbip_event_happened(ud: *mut usbip_device) -> c_int;
}
extern "C" {
    pub fn usbip_in_eh(task: *mut task_struct) -> c_int;
}
