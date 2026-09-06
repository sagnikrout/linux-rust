//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/af_unix.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unix_address {
    pub refcnt: refcount_t,
    pub len: c_int,
    pub name: [sockaddr_un; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_stat {
    pub nr_fds: core::sync::atomic::AtomicI32,
    pub nr_unix_fds: c_ulong,
}

// The AF_UNIX socket
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unix_sock {
// WARNING: sk has to be the first member
    pub sk: sock,
    pub addr: *mut unix_address,
    pub path: path,
    pub bindlock: mutex iolock,,
    pub peer: *mut sock,
    pub listener: *mut sock,
    pub vertex: *mut unix_vertex,
    pub lock: spinlock_t,
    pub peer_wq: socket_wq,

    pub peer_wake: wait_queue_entry_t,
    pub scm_stat: scm_stat,
    pub inq_len: c_int,
    pub recvmsg_inq: bool,
    pub scm_rights_notrunc: bool,

    pub oob_skb: *mut sk_buff,

}

