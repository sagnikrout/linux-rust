//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/scm.h
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

// Well, we should have at least one descriptor open
// to accept passed FDs 8)
//
pub const SCM_MAX_FD: c_int = 253;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_creds {
    pub pid: u32,
    pub uid: kuid_t,
    pub gid: kgid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_fp_list {
    pub count: c_short,
    pub count_unix: c_short,
    pub max: c_short,

    pub inflight: bool,
    pub dead: bool,
    pub vertices: list_head,
    pub edges: *mut unix_edge,

    pub user: *mut user_struct,
    pub fp: [*mut file; SCM_MAX_FD],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_cookie {
    pub /: *mut *mut *mut pid pid; / Skb credentials,
    pub /: *mut *mut *mut scm_fp_list fp; / Passed files,
    pub /: *mut *mut scm_creds creds; / Skb credentials,

    pub /: *mut *mut u32 secid; / Passed security ID,

}

extern "C" {
    pub fn scm_detach_fds(msg: *mut msghdr, scm: *mut scm_cookie, notrunc: bool);
}
extern "C" {
    pub fn scm_detach_fds_compat(msg: *mut msghdr, scm: *mut scm_cookie, notrunc: bool);
}
extern "C" {
    pub fn __scm_send(sock: *mut socket, msg: *mut msghdr, scm: *mut scm_cookie) -> c_int;
}
extern "C" {
    pub fn __scm_destroy(scm: *mut scm_cookie);
}

extern "C" {
    pub fn __scm_send(_arg: sock, _arg: msg, _arg: scm) -> return;
}
