//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/inet_diag.h
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
pub const _INET_DIAG_H_: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_handler {
    pub owner: *mut module,
    pub r): *const inet_diag_req_v2,
    pub req): *const inet_diag_req_v2,
    pub info): *mut c_void,
    pub skb): *mut sk_buff,
    pub req): *const inet_diag_req_v2,
    pub idiag_type: __u16,
    pub idiag_info_size: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_dump_data {
    pub req_nlas: [*mut nlattr; __INET_DIAG_REQ_MAX],
    pub bpf_stg_diag: *mut bpf_sk_storage_diag,
    pub /: *mut *mut bool mark_needed; / INET_DIAG_BC_MARK_COND present.,

    pub /: *mut *mut bool cgroup_needed; / INET_DIAG_BC_CGROUP_COND present.,

    pub /: *mut *mut bool userlocks_needed; / INET_DIAG_BC_AUTO present.,
}

extern "C" {
    pub fn inet_diag_bc_sk(cb_data: *const inet_diag_dump_data, sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn inet_diag_msg_common_fill(r: *mut inet_diag_msg, sk: *mut sock);
}

// INET_DIAG_SOCKOPT
extern "C" {
    pub fn inet_diag_register(handler: *const inet_diag_handler) -> c_int;
}
extern "C" {
    pub fn inet_diag_unregister(handler: *const inet_diag_handler);
}
