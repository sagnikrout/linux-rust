//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/bc_xprt.h
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
// Functions to create and manage the backchannel
//

extern "C" {
    pub fn xprt_complete_bc_request(req: *mut rpc_rqst, copied: u32);
}
extern "C" {
    pub fn xprt_free_bc_request(req: *mut rpc_rqst);
}
extern "C" {
    pub fn xprt_setup_backchannel(: *mut rpc_xprt, min_reqs: c_uint) -> c_int;
}
extern "C" {
    pub fn xprt_destroy_backchannel(: *mut rpc_xprt, max_reqs: c_uint);
}
extern "C" {
    pub fn xprt_enqueue_bc_request(req: *mut rpc_rqst);
}
// Socket backchannel transport methods
extern "C" {
    pub fn xprt_setup_bc(xprt: *mut rpc_xprt, min_reqs: c_uint) -> c_int;
}
extern "C" {
    pub fn xprt_destroy_bc(xprt: *mut rpc_xprt, max_reqs: c_uint);
}
extern "C" {
    pub fn xprt_free_bc_rqst(req: *mut rpc_rqst);
}
extern "C" {
    pub fn xprt_bc_max_slots(xprt: *mut rpc_xprt) -> c_uint;
}
extern "C" {
    pub fn xprt_svc_shutdown_bc(xprt: *mut rpc_xprt);
}
extern "C" {
    pub fn xprt_svc_destroy_nullify_bc(xprt: *mut rpc_xprt, serv: *mut svc_serv);
}
//
// Determine if a shared backchannel is in use
//

