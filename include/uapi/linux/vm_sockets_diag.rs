//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vm_sockets_diag.h
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
// AF_VSOCK sock_diag(7) interface for querying open sockets

// Request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsock_diag_req {
    pub /: *mut *mut __u8 sdiag_family; / must be AF_VSOCK,
    pub /: *mut *mut __u8 sdiag_protocol; / must be 0,
    pub /: *mut *mut __u16 pad; / must be 0,
    pub /: *mut *mut __u32 vdiag_states; / query bitmap (e.g. 1 << TCP_LISTEN),
    pub /: *mut *mut __u32 vdiag_ino; / must be 0 (reserved),
    pub /: *mut *mut __u32 vdiag_show; / must be 0 (reserved),
    pub vdiag_cookie: [__u32; 2],
}

// Response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsock_diag_msg {
    pub /: *mut *mut __u8 vdiag_family; / AF_VSOCK,
    pub /: *mut *mut __u8 vdiag_type; / SOCK_STREAM or SOCK_DGRAM,
    pub /: *mut *mut __u8 vdiag_state; / sk_state (e.g. TCP_LISTEN),
    pub /: *mut *mut __u8 vdiag_shutdown; / local RCV_SHUTDOWN | SEND_SHUTDOWN,
    pub vdiag_src_cid: __u32,
    pub vdiag_src_port: __u32,
    pub vdiag_dst_cid: __u32,
    pub vdiag_dst_port: __u32,
    pub vdiag_ino: __u32,
    pub vdiag_cookie: [__u32; 2],
}
