//! Automatically rewritten from C Header to Rust Module
//! Source: net/vmw_vsock/vmci_transport.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// VMware vSockets Driver
//
// Copyright (C) 2013 VMware, Inc. All rights reserved.
//

// If the packet format changes in a release then this should change too.
pub const VMCI_TRANSPORT_PACKET_VERSION: c_int = 1;
// The resource ID on which control packets are sent.
pub const VMCI_TRANSPORT_PACKET_RID: c_int = 1;
// The resource ID on which control packets are sent to the hypervisor.
pub const VMCI_TRANSPORT_HYPERVISOR_PACKET_RID: c_int = 15;
pub const VSOCK_PROTO_INVALID: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmci_transport_packet_type {
    VMCI_TRANSPORT_PACKET_TYPE_INVALID = 0,
    VMCI_TRANSPORT_PACKET_TYPE_REQUEST,
    VMCI_TRANSPORT_PACKET_TYPE_NEGOTIATE,
    VMCI_TRANSPORT_PACKET_TYPE_OFFER,
    VMCI_TRANSPORT_PACKET_TYPE_ATTACH,
    VMCI_TRANSPORT_PACKET_TYPE_WROTE,
    VMCI_TRANSPORT_PACKET_TYPE_READ,
    VMCI_TRANSPORT_PACKET_TYPE_RST,
    VMCI_TRANSPORT_PACKET_TYPE_SHUTDOWN,
    VMCI_TRANSPORT_PACKET_TYPE_WAITING_WRITE,
    VMCI_TRANSPORT_PACKET_TYPE_WAITING_READ,
    VMCI_TRANSPORT_PACKET_TYPE_REQUEST2,
    VMCI_TRANSPORT_PACKET_TYPE_NEGOTIATE2,
    VMCI_TRANSPORT_PACKET_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_transport_waiting_info {
    pub generation: u64,
    pub offset: u64,
}

// Control packet type for STREAM sockets.  DGRAMs have no control packets nor
// special packet header for data packets, they are just raw VMCI DGRAM
// messages.  For STREAMs, control packets are sent over the control channel
// while data is written and read directly from queue pairs with no packet
// format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_transport_packet {
    pub dg: vmci_datagram,
    pub version: u8,
    pub type: u8,
    pub proto: u16,
    pub src_port: u32,
    pub dst_port: u32,
    pub _reserved2: u32,
    pub size: u64,
    pub mode: u64,
    pub handle: vmci_handle,
    pub wait: vmci_transport_waiting_info,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_transport_notify_pkt {
    pub write_notify_window: u64,
    pub write_notify_min_window: u64,
    pub peer_waiting_read: bool,
    pub peer_waiting_write: bool,
    pub peer_waiting_write_detected: bool,
    pub sent_waiting_read: bool,
    pub sent_waiting_write: bool,
    pub peer_waiting_read_info: vmci_transport_waiting_info,
    pub peer_waiting_write_info: vmci_transport_waiting_info,
    pub produce_q_generation: u64,
    pub consume_q_generation: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_transport_notify_pkt_q_state {
    pub write_notify_window: u64,
    pub write_notify_min_window: u64,
    pub peer_waiting_write: bool,
    pub peer_waiting_write_detected: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union vmci_transport_notify {
    pub pkt: vmci_transport_notify_pkt,
    pub pkt_q_state: vmci_transport_notify_pkt_q_state,
}

// Our transport-specific data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_transport {
// For DGRAMs.
    pub dg_handle: vmci_handle,
// For STREAMs.
    pub qp_handle: vmci_handle,
    pub qpair: *mut vmci_qp,
    pub produce_size: u64,
    pub consume_size: u64,
    pub detach_sub_id: u32,
    pub notify: vmci_transport_notify,
    pub notify_ops: *const vmci_transport_notify_ops,
    pub elem: list_head,
    pub sk: *mut sock,
    pub /: *mut *mut spinlock_t lock; / protects sk.,
}

extern "C" {
    pub fn vmci_transport_send_wrote(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn vmci_transport_send_read(sk: *mut sock) -> c_int;
}
