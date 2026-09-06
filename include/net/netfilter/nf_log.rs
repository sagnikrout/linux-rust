//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_log.h
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

// Log tcp sequence, tcp options, ip options and uid owning local socket
pub const NF_LOG_DEFAULT_MASK: c_uint = 0x0f;
// This flag indicates that copy_len field in nf_loginfo is set
pub const NF_LOG_F_COPY_LEN: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_log_type {
    NF_LOG_TYPE_LOG		= 0,
    NF_LOG_TYPE_ULOG,
    NF_LOG_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_loginfo {
    pub type: u_int8_t,
// copy_len will be used iff you set
// NF_LOG_F_COPY_LEN in flags
//
    pub copy_len: u_int32_t,
    pub group: u_int16_t,
    pub qthreshold: u_int16_t,
    pub flags: u_int16_t,
    pub ulog: },
    pub level: u_int8_t,
    pub logflags: u_int8_t,
    pub log: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_logger {
    pub name: *mut c_char,
    pub type: nf_log_type,
    pub logfn: *mut nf_logfn,
    pub me: *mut module,
}

// sysctl_nf_log_all_netns - allow LOG target in all network namespaces
// Function to register/unregister log function.
extern "C" {
    pub fn nf_log_register(pf: u_int8_t, logger: *mut nf_logger) -> c_int;
}
extern "C" {
    pub fn nf_log_unregister(logger: *mut nf_logger);
}
// Check if any logger is registered for a given protocol family.
extern "C" {
    pub fn nf_log_is_registered(pf: u_int8_t) -> bool;
}
extern "C" {
    pub fn nf_log_set(net: *mut net, pf: u_int8_t, logger: *const nf_logger) -> c_int;
}
extern "C" {
    pub fn nf_log_unset(net: *mut net, logger: *const nf_logger);
}
extern "C" {
    pub fn nf_log_unbind_pf(net: *mut net, pf: u_int8_t);
}
extern "C" {
    pub fn nf_logger_find_get(pf: c_int, type: nf_log_type) -> c_int;
}
extern "C" {
    pub fn nf_logger_put(pf: c_int, type: nf_log_type);
}

// Calls the registered backend logging function
extern "C" {
    pub fn nf_log_buf_close(m: *mut nf_log_buf);
}
