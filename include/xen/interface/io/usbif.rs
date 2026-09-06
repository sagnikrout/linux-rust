//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/usbif.h
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


// SPDX-License-Identifier: MIT
//
// usbif.h
//
// USB I/O interface for Xen guest OSes.
//
// Copyright (C) 2009, FUJITSU LABORATORIES LTD.
// Author: Noboru Iwamatsu <n_iwamatsu@jp.fujitsu.com>
//

//
// Detailed Interface Description
// ==============================
// The pvUSB interface is using a split driver design: a frontend driver in
// the guest and a backend driver in a driver domain (normally dom0) having
// access to the physical USB device(s) being passed to the guest.
//
// The frontend and backend drivers use XenStore to initiate the connection
// between them, the I/O activity is handled via two shared ring pages and an
// event channel. As the interface between frontend and backend is at the USB
// host connector level, multiple (up to 31) physical USB devices can be
// handled by a single connection.
//
// The Xen pvUSB device name is "qusb", so the frontend's XenStore entries are
// to be found under "device/qusb", while the backend's XenStore entries are
// under "backend/<guest-dom-id>/qusb".
//
// When a new pvUSB connection is established, the frontend needs to setup the
// two shared ring pages for communication and the event channel. The ring
// pages need to be made available to the backend via the grant table
// interface.
//
// One of the shared ring pages is used by the backend to inform the frontend
// about USB device plug events (device to be added or removed). This is the
// "conn-ring".
//
// The other ring page is used for USB I/O communication (requests and
// responses). This is the "urb-ring".
//
// Feature and Parameter Negotiation
// =================================
// The two halves of a Xen pvUSB driver utilize nodes within the XenStore to
// communicate capabilities and to negotiate operating parameters. This
// section enumerates these nodes which reside in the respective front and
// backend portions of the XenStore, following the XenBus convention.
//
// Any specified default value is in effect if the corresponding XenBus node
// is not present in the XenStore.
//
// XenStore nodes in sections marked "PRIVATE" are solely for use by the
// driver side whose XenBus tree contains them.
//
// Backend XenBus Nodes
//
// ------------------ Backend Device Identification (PRIVATE) ------------------
//
// num-ports
// Values:         unsigned [1...31]
//
// Number of ports for this (virtual) USB host connector.
//
// usb-ver
// Values:         unsigned [1...2]
//
// USB version of this host connector: 1 = USB 1.1, 2 = USB 2.0.
//
// port/[1...31]
// Values:         string
//
// Physical USB device connected to the given port, e.g. "3-1.5".
//
// Frontend XenBus Nodes
//
// ----------------------- Request Transport Parameters -----------------------
//
// event-channel
// Values:         unsigned
//
// The identifier of the Xen event channel used to signal activity
// in the ring buffer.
//
// urb-ring-ref
// Values:         unsigned
//
// The Xen grant reference granting permission for the backend to map
// the sole page in a single page sized ring buffer. This is the ring
// buffer for urb requests.
//
// conn-ring-ref
// Values:         unsigned
//
// The Xen grant reference granting permission for the backend to map
// the sole page in a single page sized ring buffer. This is the ring
// buffer for connection/disconnection requests.
//
// protocol
// Values:         string (XEN_IO_PROTO_ABI_*)
// Default Value:  XEN_IO_PROTO_ABI_NATIVE
//
// The machine ABI rules governing the format of all ring request and
// response structures.
//
// Protocol Description
// ====================
//
// -------------------------- USB device plug events --------------------------
//
// USB device plug events are send via the "conn-ring" shared page. As only
// events are being sent, the respective requests from the frontend to the
// backend are just dummy ones.
// The events sent to the frontend have the following layout:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |    portnum     |     speed      | 4
// +----------------+----------------+----------------+----------------+
// id - uint16_t, event id (taken from the actual frontend dummy request)
// portnum - uint8_t, port number (1 ... 31)
// speed - uint8_t, device XENUSB_SPEED_*, XENUSB_SPEED_NONE == unplug
//
// The dummy request:
// 0                1        octet
// +----------------+----------------+
// |               id                | 2
// +----------------+----------------+
// id - uint16_t, guest supplied value (no need for being unique)
//
// -------------------------- USB I/O request ---------------------------------
//
// A single USB I/O request on the "urb-ring" has the following layout:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |         nr_buffer_segs          | 4
// +----------------+----------------+----------------+----------------+
// |                               pipe                                | 8
// +----------------+----------------+----------------+----------------+
// |         transfer_flags          |          buffer_length          | 12
// +----------------+----------------+----------------+----------------+
// |                       request type specific                       | 16
// |                               data                                | 20
// +----------------+----------------+----------------+----------------+
// |                              seg[0]                               | 24
// |                               data                                | 28
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |             seg[XENUSB_MAX_SEGMENTS_PER_REQUEST - 1]              | 144
// |                               data                                | 148
// +----------------+----------------+----------------+----------------+
// Bit field bit number 0 is always least significant bit, undefined bits must
// be zero.
// id - uint16_t, guest supplied value
// nr_buffer_segs - uint16_t, number of segment entries in seg[] array
// pipe - uint32_t, bit field with multiple information:
// bits 0-4: port request to send to
// bit 5: unlink request with specified id (cancel I/O) if set (see below)
// bit 7: direction (1 = read from device)
// bits 8-14: device number on port
// bits 15-18: endpoint of device
// bits 30-31: request type: 00 = isochronous, 01 = interrupt,
// 10 = control, 11 = bulk
// transfer_flags - uint16_t, bit field with processing flags:
// bit 0: less data than specified allowed
// buffer_length - uint16_t, total length of data
// request type specific data - 8 bytes, see below
// seg[] - array with 8 byte elements, see below
//
// Request type specific data for isochronous request:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |            interval             |           start_frame           | 4
// +----------------+----------------+----------------+----------------+
// |       number_of_packets         |       nr_frame_desc_segs        | 8
// +----------------+----------------+----------------+----------------+
// interval - uint16_t, time interval in msecs between frames
// start_frame - uint16_t, start frame number
// number_of_packets - uint16_t, number of packets to transfer
// nr_frame_desc_segs - uint16_t number of seg[] frame descriptors elements
//
// Request type specific data for interrupt request:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |            interval             |                0                | 4
// +----------------+----------------+----------------+----------------+
// |                                 0                                 | 8
// +----------------+----------------+----------------+----------------+
// interval - uint16_t, time in msecs until interruption
//
// Request type specific data for control request:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |                      data of setup packet                         | 4
// |                                                                   | 8
// +----------------+----------------+----------------+----------------+
//
// Request type specific data for bulk request:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |                                 0                                 | 4
// |                                 0                                 | 8
// +----------------+----------------+----------------+----------------+
//
// Request type specific data for unlink request:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |           unlink_id             |                0                | 4
// +----------------+----------------+----------------+----------------+
// |                                 0                                 | 8
// +----------------+----------------+----------------+----------------+
// unlink_id - uint16_t, request id of request to terminate
//
// seg[] array element layout:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |                               gref                                | 4
// +----------------+----------------+----------------+----------------+
// |             offset              |             length              | 8
// +----------------+----------------+----------------+----------------+
// gref - uint32_t, grant reference of buffer page
// offset - uint16_t, offset of buffer start in page
// length - uint16_t, length of buffer in page
//
// -------------------------- USB I/O response --------------------------------
//
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |          start_frame            | 4
// +----------------+----------------+----------------+----------------+
// |                              status                               | 8
// +----------------+----------------+----------------+----------------+
// |                          actual_length                            | 12
// +----------------+----------------+----------------+----------------+
// |                           error_count                             | 16
// +----------------+----------------+----------------+----------------+
// id - uint16_t, id of the request this response belongs to
// start_frame - uint16_t, start_frame this response (iso requests only)
// status - int32_t, XENUSB_STATUS_* (non-iso requests)
// actual_length - uint32_t, actual size of data transferred
// error_count - uint32_t, number of errors (iso requests)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xenusb_spec_version {
    XENUSB_VER_UNKNOWN = 0,
    XENUSB_VER_USB11,
    XENUSB_VER_USB20,
    XENUSB_VER_USB30,	/* not supported yet */
}

