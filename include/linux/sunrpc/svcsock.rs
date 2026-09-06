//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/svcsock.h
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
// linux/include/linux/sunrpc/svcsock.h
//
// RPC server socket I/O.
//
// Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
//

//
// RPC server socket.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_sock {
    pub sk_xprt: svc_xprt,
    pub /: *mut *mut *mut socket  sk_sock; / berkeley socket layer,
    pub /: *mut *mut *mut sock  sk_sk; / INET layer,
// We keep the old state_change and data_ready CB's here
    pub ): *mut *mut void (sk_ostate)(struct sock,
    pub ): *mut *mut void (sk_odata)(struct sock,
    pub ): *mut *mut void (sk_owspace)(struct sock,
// For sends (protected by xpt_mutex)
    pub sk_bvec: *mut bio_vec,
// private TCP part
// On-the-wire fragment header:
    pub sk_marker: __be32,
// As we receive a record, this includes the length received so
// far (including the fragment header):
    pub sk_tcplen: u32,
// Total length of the data (not including fragment headers)
// received so far in the fragments making up this rpc:
    pub sk_datalen: u32,
    pub sk_frag_cache: page_frag_cache,
    pub sk_handshake_done: completion,
// received data
    pub sk_maxpages: c_ulong,
    pub __counted_by(sk_maxpages): *mut *mut page  sk_pages[],
}

//
// Function prototypes.
//
extern "C" {
    pub fn svc_recv(rqstp: *mut svc_rqst, timeo: c_long) -> c_int;
}
extern "C" {
    pub fn svc_send(rqstp: *mut svc_rqst);
}
extern "C" {
    pub fn svc_init_xprt_sock();
}
extern "C" {
    pub fn svc_cleanup_xprt_sock();
}
//
// svc_makesock socket characteristics
//

