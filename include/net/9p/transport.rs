//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/9p/transport.h
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
// Transport Definition
//
// Copyright (C) 2005 by Latchesar Ionkov <lucho@ionkov.net>
// Copyright (C) 2004-2008 by Eric Van Hensbergen <ericvh@gmail.com>
//

pub const P9_FD_PORT: c_int = 564;
pub const P9_RDMA_PORT: c_int = 5640;
pub const P9_RDMA_SQ_DEPTH: c_int = 32;
pub const P9_RDMA_RQ_DEPTH: c_int = 32;

//
// struct p9_trans_module - transport module interface
// @list: used to maintain a list of currently available transports
// @name: the human-readable name of the transport
// @maxsize: transport provided maximum packet size
// @pooled_rbuffers: currently only set for RDMA transport which pulls the
// response buffers from a shared pool, and accordingly
// we're less flexible when choosing the response message
// size in this case
// @def: set if this transport should be considered the default
// @supports_vmalloc: set if this transport can work with vmalloc'd buffers
// (non-physically contiguous memory). Transports requiring
// DMA should leave this as false.
// @create: member function to create a new connection on this transport
// @close: member function to discard a connection on this transport
// @request: member function to issue a request to the transport
// @cancel: member function to cancel a request (if it hasn't been sent)
// @cancelled: member function to notify that a cancelled request will not
// receive a reply
//
// This is the basic API for a transport module which is registered by the
// transport module with the 9P core network module and used by the client
// to instantiate a new connection on a transport.
//
// The transport module list is protected by v9fs_trans_lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p9_trans_module {
    pub list: list_head,
    pub /: *mut *mut *mut char name; / name of transport,
    pub /: *mut *mut int maxsize; / max message size of transport,
    pub pooled_rbuffers: bool,
    pub /: *mut *mut bool def; / this transport should be default,
    pub /: *mut *mut bool supports_vmalloc; / can work with vmalloc'd buffers,
    pub owner: *mut module,
    pub fc): *mut fs_context,
    pub client): *mut *mut void (close)(struct p9_client,
    pub req): *mut *mut *mut int (request)(struct p9_client client, struct p9_req_t,
    pub req): *mut *mut *mut int (cancel)(struct p9_client client, struct p9_req_t,
    pub req): *mut *mut *mut int (cancelled)(struct p9_client client, struct p9_req_t,
    pub in_hdr_len): int inlen, int outlen, int,
    pub client): *mut *mut *mut int (show_options)(struct seq_file m, struct p9_client,
}

extern "C" {
    pub fn v9fs_register_trans(m: *mut p9_trans_module);
}
extern "C" {
    pub fn v9fs_unregister_trans(m: *mut p9_trans_module);
}
extern "C" {
    pub fn v9fs_put_trans(m: *mut p9_trans_module);
}