//
// USB pipe in xenusb_request
//
// - port number:      bits 0-4
// (USB_MAXCHILDREN is 31)
//
// - operation flag:   bit 5
// (0 = submit urb,
// 1 = unlink urb)
//
// - direction:        bit 7
// (0 = Host-to-Device [Out]
// 1 = Device-to-Host [In])
//
// - device address:   bits 8-14
//
// - endpoint:         bits 15-18
//
// - pipe type:        bits 30-31
// (00 = isochronous, 01 = interrupt,
// 10 = control, 11 = bulk)
//
pub const XENUSB_PIPE_PORT_MASK: c_uint = 0x0000001f;
pub const XENUSB_PIPE_UNLINK: c_uint = 0x00000020;
pub const XENUSB_PIPE_DIR: c_uint = 0x00000080;
pub const XENUSB_PIPE_DEV_MASK: c_uint = 0x0000007f;
pub const XENUSB_PIPE_DEV_SHIFT: c_int = 8;
pub const XENUSB_PIPE_EP_MASK: c_uint = 0x0000000f;
pub const XENUSB_PIPE_EP_SHIFT: c_int = 15;
pub const XENUSB_PIPE_TYPE_MASK: c_uint = 0x00000003;
pub const XENUSB_PIPE_TYPE_SHIFT: c_int = 30;
pub const XENUSB_PIPE_TYPE_ISOC: c_int = 0;
pub const XENUSB_PIPE_TYPE_INT: c_int = 1;
pub const XENUSB_PIPE_TYPE_CTRL: c_int = 2;
pub const XENUSB_PIPE_TYPE_BULK: c_int = 3;

