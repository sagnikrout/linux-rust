//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/smbdirect.h
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
// Copyright (C) 2017, Microsoft Corporation.
//
// Author(s): Long Li <longli@microsoft.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbd_connection {
    pub socket: *mut smbdirect_socket,
}

// Create a SMBDirect session
// Reconnect SMBDirect session
extern "C" {
    pub fn smbd_reconnect(server: *mut TCP_Server_Info) -> c_int;
}
// Destroy SMBDirect session
extern "C" {
    pub fn smbd_destroy(server: *mut TCP_Server_Info);
}
// Interface for carrying upper layer I/O through send/recv
extern "C" {
    pub fn smbd_recv(info: *mut smbd_connection, msg: *mut msghdr) -> c_int;
}
// Interfaces to register and deregister MR for RDMA read/write
extern "C" {
    pub fn smbd_deregister_mr(mr: *mut smbdirect_mr_io);
}
extern "C" {
    pub fn smbd_debug_proc_show(server: *mut TCP_Server_Info, m: *mut seq_file);
}

pub const cifs_rdma_enabled(server): c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbd_connection {
    pub NULL;}: *mut *mut *mut TCP_Server_Info server, sockaddr dstaddr) {return,
    pub }: *mut *mut static inline int smbd_reconnect(struct TCP_Server_Info server) {return -1;,
    pub }: *mut *mut *mut static inline int smbd_recv(struct smbd_connection info, struct msghdr msg) {return -1;,
    pub }: *mut *mut *mut static inline int smbd_send(struct TCP_Server_Info server, int num_rqst, struct smb_rqst rqst) {return -1;,

