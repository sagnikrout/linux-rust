//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/xprtsock.h
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
// linux/include/linux/sunrpc/xprtsock.h
//
// Declarations for the RPC transport socket provider.
//
extern "C" {
    pub fn init_socket_xprt() -> c_int;
}
extern "C" {
    pub fn cleanup_socket_xprt();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_xprt {
    pub xprt: rpc_xprt,
//
// Network layer
//
    pub sock: *mut *mut socket,
    pub inet: *mut *mut sock,
    pub file: *mut *mut file,
//
// State of TCP reply receive
//
    pub __attribute__((packed)): },
    pub copied: c_ulong,
    pub recv: },
//
// State of TCP transmit queue
//
    pub offset: u32,
    pub xmit: },
//
// Connection of transports
//
    pub sock_state: c_ulong,
    pub connect_worker: delayed_work,
    pub error_worker: work_struct,
    pub recv_worker: work_struct,
    pub recv_mutex: mutex,
    pub handshake_done: completion,
    pub srcaddr: sockaddr_storage,
    pub srcport: c_ushort,
    pub xprt_err: c_int,
    pub clnt: *mut rpc_clnt,
//
// UDP socket buffer size parameters
//
    pub tcp_timeout: rpc_timeout,
//
// Saved socket callback addresses
//
    pub ): *mut *mut void (old_data_ready)(struct sock,
    pub ): *mut *mut void (old_state_change)(struct sock,
    pub ): *mut *mut void (old_write_space)(struct sock,
    pub ): *mut *mut void (old_error_report)(struct sock,
}

//
// TCP RPC flags
//