pub const XENUSB_MAX_PORTNR: c_int = 31;
pub const XENUSB_RING_SIZE: c_int = 4096;
//
// RING for transferring urbs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_request_segment {
    pub gref: grant_ref_t,
    pub offset: u16,
    pub length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_urb_request {
    pub /: *mut *mut uint16_t id; / request id,
    pub /: *mut *mut uint16_t nr_buffer_segs; / number of urb->transfer_buffer segments,
// basic urb parameter
    pub pipe: u32,
    pub transfer_flags: u16,
pub const XENUSB_SHORT_NOT_OK: c_uint = 0x0001;
    pub buffer_length: u16,
    pub /: *mut *mut uint8_t ctrl[8]; / setup_packet (Ctrl),
    pub /: *mut *mut *mut uint16_t interval; / maximum (10248) in usb core,
    pub /: *mut *mut uint16_t start_frame; / start frame,
    pub /: *mut *mut uint16_t number_of_packets; / number of ISO packet,
    pub /: *mut *mut uint16_t nr_frame_desc_segs; / number of iso_frame_desc segments,
    pub isoc: },
    pub /: *mut *mut *mut uint16_t interval; / maximum (10248) in usb core,
    pub pad: [u16; 3],
    pub intr: },
    pub /: *mut *mut uint16_t unlink_id; / unlink request id,
    pub pad: [u16; 3],
    pub unlink: },
    pub u: },
// urb data segments
    pub seg: [xenusb_request_segment; XENUSB_MAX_SEGMENTS_PER_REQUEST],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_urb_response {
    pub /: *mut *mut uint16_t id; / request id,
    pub /: *mut *mut uint16_t start_frame; / start frame (ISO),
    pub /: *mut *mut int32_t status; / status (non-ISO),
pub const XENUSB_STATUS_OK: c_int = 0;

    pub /: *mut *mut int32_t actual_length; / actual transfer length,
    pub /: *mut *mut int32_t error_count; / number of ISO errors,
}

//
// RING for notifying connect/disconnect events to frontend
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_conn_request {
    pub id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenusb_conn_response {
    pub /: *mut *mut uint16_t id; / request id,
    pub /: *mut *mut uint8_t portnum; / port number,
    pub /: *mut *mut uint8_t speed; / usb_device_speed,
pub const XENUSB_SPEED_NONE: c_int = 0;
pub const XENUSB_SPEED_LOW: c_int = 1;
pub const XENUSB_SPEED_FULL: c_int = 2;
pub const XENUSB_SPEED_HIGH: c_int = 3;
}

