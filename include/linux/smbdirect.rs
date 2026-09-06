//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/smbdirect.h
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
//
// Copyright (C) 2025, Stefan Metzmacher
//

// SMB-DIRECT buffer descriptor V1 structure [MS-SMBD] 2.2.3.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_buffer_descriptor_v1 {
    pub offset: __le64,
    pub token: __le32,
    pub length: __le32,
    pub __packed: },
//
// Connection parameters mostly from [MS-SMBD] 3.1.1.1
//
// These are setup and negotiated at the beginning of a
// connection and remain constant unless explicitly changed.
//
// Some values are important for the upper layer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_socket_parameters {
    pub flags: __u64,

    pub resolve_addr_timeout_msec: __u32,
    pub resolve_route_timeout_msec: __u32,
    pub rdma_connect_timeout_msec: __u32,
    pub negotiate_timeout_msec: __u32,
    pub /: *mut *mut __u16 initiator_depth; / limited to U8_MAX,
    pub /: *mut *mut __u16 responder_resources; / limited to U8_MAX,
    pub recv_credit_max: __u16,
    pub send_credit_target: __u16,
    pub max_send_size: __u32,
    pub max_fragmented_send_size: __u32,
    pub max_recv_size: __u32,
    pub max_fragmented_recv_size: __u32,
    pub max_read_write_size: __u32,
    pub max_frmr_depth: __u32,
    pub keepalive_interval_msec: __u32,
    pub keepalive_timeout_msec: __u32,
    pub __packed: },

    pub smbdirect_socket: struct,
    pub smbdirect_send_batch: struct,
    pub smbdirect_mr_io: struct,

    pub netdev): *mut u8 smbdirect_netdev_rdma_capable_node_type(struct net_device,
    pub attrs): *const bool smbdirect_frwr_is_supported(struct ib_device_attr,
    pub _sc): *mut *mut int smbdirect_socket_create_kern(struct net net, struct smbdirect_socket,
    pub _sc): *mut *mut int smbdirect_socket_create_accepting(struct rdma_cm_id id, struct smbdirect_socket,
    pub sp): *const smbdirect_socket_parameters,
    pub sc): *mut smbdirect_socket_get_current_parameters(struct smbdirect_socket,
    pub gfp_mask): gfp_t,
pub const SMBDIRECT_LOG_ERR: c_uint = 0x0;
pub const SMBDIRECT_LOG_INFO: c_uint = 0x1;
pub const SMBDIRECT_LOG_OUTGOING: c_uint = 0x1;
pub const SMBDIRECT_LOG_INCOMING: c_uint = 0x2;
pub const SMBDIRECT_LOG_READ: c_uint = 0x4;
pub const SMBDIRECT_LOG_WRITE: c_uint = 0x8;
pub const SMBDIRECT_LOG_RDMA_SEND: c_uint = 0x10;
pub const SMBDIRECT_LOG_RDMA_RECV: c_uint = 0x20;
pub const SMBDIRECT_LOG_KEEP_ALIVE: c_uint = 0x40;
pub const SMBDIRECT_LOG_RDMA_EVENT: c_uint = 0x80;
pub const SMBDIRECT_LOG_RDMA_MR: c_uint = 0x100;
pub const SMBDIRECT_LOG_RDMA_RW: c_uint = 0x200;
pub const SMBDIRECT_LOG_NEGOTIATE: c_uint = 0x400;
    pub vaf)): *mut va_format,
    pub sc): *mut bool smbdirect_connection_is_connected(struct smbdirect_socket,
    pub sc): *mut int smbdirect_connection_wait_for_connected(struct smbdirect_socket,
    pub addr): *mut *mut int smbdirect_socket_bind(struct smbdirect_socket sc, struct sockaddr,
    pub sc): *mut void smbdirect_socket_shutdown(struct smbdirect_socket,
    pub sc): *mut void smbdirect_socket_release(struct smbdirect_socket,
    pub is_last): bool,
//
// This is only temporary and only needed
// as long as the client still requires
// to use smbdirect_connection_send_single_iter()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_send_batch_storage {
    pub __msg_list: list_head,
    pub __space: [__aligned_u64; 5],
}

extern "C" {
    pub fn smbdirect_connection_send_wait_zero_pending(sc: *mut smbdirect_socket) -> c_int;
}
extern "C" {
    pub fn smbdirect_socket_listen(sc: *mut smbdirect_socket, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn smbdirect_connection_deregister_mr_io(mr: *mut smbdirect_mr_io);
}
