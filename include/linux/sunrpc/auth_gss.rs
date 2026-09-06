//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/auth_gss.h
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
// linux/include/linux/sunrpc/auth_gss.h
//
// Declarations for RPCSEC_GSS
//
// Dug Song <dugsong@monkey.org>
// Andy Adamson <andros@umich.edu>
// Bruce Fields <bfields@umich.edu>
// Copyright (c) 2000 The Regents of the University of Michigan
//

pub const RPC_GSS_VERSION: c_int = 1;
pub const MAXSEQ: c_uint = 0x80000000 /* maximum legal sequence number, from rfc 2203 */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_gss_proc {
    RPC_GSS_PROC_DATA = 0,
    RPC_GSS_PROC_INIT = 1,
    RPC_GSS_PROC_CONTINUE_INIT = 2,
    RPC_GSS_PROC_DESTROY = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_gss_svc {
    RPC_GSS_SVC_NONE = 1,
    RPC_GSS_SVC_INTEGRITY = 2,
    RPC_GSS_SVC_PRIVACY = 3
}

// on-the-wire gss cred:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_gss_wire_cred {
    pub /: *mut *mut u32 gc_v; / version,
    pub /: *mut *mut u32 gc_proc; / control procedure,
    pub /: *mut *mut u32 gc_seq; / sequence number,
    pub /: *mut *mut u32 gc_svc; / service,
    pub /: *mut *mut xdr_netobj gc_ctx; / context handle,
}

// on-the-wire gss verifier:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_gss_wire_verf {
    pub gv_flavor: u32,
    pub gv_verf: xdr_netobj,
}

// return from gss NULL PROC init sec context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_gss_init_res {
    pub /: *mut *mut xdr_netobj gr_ctx; / context handle,
    pub /: *mut *mut u32 gr_major; / major status,
    pub /: *mut *mut u32 gr_minor; / minor status,
    pub /: *mut *mut u32 gr_win; / sequence window,
    pub /: *mut *mut xdr_netobj gr_token; / token,
}

// The gss_cl_ctx struct holds all the information the rpcsec_gss client
// code needs to know about a single security context.  In particular,
// gc_gss_ctx is the context handle that is used to do gss-api calls, while
// gc_wire_ctx is the context handle that is used to identify the context on
// the wire when communicating with a server.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_cl_ctx {
    pub count: refcount_t,
    pub gc_proc: rpc_gss_proc,
    pub gc_seq: u32,
    pub gc_seq_xmit: u32,
    pub gc_seq_lock: spinlock_t,
    pub gc_gss_ctx: *mut gss_ctx,
    pub gc_wire_ctx: xdr_netobj,
    pub gc_acceptor: xdr_netobj,
    pub gc_win: u32,
    pub gc_expiry: c_ulong,
    pub gc_rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_cred {
    pub gc_base: rpc_cred,
    pub gc_service: rpc_gss_svc,
    pub gc_ctx: *mut gss_cl_ctx __rcu,
    pub gc_upcall: *mut gss_upcall_msg,
    pub gc_principal: *const c_char,
    pub gc_upcall_timestamp: c_ulong,
}
