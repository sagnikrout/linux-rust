//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vsockmon.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// vsockmon is the AF_VSOCK packet capture device.  Packets captured have the
// following layout:
//
// +-----------------------------------+
// |           vsockmon header         |
// |      (struct af_vsockmon_hdr)     |
// +-----------------------------------+
// |          transport header         |
// | (af_vsockmon_hdr->len bytes long) |
// +-----------------------------------+
// |              payload              |
// |       (until end of packet)       |
// +-----------------------------------+
//
// The vsockmon header is a transport-independent description of the packet.
// It duplicates some of the information from the transport header so that
// no transport-specific knowledge is necessary to process packets.
//
// The transport header is useful for low-level transport-specific packet
// analysis.  Transport type is given in af_vsockmon_hdr->transport and
// transport header length is given in af_vsockmon_hdr->len.
//
// If af_vsockmon_hdr->op is AF_VSOCK_OP_PAYLOAD then the payload follows the
// transport header.  Other ops do not have a payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct af_vsockmon_hdr {
    pub src_cid: __le64,
    pub dst_cid: __le64,
    pub src_port: __le32,
    pub dst_port: __le32,
    pub /: *mut *mut __le16 op; / enum af_vsockmon_op,
    pub /: *mut *mut __le16 transport; / enum af_vsockmon_transport,
    pub /: *mut *mut __le16 len; / Transport header length,
    pub reserved: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum af_vsockmon_op {
    AF_VSOCK_OP_UNKNOWN = 0,
    AF_VSOCK_OP_CONNECT = 1,
    AF_VSOCK_OP_DISCONNECT = 2,
    AF_VSOCK_OP_CONTROL = 3,
    AF_VSOCK_OP_PAYLOAD = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum af_vsockmon_transport {
    AF_VSOCK_TRANSPORT_UNKNOWN = 0,
    AF_VSOCK_TRANSPORT_NO_INFO = 1,	/* No transport information */

// Transport header type: struct virtio_vsock_hdr
    AF_VSOCK_TRANSPORT_VIRTIO = 2,
}
